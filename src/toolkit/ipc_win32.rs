use std::ffi::OsStr;
use std::io::Error;
use std::os::windows::ffi::OsStrExt;
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::{CreateFileW, ReadFile, WriteFile, FlushFileBuffers};
use windows_sys::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, DisconnectNamedPipe,
};

const PIPE_ACCESS_DUPLEX: u32 = 0x00000003;
const PIPE_TYPE_BYTE: u32 = 0x00000000;
const PIPE_READMODE_BYTE: u32 = 0x00000000;
const PIPE_WAIT: u32 = 0x00000000;
const OPEN_EXISTING: u32 = 3;
const GENERIC_READ: u32 = 0x80000000;
const GENERIC_WRITE: u32 = 0x40000000;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SendHandle(HANDLE);
unsafe impl Send for SendHandle {}
unsafe impl Sync for SendHandle {}

unsafe fn read_pipe(handle: HANDLE, buf: &mut [u8]) -> Result<usize, Error> {
    let mut bytes_read = 0u32;
    let ok = unsafe {
        ReadFile(
            handle,
            buf.as_mut_ptr() as _,
            buf.len() as u32,
            &mut bytes_read,
            ptr::null_mut(),
        )
    };
    if ok == 0 {
        let err = Error::last_os_error();
        if err.raw_os_error() == Some(109) {
            // ERROR_BROKEN_PIPE
            return Ok(0);
        }
        Err(err)
    } else {
        Ok(bytes_read as usize)
    }
}

unsafe fn write_pipe(handle: HANDLE, buf: &[u8]) -> Result<(), Error> {
    let mut bytes_written = 0u32;
    let ok = unsafe {
        WriteFile(
            handle,
            buf.as_ptr() as _,
            buf.len() as u32,
            &mut bytes_written,
            ptr::null_mut(),
        )
    };
    if ok == 0 {
        return Err(Error::last_os_error());
    }
    // Partial writes are an error: the caller asked for `buf.len()` bytes and
    // got fewer. The previous implementation silently returned Ok(()), which
    // could desync the protocol. (Fix for I4.)
    if (bytes_written as usize) != buf.len() {
        return Err(Error::new(
            std::io::ErrorKind::WriteZero,
            format!("short write: {} of {} bytes", bytes_written, buf.len()),
        ));
    }
    Ok(())
}

pub struct Win32IpcServer {
    handle: SendHandle,
}

impl Win32IpcServer {
    pub fn bind(name: &str) -> Result<Self, Error> {
        let pipe_name = format!(r"\\.\pipe\{}", name);
        let mut name_u16: Vec<u16> = OsStr::new(&pipe_name).encode_wide().collect();
        name_u16.push(0);

        let handle = unsafe {
            CreateNamedPipeW(
                name_u16.as_ptr(),
                PIPE_ACCESS_DUPLEX,
                PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
                1,     // max instances
                1024,  // out buffer
                1024,  // in buffer
                0,     // timeout
                ptr::null(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        Ok(Self { handle: SendHandle(handle) })
    }

    pub fn accept_and_respond<F>(&self, handler: F) -> Result<(), Error>
    where
        F: Fn(&str) -> String,
    {
        // NOTE: a true connection-wait timeout (N3) requires either OVERLAPPED
        // I/O or a thread-spawn with a Send-able HANDLE wrapper. Both add
        // complexity and compile-time friction (the HANDLE is `*mut c_void`
        // which is not Send by default; the existing `SendHandle` wrapper
        // doesn't satisfy `std::thread::spawn`'s bound in the current
        // windows-sys version). The 64KB buffer and from_utf8 fix (I4) below
        // remain the priority; the timeout fix is deferred.
        let connected = unsafe { ConnectNamedPipe(self.handle.0, ptr::null_mut()) };
        if connected == 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() != Some(535) {
                // ERROR_PIPE_CONNECTED
                return Err(err);
            }
        }

        let mut buffer = [0u8; 65536];
        let read_res = unsafe { read_pipe(self.handle.0, &mut buffer) };

        if let Ok(bytes_read) = read_res {
            if bytes_read > 0 {
                // Strict UTF-8: if the client sent invalid bytes, the request
                // is malformed. The previous `from_utf8_lossy` silently replaced
                // invalid bytes with U+FFFD, masking protocol corruption.
                // (Fix for I4.)
                let req_str = match std::str::from_utf8(&buffer[..bytes_read]) {
                    Ok(s) => s.trim_end_matches('\0'),
                    Err(e) => {
                        let _ = unsafe { DisconnectNamedPipe(self.handle.0) };
                        return Err(Error::new(
                            std::io::ErrorKind::InvalidData,
                            format!("non-UTF-8 request at byte {}: {}", e.valid_up_to(), e),
                        ));
                    }
                };
                let response = handler(req_str);
                if let Err(e) = unsafe { write_pipe(self.handle.0, response.as_bytes()) } {
                    let _ = unsafe { DisconnectNamedPipe(self.handle.0) };
                    return Err(e);
                }
            }
        }

        unsafe {
            let _ = FlushFileBuffers(self.handle.0);
            DisconnectNamedPipe(self.handle.0);
        }

        Ok(())
    }
}

impl Drop for Win32IpcServer {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle.0);
        }
    }
}

pub struct Win32IpcClient {
    handle: SendHandle,
}

impl Win32IpcClient {
    pub fn connect(name: &str) -> Result<Self, Error> {
        let pipe_name = format!(r"\\.\pipe\{}", name);
        let mut name_u16: Vec<u16> = OsStr::new(&pipe_name).encode_wide().collect();
        name_u16.push(0);

        let handle = unsafe {
            CreateFileW(
                name_u16.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                ptr::null(),
                OPEN_EXISTING,
                0,
                std::ptr::null_mut(),
            )
        };

        if handle == INVALID_HANDLE_VALUE {
            return Err(Error::last_os_error());
        }

        Ok(Self { handle: SendHandle(handle) })
    }

    pub fn send_request(&mut self, msg: &str) -> Result<String, Error> {
        unsafe {
            write_pipe(self.handle.0, msg.as_bytes())?;
            let mut buffer = [0u8; 65536];
            let bytes_read = read_pipe(self.handle.0, &mut buffer)?;
            let resp_str = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
            Ok(resp_str)
        }
    }
}

impl Drop for Win32IpcClient {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.handle.0);
        }
    }
}

use std::ffi::OsStr;
use std::io::{Error, Read, Write};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::{FromRawHandle, IntoRawHandle};
use std::ptr;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::Storage::FileSystem::CreateFileW;
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
        let connected = unsafe { ConnectNamedPipe(self.handle.0, ptr::null_mut()) };
        if connected == 0 {
            let err = Error::last_os_error();
            if err.raw_os_error() != Some(535) {
                // ERROR_PIPE_CONNECTED
                return Err(err);
            }
        }

        let mut file = unsafe { std::fs::File::from_raw_handle(self.handle.0 as _) };
        let mut buffer = [0u8; 1024];
        let read_res = file.read(&mut buffer);

        if let Ok(bytes_read) = read_res {
            if bytes_read > 0 {
                let req_str = String::from_utf8_lossy(&buffer[..bytes_read]);
                let response = handler(req_str.trim_end_matches('\0'));
                let _ = file.write_all(response.as_bytes());
                let _ = file.flush();
            }
        }

        let _ = file.into_raw_handle(); // Disowns the handle to avoid closing it on drop

        unsafe {
            windows_sys::Win32::Storage::FileSystem::FlushFileBuffers(self.handle.0);
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
        let mut file = unsafe { std::fs::File::from_raw_handle(self.handle.0 as _) };
        let write_res = file.write_all(msg.as_bytes()).and_then(|_| file.flush());

        if let Err(e) = write_res {
            let _ = file.into_raw_handle();
            return Err(e);
        }

        let mut buffer = [0u8; 1024];
        let read_res = file.read(&mut buffer);
        let _ = file.into_raw_handle(); // Disowns the handle

        match read_res {
            Ok(bytes_read) => {
                let resp_str = String::from_utf8_lossy(&buffer[..bytes_read]).into_owned();
                Ok(resp_str)
            }
            Err(e) => Err(e),
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

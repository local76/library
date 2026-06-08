//! Single-instance application guard helpers.
//!
//! **Taxonomy Classification**: Execution State (Lifecycle - Foreground) + Platform (Native).

#[cfg(target_os = "windows")]
type MutexHandle = windows_sys::Win32::Foundation::HANDLE;

#[derive(Debug)]
enum SingleInstanceHandle {
    #[cfg(target_os = "windows")]
    Windows(MutexHandle),
    #[cfg(target_os = "linux")]
    Unix(std::os::unix::net::UnixListener),
    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    None,
}

/// Ensures only one instance of the TUI application is active at any time.
pub struct SingleInstanceGuard {
    #[allow(dead_code)]
    handle: SingleInstanceHandle,
}

fn get_exe_name() -> String {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.file_name().map(|f| f.to_string_lossy().to_string()))
        .unwrap_or_else(|| "rCommonApp".to_string())
}

impl SingleInstanceGuard {
    pub fn try_new() -> crate::error::Result<Self> {
        #[cfg(target_os = "windows")]
        {
            let exe_name = get_exe_name();
            let mutex_name = format!("Local\\{}_SingleInstanceMutex", exe_name);
            let name: Vec<u16> = mutex_name.encode_utf16().chain(std::iter::once(0)).collect();
            let handle = unsafe {
                windows_sys::Win32::System::Threading::CreateMutexW(
                    std::ptr::null(),
                    1,
                    name.as_ptr(),
                )
            };
            if handle.is_null() {
                return Err(crate::error::RcommonError::Guard("Failed to create single-instance mutex.".to_string()));
            }

            let err = unsafe { windows_sys::Win32::Foundation::GetLastError() };
            if err == windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS {
                unsafe { windows_sys::Win32::Foundation::CloseHandle(handle) };
                return Err(crate::error::RcommonError::Guard("Another instance of this application is already running.".to_string()));
            }

            Ok(SingleInstanceGuard { handle: SingleInstanceHandle::Windows(handle) })
        }
        #[cfg(target_os = "linux")]
        {
            use std::os::unix::net::{UnixListener, UnixStream};
            let exe_name = get_exe_name();
            let socket_path = format!("/tmp/{}_single_instance.sock", exe_name);
            
            // Try to bind first
            let listener = match UnixListener::bind(&socket_path) {
                Ok(l) => l,
                Err(_) => {
                    // Bind failed, check if another instance is listening
                    if UnixStream::connect(&socket_path).is_ok() {
                        return Err(crate::error::RcommonError::Guard(
                            "Another instance of this application is already running.".to_string()
                        ));
                    }
                    // Socket is stale, try to remove and bind again
                    let _ = std::fs::remove_file(&socket_path);
                    UnixListener::bind(&socket_path).map_err(|_| {
                        crate::error::RcommonError::Guard(
                            "Another instance of this application is already running.".to_string()
                        )
                    })?
                }
            };

            Ok(SingleInstanceGuard { handle: SingleInstanceHandle::Unix(listener) })
        }
        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            Ok(SingleInstanceGuard { handle: SingleInstanceHandle::None })
        }
    }
}

impl Drop for SingleInstanceGuard {
    #[allow(irrefutable_let_patterns)]
    fn drop(&mut self) {
        #[cfg(target_os = "windows")]
        if let SingleInstanceHandle::Windows(h) = self.handle {
            if !h.is_null() {
                unsafe {
                    windows_sys::Win32::Foundation::CloseHandle(h);
                }
            }
        }
        #[cfg(target_os = "linux")]
        if let SingleInstanceHandle::Unix(_) = &self.handle {
            let exe_name = get_exe_name();
            let socket_path = format!("/tmp/{}_single_instance.sock", exe_name);
            let _ = std::fs::remove_file(socket_path);
        }
    }
}

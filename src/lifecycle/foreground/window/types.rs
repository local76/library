//! Common type definitions for window and console layouts.
//!
//! **Taxonomy Classification**: Execution State (Lifecycle - Foreground) + Platform (Native).

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct RECT {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct MONITORINFO {
    pub cbSize: u32,
    pub rcMonitor: RECT,
    pub rcWork: RECT,
    pub dwFlags: u32,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct COORD {
    pub x: i16,
    pub y: i16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct SMALL_RECT {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct CONSOLE_SELECTION_INFO {
    pub dwFlags: u32,
    pub dwSelectionAnchor: COORD,
    pub srSelection: SMALL_RECT,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(C)]
pub struct POINT {
    pub x: i32,
    pub y: i32,
}

#[cfg(target_os = "windows")]
pub(crate) unsafe fn get_console_hwnd() -> Option<*mut std::ffi::c_void> {
    let hwnd = unsafe { windows_sys::Win32::System::Console::GetConsoleWindow() };
    if hwnd.is_null() {
        None
    } else {
        Some(hwnd)
    }
}

#[cfg(target_os = "windows")]
pub(crate) unsafe fn get_console_window_rect(hwnd: *mut std::ffi::c_void) -> Option<RECT> {
    let mut rect = RECT::default();
    let ok = unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect(
            hwnd,
            &mut rect as *mut RECT as *mut windows_sys::Win32::Foundation::RECT,
        )
    };
    if ok != 0 {
        Some(rect)
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub(crate) unsafe fn get_monitor_work_rect(hwnd: *mut std::ffi::c_void) -> Option<RECT> {
    let h_monitor = unsafe {
        windows_sys::Win32::Graphics::Gdi::MonitorFromWindow(
            hwnd,
            windows_sys::Win32::Graphics::Gdi::MONITOR_DEFAULTTONEAREST,
        )
    };
    if h_monitor.is_null() {
        return None;
    }
    let mut mi = windows_sys::Win32::Graphics::Gdi::MONITORINFO {
        cbSize: std::mem::size_of::<windows_sys::Win32::Graphics::Gdi::MONITORINFO>() as u32,
        rcMonitor: windows_sys::Win32::Foundation::RECT { left: 0, top: 0, right: 0, bottom: 0 },
        rcWork: windows_sys::Win32::Foundation::RECT { left: 0, top: 0, right: 0, bottom: 0 },
        dwFlags: 0,
    };
    let ok = unsafe {
        windows_sys::Win32::Graphics::Gdi::GetMonitorInfoW(h_monitor, &mut mi as *mut _)
    };
    if ok != 0 {
        Some(RECT {
            left: mi.rcWork.left,
            top: mi.rcWork.top,
            right: mi.rcWork.right,
            bottom: mi.rcWork.bottom,
        })
    } else {
        None
    }
}

#[cfg(target_os = "windows")]
pub(crate) const SWP_FRAMECHANGED_NOOP: u32 =
    windows_sys::Win32::UI::WindowsAndMessaging::SWP_FRAMECHANGED
        | windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOZORDER
        | windows_sys::Win32::UI::WindowsAndMessaging::SWP_NOACTIVATE;

#[cfg(target_os = "windows")]
pub(crate) const GWL_STYLE: i32 = windows_sys::Win32::UI::WindowsAndMessaging::GWL_STYLE;

#[cfg(target_os = "windows")]
pub(crate) const WS_DECORATIONS: u32 = windows_sys::Win32::UI::WindowsAndMessaging::WS_CAPTION
    | windows_sys::Win32::UI::WindowsAndMessaging::WS_THICKFRAME
    | windows_sys::Win32::UI::WindowsAndMessaging::WS_MINIMIZEBOX
    | windows_sys::Win32::UI::WindowsAndMessaging::WS_MAXIMIZEBOX
    | windows_sys::Win32::UI::WindowsAndMessaging::WS_SYSMENU;

#[cfg(target_os = "windows")]
pub(crate) const SW_HIDE: i32 = windows_sys::Win32::UI::WindowsAndMessaging::SW_HIDE;

pub(crate) const BORDERLESS_DEFAULT_SIZE: f32 = 900.0;
pub(crate) const STABILIZE_ATTEMPTS: u32 = 20;
pub(crate) const STABILIZE_INTERVAL_MS: u64 = 10;

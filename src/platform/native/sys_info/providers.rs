//! Platform providers implementing PlatformProvider trait.
//!
//! **Taxonomy Classification**: Platform & Architecture (Deployment - Native) + Role (System Software).

use crate::platform::{PowerStatus, SystemBiosInfo, DiskDriveInfo, NetworkAdapterInfo, PlatformProvider};
#[cfg(target_os = "windows")]
use super::windows;

#[cfg(target_os = "linux")]
use super::linux;

#[cfg(all(
    not(any(target_os = "windows", target_os = "linux")),
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios")),
    not(any(target_os = "none", target_os = "uefi"))
))]
use super::fallback;

#[cfg(target_os = "windows")]
pub struct WindowsPlatform;

#[cfg(target_os = "windows")]
impl PlatformProvider for WindowsPlatform {
    fn get_system_screen_resolution() -> (i32, i32) { windows::get_system_screen_resolution() }
    fn get_console_window_dpi() -> u32 { windows::get_console_window_dpi() }
    fn query_accent_color() -> (u8, u8, u8) { windows::query_accent_color() }
    fn query_high_contrast() -> bool { windows::query_high_contrast() }
    fn query_os_version() -> String { windows::query_os_version() }
    fn query_dark_mode() -> bool { windows::query_dark_mode() }
    fn query_power_status() -> Option<PowerStatus> { windows::query_power_status() }
    fn query_bios_info() -> Option<SystemBiosInfo> { windows::query_bios_info() }
    fn query_shell_and_terminal() -> (String, String) { windows::query_shell_and_terminal() }
    fn query_disk_drives() -> Vec<DiskDriveInfo> { windows::query_disk_drives() }
    fn query_gpu_names() -> Vec<String> { windows::query_gpu_names() }
    fn query_network_adapters() -> Vec<NetworkAdapterInfo> { windows::query_network_adapters() }
    fn get_all_monitors() -> Vec<String> { crate::platform::native::monitors::get_all_monitors() }
}

#[cfg(target_os = "linux")]
pub struct LinuxPlatform;

#[cfg(target_os = "linux")]
impl PlatformProvider for LinuxPlatform {
    fn get_system_screen_resolution() -> (i32, i32) { linux::get_system_screen_resolution() }
    fn get_console_window_dpi() -> u32 { 96 }
    fn query_accent_color() -> (u8, u8, u8) { (0, 245, 255) }
    fn query_high_contrast() -> bool { false }
    fn query_os_version() -> String { linux::query_os_version() }
    fn query_dark_mode() -> bool { linux::query_dark_mode() }
    fn query_power_status() -> Option<PowerStatus> { linux::query_power_status() }
    fn query_bios_info() -> Option<SystemBiosInfo> { linux::query_bios_info() }
    fn query_shell_and_terminal() -> (String, String) { linux::query_shell_and_terminal() }
    fn query_disk_drives() -> Vec<DiskDriveInfo> { linux::query_disk_drives() }
    fn query_gpu_names() -> Vec<String> { linux::query_gpu_names() }
    fn query_network_adapters() -> Vec<NetworkAdapterInfo> { linux::query_network_adapters() }
    fn get_all_monitors() -> Vec<String> { crate::platform::native::monitors::get_all_monitors() }
}

#[cfg(all(
    not(any(target_os = "windows", target_os = "linux")),
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios")),
    not(any(target_os = "none", target_os = "uefi"))
))]
pub struct FallbackPlatform;

#[cfg(all(
    not(any(target_os = "windows", target_os = "linux")),
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios")),
    not(any(target_os = "none", target_os = "uefi"))
))]
impl PlatformProvider for FallbackPlatform {
    fn get_system_screen_resolution() -> (i32, i32) { fallback::get_system_screen_resolution() }
    fn get_console_window_dpi() -> u32 { fallback::get_console_window_dpi() }
    fn query_accent_color() -> (u8, u8, u8) { fallback::query_accent_color() }
    fn query_high_contrast() -> bool { false }
    fn query_os_version() -> String { fallback::query_os_version() }
    fn query_dark_mode() -> bool { fallback::query_dark_mode() }
    fn query_power_status() -> Option<PowerStatus> { fallback::query_power_status() }
    fn query_bios_info() -> Option<SystemBiosInfo> { fallback::query_bios_info() }
    fn query_shell_and_terminal() -> (String, String) { fallback::query_shell_and_terminal() }
    fn query_disk_drives() -> Vec<DiskDriveInfo> { fallback::query_disk_drives() }
    fn query_gpu_names() -> Vec<String> { fallback::query_gpu_names() }
    fn query_network_adapters() -> Vec<NetworkAdapterInfo> { fallback::query_network_adapters() }
    fn get_all_monitors() -> Vec<String> { crate::platform::native::monitors::get_all_monitors() }
}

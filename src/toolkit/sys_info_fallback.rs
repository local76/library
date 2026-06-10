//! Fallback stubs for native platform information queries when compiling without features.
//!
//! **Taxonomy Classification**: Platform & Architecture (Deployment - Native) + Role (System Software).
//!
//! **Feature Stub**: This is a fallback placeholder implementation providing safe, parameter-equivalent default values when the `sys-info` feature is disabled.

use crate::toolkit::platform::{PowerStatus, SystemBiosInfo, DiskDriveInfo, NetworkAdapterInfo};

pub fn get_system_screen_resolution() -> (i32, i32) { (1920, 1080) }
pub fn get_console_window_dpi() -> u32 { 96 }
pub fn query_accent_color() -> (u8, u8, u8) { (0, 120, 215) }
pub fn query_high_contrast() -> bool { false }
pub fn query_os_version() -> String { "Stub OS".to_string() }
pub fn query_dark_mode() -> bool { true }
pub fn query_power_status() -> Option<PowerStatus> { None }
pub fn query_bios_info() -> Option<SystemBiosInfo> { None }
pub fn query_shell_and_terminal() -> (String, String) { ("sh".to_string(), "xterm".to_string()) }
pub fn query_disk_drives() -> Vec<DiskDriveInfo> { vec![] }
pub fn query_gpu_names() -> Vec<String> { vec![] }
pub fn query_network_adapters() -> Vec<NetworkAdapterInfo> { vec![] }
#[cfg(feature = "widgets")]
pub fn get_dwm_accent_color() -> ratatui::style::Color { ratatui::style::Color::Cyan }

pub fn get_local_time_string() -> String { "2026-06-06 12:00:00".to_string() }
pub fn get_win_accent_color_hex() -> String { "#00F5FF".to_string() }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_sys_info() {
        assert_eq!(get_system_screen_resolution(), (1920, 1080));
        assert_eq!(get_console_window_dpi(), 96);
        assert_eq!(query_accent_color(), (0, 120, 215));
        assert!(!query_high_contrast());
        assert_eq!(query_os_version(), "Stub OS");
        assert!(query_dark_mode());
        assert!(query_power_status().is_none());
        assert!(query_bios_info().is_none());
        assert_eq!(query_shell_and_terminal(), ("sh".to_string(), "xterm".to_string()));
        assert!(query_disk_drives().is_empty());
        assert!(query_gpu_names().is_empty());
        assert!(query_network_adapters().is_empty());
    }
}


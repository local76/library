//! Platform & Architecture (Deployment)
//!
//! **Taxonomy Classification**: Platform & Architecture (Deployment).
//!
//! How the software is packaged and where it is designed to run.
//!
//! ### Platform & Feature Stubs
//!
//! To support clean cross-platform compilation and predictable fallback behavior,
//! this codebase follows a unified design for non-native platforms and disabled features:
//!
//! - **Platform Stub**: A fallback stub implementation providing safe, parameter-equivalent
//!   default values when compiled for target platforms where the native implementation is unavailable
//!   (e.g., Web, Mobile, Embedded).
//! - **Feature Stub**: A fallback placeholder implementation designed to compile successfully and
//!   preserve API parity when a specific feature flag (such as `sys-info` or `gui`) is disabled.
//!
//! Categories:
//! - Native Applications (compiled for host OS/hardware) - see native/
//! - Web Applications (browser engine) - future
//! - Mobile Applications (iOS/Android touch paradigms) - future
//! - Embedded Software (dedicated hardware: routers, thermostats, cars, etc.) - future
//!
//! Windows and Linux specifics live here.
//!
//! For taxonomy details, see [ARCHITECTURE.md](file:///C:/Users/jeryd/Synology/Home/Projects/local76/rCommon/ARCHITECTURE.md).
//! Cross-platform with native features and platform-specific stubs.

pub mod native;
pub mod web;  // WASM/browser stubs
pub mod mobile; // iOS/Android stubs
pub mod embedded; // Embedded hardware stubs

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PowerStatus {
    pub ac_online: bool,
    pub battery_percent: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SystemBiosInfo {
    pub manufacturer: String,
    pub product: String,
    pub model: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiskDriveInfo {
    pub path: String,
    pub total_bytes: u64,
    pub free_bytes: u64,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NetworkAdapterInfo {
    pub name: String,
    pub description: String,
    pub ip_addresses: Vec<String>,
    pub adapter_type: String, // "Wi-Fi", "Ethernet", "Bluetooth", "Virtual", "Other"
    pub is_up: bool,
}

/// A standard cross-platform query provider trait for system information and hardware metrics.
///
/// Implemented by web, mobile, embedded, and native modules to enable first-class platform fallbacks
/// and clean compile-time or run-time query dispatching.
pub trait PlatformProvider {
    /// Gets the current screen resolution as (width, height) pixels.
    fn get_system_screen_resolution() -> (i32, i32);

    /// Gets the console window DPI scale factor (default 96).
    fn get_console_window_dpi() -> u32;

    /// Queries the system accent color as (r, g, b) bytes.
    fn query_accent_color() -> (u8, u8, u8);

    /// Checks if high contrast accessibility theme is enabled.
    fn query_high_contrast() -> bool;

    /// Queries the operating system version string.
    fn query_os_version() -> String;

    /// Checks if dark mode is preferred by the operating system / environment.
    fn query_dark_mode() -> bool;

    /// Queries current power source and battery levels if available.
    fn query_power_status() -> Option<PowerStatus>;

    /// Queries BIOS or board identification details if available.
    fn query_bios_info() -> Option<SystemBiosInfo>;

    /// Queries shell environment and terminal emulator names.
    fn query_shell_and_terminal() -> (String, String);

    /// Enumerates local storage disk drives and free space metrics.
    fn query_disk_drives() -> Vec<DiskDriveInfo>;

    /// Gets the names of installed graphics processing units.
    fn query_gpu_names() -> Vec<String>;

    /// Enumerates active and inactive network adapters and IP addresses.
    fn query_network_adapters() -> Vec<NetworkAdapterInfo>;

    /// Lists connected displays and monitor details.
    fn get_all_monitors() -> Vec<String>;
}

#[cfg(target_os = "windows")]
pub use native::sys_info::WindowsPlatform;

#[cfg(target_os = "linux")]
pub use native::sys_info::LinuxPlatform;

#[cfg(all(
    not(any(target_os = "windows", target_os = "linux")),
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios")),
    not(any(target_os = "none", target_os = "uefi"))
))]
pub use native::sys_info::FallbackPlatform;

pub use web::WebPlatform;
pub use mobile::MobilePlatform;
pub use embedded::EmbeddedPlatform;

#[cfg(target_os = "windows")]
pub type CurrentPlatform = WindowsPlatform;

#[cfg(target_os = "linux")]
pub type CurrentPlatform = LinuxPlatform;

#[cfg(all(not(any(target_os = "windows", target_os = "linux")), target_arch = "wasm32"))]
pub type CurrentPlatform = WebPlatform;

#[cfg(all(not(any(target_os = "windows", target_os = "linux")), any(target_os = "android", target_os = "ios")))]
pub type CurrentPlatform = MobilePlatform;

#[cfg(all(not(any(target_os = "windows", target_os = "linux")), any(target_os = "none", target_os = "uefi")))]
pub type CurrentPlatform = EmbeddedPlatform;

#[cfg(all(
    not(any(target_os = "windows", target_os = "linux")),
    not(target_arch = "wasm32"),
    not(any(target_os = "android", target_os = "ios")),
    not(any(target_os = "none", target_os = "uefi"))
))]
pub type CurrentPlatform = FallbackPlatform;
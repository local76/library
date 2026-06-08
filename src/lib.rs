// =====================================================
// rCommon - Shared utility library for the local76 rApps ecosystem
// Organized according to the 4-layer taxonomy:
//
// 1. Interface (Presentation Layer)
//    - CLI, TUI, GUI-Native, GUI-Custom/Game-Engine, Headless/API
//
// 2. Execution State (Lifecycle)
//    - Foreground Applications, Background Processes
//
// 3. Platform & Architecture (Deployment)
//    - Native (Windows/Linux), Web, Mobile, Embedded
//
// 4. System Role (Purpose)
//    - System Software (infrastructure), Application Software (task-oriented)
//
// core/ is the only layer that must remain neutral and usable by any combination.
//
// This structure prevents accidental coupling between concerns
// (e.g., a TUI effect type being changed in a way that breaks a background service).
// =====================================================
//
// MIGRATION GUIDE FOR CONSUMERS (Moving off deprecated rcommon::win32):
// - rcommon::win32::SingleInstanceGuard -> rcommon::lifecycle::foreground::guard::SingleInstanceGuard
// - rcommon::win32::hide_console_at_startup -> rcommon::lifecycle::foreground::window::hide_console_at_startup
// - rcommon::win32::query_dark_mode -> rcommon::platform::native::sys_info::query_dark_mode
// - rcommon::win32::TerminalCell -> rcommon::core::TerminalCell
// - rcommon::win32::read_string -> rcommon::platform::native::reg::read_string
// - rcommon::win32::get_packages_breakdown -> rcommon::role::application::packages::get_packages_breakdown
//
// =====================================================


/// Core neutral primitives (TerminalCell, LcgRng, DashboardInfo, etc.).
/// Safe to use from CLI, TUI, background services, or future targets.
pub mod core;
pub mod error;

pub use error::{RcommonError, Result as RcommonResult};
#[cfg(feature = "effects")]
pub use interface::tui::screensaver::{Screensaver, ScreensaverRenderer};

// =====================================================
// 1. Interface (Presentation Layer)
// =====================================================
pub mod interface;

// Backward compatibility re-exports (so existing code like `rcommon::widgets` still works)
#[cfg(feature = "widgets")]
pub use interface::tui::widgets;
#[cfg(feature = "effects")]
pub use interface::tui::effects;
pub use interface::tui::text;
#[cfg(feature = "effects")]
pub use interface::tui::screensaver;
#[cfg(feature = "gui")]
pub use interface::gui::gui as gui;

// =====================================================
// 2. Execution State (Lifecycle)
// =====================================================
pub mod lifecycle;

// Backward compat re-exports
#[cfg(feature = "window")]
pub use lifecycle::foreground::window;
#[cfg(feature = "window")]
pub use lifecycle::foreground::guard;
#[cfg(feature = "service")]
pub use lifecycle::background::service;
#[cfg(feature = "event-log")]
pub use lifecycle::background::event_log;
#[cfg(feature = "notification")]
pub use lifecycle::background::notification;
#[cfg(feature = "clipboard")]
pub use lifecycle::background::clipboard;
pub use lifecycle::background::daemon;

// =====================================================
// 3. Platform & Architecture (Deployment)
// =====================================================
pub mod platform;

// Backward compat
pub use platform::native::sys_info;
pub use platform::native::reg;

// =====================================================
// 4. System Role (Purpose)
// =====================================================
pub mod role;

// Backward compat for application role
pub use role::application::rgb;
pub use role::application::game;
pub use role::application::packages::{
    count_scoop, count_choco, count_npm, count_steam, count_ms_store, count_native, count_winget, count_dpkg, count_pacman,
    count_flatpak, count_snap, PackageManager, PACKAGE_MANAGERS, get_packages_breakdown
};

// Platform native additions (monitors)
pub use platform::native::monitors::{get_monitors_summary, get_all_monitors};

// Lifecycle foreground additions (advanced console helpers and window)
#[cfg(feature = "window")]
pub use lifecycle::foreground::window::hide_console_at_startup;
#[cfg(feature = "window")]
pub use lifecycle::foreground::window::{
    RECT, MONITORINFO, COORD, SMALL_RECT, CONSOLE_SELECTION_INFO, POINT,
    get_console_rect, get_window_rect, set_window_pos, center_console_window, query_cursor_pos,
    relaunch_in_conhost_if_needed, should_relaunch_in_conhost, relaunch_in_conhost,
    is_console_focused,
    BorderlessConsole, ConsoleTitleGuard,
    SingleInstanceGuard
};
#[cfg(feature = "window")]
pub use lifecycle::foreground::console::{
    query_high_contrast, console_window_rect, update_screensaver_active,
    update_screensaver_timeout, get_console_title, set_console_title,
    hide_console_scrollbar
};

// Core enhancements
pub use core::SystemInfo;
pub use platform::native::sys_info::get_system_info;

// Theme enhancements
pub use platform::native::sys_info::{SystemTheme, query_accent_color, query_system_theme};

// Legacy win32 shim kept for compatibility with older consumers.
// It re-uses the re-exports above.
#[cfg(feature = "win32")]
#[deprecated(since = "1.0.0", note = "Use the domain-specific modules (interface, lifecycle, platform, role) instead")]
#[doc(hidden)]
#[allow(unused_imports)]
pub mod win32 {
    #[cfg(feature = "clipboard")]
    pub use crate::lifecycle::background::clipboard::copy_text_to_clipboard;
    
    #[cfg(feature = "service")]
    pub use crate::lifecycle::background::service::{query_service_status as query_windows_service_status, SERVICE_STATUS};
    
    #[cfg(feature = "event-log")]
    pub use crate::lifecycle::background::event_log::log_system_event as log_windows_event;
    
    #[cfg(feature = "notification")]
    pub use crate::lifecycle::background::notification::{show_toast_notification, show_toast_notification_with_id};
    
    #[cfg(feature = "window")]
    pub use crate::lifecycle::foreground::window::{
        RECT, MONITORINFO, COORD, SMALL_RECT, CONSOLE_SELECTION_INFO, POINT,
        get_console_rect, get_window_rect, set_window_pos, center_console_window, query_cursor_pos,
        relaunch_in_conhost_if_needed, should_relaunch_in_conhost, relaunch_in_conhost,
        hide_console_at_startup, is_console_focused,
        BorderlessConsole, ConsoleTitleGuard,
        SingleInstanceGuard
    };
    
    #[cfg(feature = "sys-info")]
    pub use crate::platform::native::sys_info::{
        PowerStatus, SystemBiosInfo, DiskDriveInfo, NetworkAdapterInfo, PlatformProvider, GlyphMap, DashboardInfo, SystemInfo,
        get_dwm_accent_color, get_system_screen_resolution, get_console_window_dpi, query_accent_color, query_high_contrast, query_system_theme, query_os_version, query_dark_mode, query_power_status, query_bios_info, query_shell_and_terminal, query_disk_drives, query_gpu_names, query_network_adapters, query_all_local_ips, query_local_ip, get_dashboard_info, get_system_info, SystemTheme
    };
    
    #[cfg(feature = "effects")]
    pub use crate::interface::tui::effects::{
        TuiEffect, Particle, RainDrop, MatrixRain, SimpleParticles, GravityParticles, GravityCenter, RainEffect, FireEffect, render_logo_block
    };

    #[cfg(feature = "effects")]
    pub use crate::core::{TerminalCell, LcgRng};

    pub use crate::role::application::packages::{
        count_scoop, count_choco, count_npm, count_steam, count_ms_store, count_native, count_winget, count_dpkg, count_pacman, count_flatpak, count_snap, PackageManager, PACKAGE_MANAGERS, get_packages_breakdown
    };
    pub use crate::platform::native::monitors::{get_monitors_summary, get_all_monitors};
    
    #[cfg(feature = "window")]
    pub use crate::lifecycle::foreground::console::{
        console_window_rect, update_screensaver_active, update_screensaver_timeout, get_console_title, set_console_title, hide_console_scrollbar
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_ip() {
        #[cfg(feature = "sys-info")]
        {
            let ip = sys_info::query_local_ip();
            println!("Local IP: {:?}", ip);
        }
    }

    #[test]
    fn test_xml_escaping() {
        #[cfg(feature = "notification")]
        notification::show_toast_notification("<test>&", "\"message'\"");
    }

    #[test]
    fn test_registry_persistence() {
        #[cfg(feature = "reg")]
        {
            let key_name = "test_config_key";
            let path = "Software\\rApps\\Test";
            
            // 1. Initial read should be None
            let _ = reg::delete_value(reg::HKEY_CURRENT_USER, path, key_name);
            let val_init = reg::read_string(reg::HKEY_CURRENT_USER, path, key_name);
            assert_eq!(val_init, None);

            // 2. Write key
            let write_ok = reg::write_string(reg::HKEY_CURRENT_USER, path, key_name, "hello_world");
            if let Err(ref e) = write_ok {
                panic!("Write failed with error: {:?}", e);
            }
            assert!(write_ok.is_ok());

            // 3. Read back
            let val = reg::read_string(reg::HKEY_CURRENT_USER, path, key_name);
            assert_eq!(val, Some("hello_world".to_string()));

            // 4. Delete key
            let delete_ok = reg::delete_value(reg::HKEY_CURRENT_USER, path, key_name);
            assert!(delete_ok.is_ok());

            // 5. Read back again
            let val_post = reg::read_string(reg::HKEY_CURRENT_USER, path, key_name);
            assert_eq!(val_post, None);
        }
    }

    #[test]
    fn test_sys_info_stubs() {
        #[cfg(feature = "sys-info")]
        {
            let res = sys_info::get_system_screen_resolution();
            assert!(res.0 > 0 && res.1 > 0);
            let dpi = sys_info::get_console_window_dpi();
            assert!(dpi > 0);
        }
    }

    #[test]
    fn test_platform_provider_implementations() {
        #[cfg(feature = "sys-info")]
        {
            use crate::platform::{PlatformProvider, WebPlatform, MobilePlatform, EmbeddedPlatform, CurrentPlatform};

            // 1. CurrentPlatform
            let res = CurrentPlatform::get_system_screen_resolution();
            assert!(res.0 > 0 && res.1 > 0);
            let dpi = CurrentPlatform::get_console_window_dpi();
            assert!(dpi > 0);
            let _accent = CurrentPlatform::query_accent_color();
            let _hc = CurrentPlatform::query_high_contrast();
            let _os = CurrentPlatform::query_os_version();
            let _dark = CurrentPlatform::query_dark_mode();
            let _power = CurrentPlatform::query_power_status();
            let _bios = CurrentPlatform::query_bios_info();
            let _shell = CurrentPlatform::query_shell_and_terminal();
            let _disks = CurrentPlatform::query_disk_drives();
            let _gpus = CurrentPlatform::query_gpu_names();
            let _network = CurrentPlatform::query_network_adapters();
            let _monitors = CurrentPlatform::get_all_monitors();

            // 2. WebPlatform
            assert_eq!(WebPlatform::get_system_screen_resolution(), (1920, 1080));
            assert_eq!(WebPlatform::get_console_window_dpi(), 96);
            assert_eq!(WebPlatform::query_accent_color(), (0, 120, 215));
            assert_eq!(WebPlatform::query_os_version(), "Web Browser (WASM)");
            assert!(WebPlatform::query_dark_mode());
            assert!(WebPlatform::query_power_status().is_none());

            // 3. MobilePlatform
            assert_eq!(MobilePlatform::get_system_screen_resolution(), (1080, 2400));
            assert_eq!(MobilePlatform::get_console_window_dpi(), 320);
            assert_eq!(MobilePlatform::query_accent_color(), (103, 80, 164));
            assert!(MobilePlatform::query_dark_mode());
            let mobile_power = MobilePlatform::query_power_status().unwrap();
            assert_eq!(mobile_power.ac_online, false);
            assert_eq!(mobile_power.battery_percent, 85);

            // 4. EmbeddedPlatform
            assert_eq!(EmbeddedPlatform::get_system_screen_resolution(), (320, 240));
            assert_eq!(EmbeddedPlatform::get_console_window_dpi(), 96);
            assert_eq!(EmbeddedPlatform::query_accent_color(), (0, 255, 0));
            assert_eq!(EmbeddedPlatform::query_os_version(), "Embedded Bare-Metal / RTOS");
            assert_eq!(EmbeddedPlatform::query_dark_mode(), false);
            let embedded_power = EmbeddedPlatform::query_power_status().unwrap();
            assert_eq!(embedded_power.ac_online, true);
            assert_eq!(embedded_power.battery_percent, 100);
        }
    }

    #[test]
    fn test_focus_active_helpers() {
        #[cfg(feature = "window")]
        let _ = is_console_focused();

        let mut game = role::application::game::ObstacleJumpGame::new(100.0);
        assert!(game.active);
        game.tick(0.1, false, false);
        assert!(game.timer > 0.0);

        game.active = false;
        let prev_timer = game.timer;
        game.tick(0.1, false, false);
        assert_eq!(game.timer, prev_timer);
    }

    #[test]
    fn test_dashboard_info() {
        #[cfg(feature = "sys-info")]
        {
            let info = sys_info::get_dashboard_info();
            assert!(!info.os.is_empty());
        }
    }

    #[test]
    fn test_daemon_helpers() {
        daemon::set_process_priority(daemon::ProcessPriority::BelowNormal);
        daemon::set_process_priority(daemon::ProcessPriority::Idle);

        #[allow(deprecated)]
        {
            daemon::set_low_priority();
            daemon::set_idle_priority();
        }

        daemon::prevent_system_sleep(true);
        daemon::prevent_system_sleep(false);

        {
            let _guard = daemon::BackgroundPowerGuard::acquire();
        }
    }
}

// =====================================================
// Prep for multi-crate future (per taxonomy sections)
// =====================================================
// When ready, this can become a Cargo workspace:
// [workspace]
// members = ["core", "interface/tui", "lifecycle", "platform/native", "role/*"]
// Each section crate would re-export from its modules.
// For now, single crate keeps git-dep + [patch] simple for r* apps.
// Update consumers gradually to use taxonomy paths (e.g. rcommon::interface::tui).

/// Extension trait to expose background daemon services over IPC.
#[cfg(feature = "interface-api")]
pub trait DaemonIpcExt {
    /// Exposes the daemon service via IPC using the Headless/API layer.
    /// Binds to a local named pipe (Windows) or domain socket (Unix) named after the daemon,
    /// and listens for requests.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use rcommon::lifecycle::background::daemon::{DaemonService, DaemonConfig};
    /// use rcommon::interface::api::{IpcResponse, IpcRequest};
    /// use rcommon::DaemonIpcExt;
    ///
    /// let config = DaemonConfig::new("my_daemon");
    /// let daemon = DaemonService::bootstrap(config).unwrap();
    ///
    /// daemon.run_ipc_server(|req| {
    ///     match req.command.as_str() {
    ///         "status" => IpcResponse::ok("running", ""),
    ///         _ => IpcResponse::err("unknown command"),
    ///     }
    /// }).unwrap();
    /// ```
    fn run_ipc_server<F>(&self, handler: F) -> Result<(), crate::error::RcommonError>
    where
        F: Fn(interface::api::IpcRequest) -> interface::api::IpcResponse;
}

#[cfg(feature = "interface-api")]
impl DaemonIpcExt for lifecycle::background::daemon::DaemonService {
    fn run_ipc_server<F>(&self, handler: F) -> Result<(), crate::error::RcommonError>
    where
        F: Fn(interface::api::IpcRequest) -> interface::api::IpcResponse
    {
        let host = interface::api::IpcServiceHost::new(self.name())?;
        host.run(handler);
        Ok(())
    }
}
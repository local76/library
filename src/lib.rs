// =====================================================
// library - Shared utility library for the local76 apps ecosystem
// Reorganized in 2026.6.9 into a simplified flat folder tree:
// 1. core/ (neutral foundation)
// 2. ui/ (widgets & design)
// 3. backend/ (platform & deployment)
// 4. app/ (controllers & lifecycle)
// 5. scenes/ (screensaver effects)
// =====================================================

// New simplified modules
pub mod core;
pub mod ui;
pub mod toolkit;
pub mod apps;
pub mod screensavers;

// Backward-compatibility shims (re-exporting from the new structure)
pub mod interface;
pub mod lifecycle;
pub mod platform;
pub mod role;
pub mod screensaver_runtime;

// Re-export error and main traits
pub mod error {
    pub use crate::core::error::*;
}
pub use error::{LibraryError, Result as LibraryResult};
#[cfg(feature = "effects")]
pub use interface::tui::screensaver::{Screensaver, ScreensaverRenderer};

// =====================================================
// Backward compatibility re-exports (3.x -> 4.x)
// =====================================================
#[cfg(feature = "widgets")]
pub use interface::tui::widgets;
#[cfg(feature = "effects")]
pub use interface::tui::effects;
#[cfg(feature = "widgets")]
pub use interface::tui::text;
#[cfg(feature = "effects")]
#[allow(deprecated)]
pub use interface::tui::screensaver;
#[cfg(feature = "gui")]
#[allow(deprecated)]
pub use interface::gui::egui_helpers as gui;

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
#[cfg(feature = "window")]
pub use lifecycle::foreground::identity;

#[cfg(feature = "sys-info")]
pub use platform::native::sys_info;
#[cfg(feature = "reg")]
pub use platform::native::reg;

#[cfg(feature = "rgb")]
pub use role::application::rgb;
#[cfg(feature = "role-application")]
pub use role::application::game;
#[cfg(feature = "sys-info")]
pub use role::application::packages::{
    count_scoop, count_choco, count_npm, count_steam, count_ms_store, count_native, count_winget, count_dpkg, count_pacman,
    count_flatpak, count_snap, count_apk, count_rpm, count_brew, count_emerge, PackageManager, PACKAGE_MANAGERS, get_packages_breakdown
};

pub use platform::native::monitors::{get_monitors_summary, get_all_monitors};

#[cfg(feature = "lifecycle-foreground")]
pub use lifecycle::foreground::set_tui_panic_hook;
#[cfg(all(feature = "window", feature = "widgets"))]
pub use lifecycle::foreground::tui_bootstrap::{is_app_shutting_down, set_app_shutting_down};

#[cfg(feature = "window")]
pub use lifecycle::foreground::window::{hide_console_at_startup, show_console_window};
#[cfg(feature = "window")]
#[allow(deprecated)]
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
pub use core::{SystemInfo, hsl_to_rgb, rgb_to_hsl, write_file_atomic};
#[cfg(feature = "sys-info")]
pub use platform::native::sys_info::get_system_info;

// GPU compute
#[cfg(feature = "gpu")]
pub use platform::native::gpu::{init_headless_gpu, run_compute_shader};

// Theme
#[cfg(feature = "sys-info")]
pub use platform::native::sys_info::{SystemTheme, query_accent_color, query_system_theme};

// Deprecated type aliases
#[cfg(feature = "effects")]
#[deprecated(note = "renamed to FallingGlyphs; use library::ui::effects::FallingGlyphs")]
pub type MatrixRain = ui::effects::FallingGlyphs;
#[cfg(feature = "effects")]
#[deprecated(note = "renamed to FlowingParticles; use library::ui::effects::FlowingParticles")]
pub type SimpleParticles = ui::effects::FlowingParticles;
#[cfg(feature = "effects")]
#[deprecated(note = "renamed to PulledParticles; use library::ui::effects::PulledParticles")]
pub type GravityParticles = ui::effects::PulledParticles;
#[cfg(feature = "effects")]
#[deprecated(note = "renamed to FallingDroplets; use library::ui::effects::FallingDroplets")]
pub type RainEffect = ui::effects::FallingDroplets;
#[cfg(feature = "effects")]
#[deprecated(note = "renamed to RisingFlames; use library::ui::effects::RisingFlames")]
pub type FireEffect = ui::effects::RisingFlames;

/// Extension trait to expose background daemon services over IPC.
#[cfg(feature = "interface-api")]
pub trait DaemonIpcExt {
    fn run_ipc_server<F>(&self, handler: F) -> Result<(), crate::core::error::LibraryError>
    where
        F: Fn(interface::api::IpcRequest) -> interface::api::IpcResponse;
}

#[cfg(feature = "interface-api")]
impl DaemonIpcExt for apps::daemon::DaemonService {
    fn run_ipc_server<F>(&self, handler: F) -> Result<(), crate::core::error::LibraryError>
    where
        F: Fn(interface::api::IpcRequest) -> interface::api::IpcResponse
    {
        let host = toolkit::ipc::IpcServiceHost::new(self.name())?;
        host.run(handler);
        Ok(())
    }
}
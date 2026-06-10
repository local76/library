//! Application lifecycle, foreground/background controllers, and CLI executors.
//!
//! **Taxonomy Classification**: Execution State (Lifecycle) / Foreground & Background.

#[cfg(feature = "service")]
pub mod daemon;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "service")]
pub mod worker;

#[cfg(feature = "event-log")]
pub mod event_log;

#[cfg(feature = "lifecycle-background")]
pub mod file_log;

#[cfg(feature = "notification")]
pub mod notification;

#[cfg(feature = "window")]
pub mod guard;
#[cfg(feature = "sys-info")]
pub mod identity;
#[cfg(feature = "lifecycle-foreground")]
pub mod panic;
#[cfg(feature = "lifecycle-foreground")]
pub mod power_sync;

#[cfg(feature = "widgets")]
pub mod tui_bootstrap;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "role-application")]
pub mod game;

#[cfg(feature = "chrome")]
pub mod chrome;

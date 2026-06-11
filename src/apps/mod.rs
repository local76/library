//! Application lifecycle, foreground/background controllers, and CLI executors.

#[cfg(feature = "service")]
pub mod daemon;
#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "service")]
pub mod worker;

#[cfg(feature = "event-log")]
pub mod event_log;

// File-based logging is core to every app. Always available.
pub mod file_log;

#[cfg(feature = "notification")]
pub mod notification;

#[cfg(feature = "window")]
pub mod guard;
#[cfg(feature = "sys-info")]
pub mod identity;
#[cfg(feature = "widgets")]
pub mod panic;
#[cfg(feature = "widgets")]
pub mod power_sync;

#[cfg(feature = "widgets")]
pub mod bootstrap;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "chrome")]
pub mod chrome;

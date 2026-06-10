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

pub mod file_log;

#[cfg(feature = "notification")]
pub mod notification;

pub mod console;
pub mod guard;
pub mod identity;
pub mod panic;
pub mod power_sync;

#[cfg(feature = "widgets")]
pub mod tui_bootstrap;

pub mod window;

#[cfg(feature = "sys-info")]
pub mod doctor;
#[cfg(feature = "sys-info")]
pub mod scaffold;

pub mod game;

//! Background Processes lifecycle.
//!
//! **Taxonomy Classification**: Execution State (Lifecycle - Background).
//!
//! Part of Execution State (Lifecycle).
//! Silent running, services, daemons, no UI focus needed.

// Background code moved here (services, logging, notifications, clipboard for non-UI use).

#[cfg(feature = "service")]
pub mod service;
#[cfg(feature = "event-log")]
pub mod event_log;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "clipboard")]
pub mod clipboard;
pub mod daemon;  // Power/priority for daemons.

// Re-exports
#[cfg(feature = "service")]
pub use service::{
    SERVICE_STATUS, query_service_status, query_windows_service_status, has_admin_privileges, start_service, stop_service, restart_service
};
#[cfg(feature = "event-log")]
pub use event_log::{log_system_event, log_windows_event};
#[cfg(feature = "notification")]
pub use notification::{show_toast_notification, show_toast_notification_with_id};
#[cfg(feature = "clipboard")]
pub use clipboard::copy_text_to_clipboard;
pub use daemon::{
    get_sleep_prevention_count, ProcessPriority, set_process_priority, set_low_priority, set_idle_priority, PowerRequest, set_thread_execution_state, prevent_system_sleep, BackgroundPowerGuard, background_power_guard, DaemonConfig, DaemonPriority, DaemonService,
};
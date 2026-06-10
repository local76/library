//! Backward compatibility shim for lifecycle.
//! Re-exports from the new app and backend modules.

pub mod foreground {
    pub use crate::apps::console;
    pub use crate::apps::guard;
    pub use crate::apps::identity;
    pub use crate::apps::panic;
    pub use crate::apps::panic::set_tui_panic_hook;
    pub use crate::apps::power_sync;
    pub use crate::apps::tui_bootstrap;
    pub use crate::apps::window;

    #[cfg(feature = "window")]
    pub use crate::apps::window::WindowDrag;
}

pub mod background {
    pub use crate::apps::daemon;
    pub use crate::apps::service;
    pub use crate::apps::worker;
    pub use crate::apps::event_log;
    pub use crate::apps::file_log;
    pub use crate::apps::notification;
    pub use crate::toolkit::clipboard;
}

pub use foreground::panic::set_tui_panic_hook;

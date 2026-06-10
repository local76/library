//! Backward compatibility shim for role.
//! Re-exports from the new backend, app, and scenes modules.

pub mod application {
    #[cfg(feature = "sys-info")]
    pub use crate::toolkit::packages;
    
    pub use crate::apps::game;
    pub use crate::core::screen_palette as palette;
    pub use crate::core::formatting;
    pub use crate::screensavers as scenes;
    
    pub mod rgb {
        #[cfg(feature = "rgb")]
        pub use crate::toolkit::rgb_controller as controller;
        #[cfg(feature = "rgb")]
        pub use crate::toolkit::rgb_protocol as protocol;
        
        #[cfg(feature = "rgb")]
        pub use crate::toolkit::rgb_controller::is_openrgb_enabled;
        #[cfg(feature = "rgb")]
        pub use crate::toolkit::rgb_controller::RgbController;
    }
}

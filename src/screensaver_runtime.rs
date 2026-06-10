//! Backward compatibility shim for screensaver_runtime.
//! Re-exports from the new app module.

#[cfg(feature = "screensaver-runtime")]
pub use crate::apps::screensaver_runtime::*;

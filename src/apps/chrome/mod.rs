//! Cross-app TUI chrome helpers — the keyboard, mouse, and theme boilerplate
//! that every local76 TUI app repeats.
//!
//! **Taxonomy Classification**: Interface (TUI / Chrome Layer).
//!
//! Each helper is **field-free**: it doesn't try to own app state. Apps keep
//! their bespoke `App` structs and call into these helpers to reduce ~80 lines
//! of duplicated keyboard/mouse/embedded-docs logic per app.
//!
//! # Submodules
//! - [`embedded_docs`]: F1..F7 documentation bindings (README, SUPPORT, ...).
//! - [`app_state`]: key predicates (quit-key, help-toggle-key, doc-key, ...).
//! - [`chrome_mouse`]: title-bar drag, button hit-tests, text-selection drag, scroll-wheel.

use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};

pub mod embedded_docs;
pub mod app_state;
pub mod chrome_mouse;

pub use embedded_docs::{DOC_FILES, doc_for_f_key, is_doc_f_key, open_embedded_markdown};
pub use app_state::{is_help_toggle_key, is_quit_key, scroll_for_key};
pub use chrome_mouse::{
    BtnRect, ChromeAction, ChromeLayout, handle_chrome_mouse,
};

/// Returns true if this `KeyEvent` is a key-press (not a release/repeat).
pub fn is_press(event_kind: KeyEventKind) -> bool {
    event_kind == KeyEventKind::Press
}

/// Builds the `KeyCode -> quit-key?` check as a free helper for code that
/// already extracted the (code, mods) pair.
#[inline]
pub fn is_quit_code(code: KeyCode, mods: KeyModifiers) -> bool {
    is_quit_key(code, mods)
}

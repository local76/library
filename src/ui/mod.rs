//! User Interface (TUI and GUI) components and layout structures.
//!
//! **Taxonomy Classification**: Interface (TUI/GUI / Presentation Layer).

#[cfg(feature = "widgets")]
pub mod textbox;
#[cfg(feature = "widgets")]
pub mod scrollbar;
#[cfg(feature = "widgets")]
pub mod tabs;

#[cfg(feature = "widgets")]
pub mod status_bar;
#[cfg(feature = "widgets")]
pub mod toast;
#[cfg(feature = "widgets")]
pub mod screensaver_renderer;

#[cfg(feature = "widgets")]
pub mod colors;
#[cfg(feature = "widgets")]
pub mod effect_preview;
#[cfg(feature = "widgets")]
pub mod layout;
#[cfg(feature = "widgets")]
pub mod layout_guard;
#[cfg(feature = "widgets")]
pub mod markdown;
#[cfg(feature = "widgets")]
pub mod mouse_selection;
#[cfg(feature = "widgets")]
pub mod text;
#[cfg(feature = "widgets")]
pub mod theme;
#[cfg(feature = "widgets")]
pub mod title_banner;

#[cfg(feature = "gui")]
pub mod egui_helpers;
#[cfg(feature = "gui")]
pub mod gui_native;

#[cfg(feature = "effects")]
pub mod effects;

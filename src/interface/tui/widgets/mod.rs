//! First-class custom TUI widgets (gauges, lists, textboxes, tabs).
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer).
//!
//! # 4.0 design system split
//!
//! Chrome-y widgets (`colors`, `effect_preview`, `layout_guard`,
//! `mouse_selection`, `title_banner`, `toast`) were moved to
//! [`crate::interface::tui::design`] in 4.0. They are still re-exported
//! here and at the `tui::` level for one minor release as deprecated
//! back-compat. New code should use the `design::` path.
//!
//! The remaining "Accent*" widget family (gauge, list, scrollbar, tabs,
//! textbox) lives here because they are widget-shaped (not chrome-shaped)
//! and don't depend on theme.rs or the status/toast/markdown subsystems.
//!
//! # Focus & Active States
//! - **Focused**: Controls visual emphasis. Focused widgets render with active accent styling (active colors, bold, bright text). Unfocused widgets render with muted, dimmed borders and indicators to preserve visual hierarchy in tab/focus layouts.

// 4.0 widget family (stays here — these are widget-shaped, not chrome-shaped)
pub mod gauge;
pub mod list;
pub mod scrollbar;
pub mod tabs;
pub mod textbox;

// 3.x back-compat re-exports for chrome widgets that moved to design/ in 4.0
#[allow(deprecated)]
pub mod colors {
    pub use crate::interface::tui::design::colors::*;
}
#[allow(deprecated)]
pub mod effect_preview {
    pub use crate::interface::tui::design::effect_preview::*;
}
#[allow(deprecated)]
pub mod layout_guard {
    pub use crate::interface::tui::design::layout_guard::*;
}
#[allow(deprecated)]
pub mod mouse_selection {
    pub use crate::interface::tui::design::mouse_selection::*;
}
#[allow(deprecated)]
pub mod title_banner {
    pub use crate::interface::tui::design::title_banner::*;
}
#[allow(deprecated)]
pub mod toast {
    pub use crate::interface::tui::design::toast::*;
}

// 4.0 widget re-exports
pub use gauge::AccentGauge;
pub use list::AccentList;
pub use scrollbar::AccentScrollbar;
pub use tabs::AccentTabs;
pub use textbox::{AccentTextBox, TextBox};

// 3.x back-compat re-exports for chrome moved to design/ in 4.0
#[allow(deprecated)]
pub use colors::{AccentColors, AccentTheme};
#[allow(deprecated)]
pub use effect_preview::draw_effect_preview;
#[allow(deprecated)]
pub use layout_guard::{is_too_small, render_too_small_warning};
#[allow(deprecated)]
pub use mouse_selection::MouseSelection;
#[allow(deprecated)]
pub use title_banner::{draw_title_banner, ButtonRect};
#[allow(deprecated)]
pub use toast::{ToastBox, ToastKind};

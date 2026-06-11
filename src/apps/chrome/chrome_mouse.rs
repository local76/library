//! Cross-app chrome mouse handler — title-bar drag, quit/help button
//! hit-tests, text-selection drag, markdown scroll-wheel.
//!
//! **Taxonomy Classification**: Interface (Chrome Layer).
//!
//! Apps have bespoke mouse handlers but the **chrome** part is identical:
//! - Click in title rows 0..=2 → start title-bar drag (saves cursor + window pos)
//! - Click on a quit-btn rect → `Quit`
//! - Click on a help-btn rect → `ToggleHelp`
//! - Click elsewhere → start text-selection drag
//! - Drag while text-selection is active → `ExtendTextSelection`
//! - Release after a drag > 1 cell → `EndTextSelection { copied: true }`
//! - Mouse wheel while markdown is open → `ScrollMarkdown { delta }`
//!
//! The app's `app::mouse::handle_mouse` calls `handle_chrome_mouse(layout, event)`
//! first, then dispatches on the returned `ChromeAction`. App-specific modal
//! logic (e.g. clicking into a "Backups" list, clicking into a Wi-Fi list)
//! stays in the app.

use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

/// A clickable button rect in terminal coordinates. Used for the quit and
/// help buttons that every r* app's title banner draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BtnRect {
    pub y: u16,
    pub x_start: u16,
    pub x_end: u16,
}

impl BtnRect {
    pub fn contains(&self, row: u16, col: u16) -> bool {
        row == self.y && col >= self.x_start && col < self.x_end
    }
}

/// Layout of the chrome elements (passed by the app to `handle_chrome_mouse`).
///
/// `title_rows` is the inclusive row range of the draggable title bar (typically
/// `0..=2`). Pass an empty range to disable title-bar drag (e.g. when a modal
/// is open).
#[derive(Debug, Clone, Copy)]
pub struct ChromeLayout {
    pub term_size: (u16, u16),
    pub title_rows_start: u16,
    pub title_rows_end: u16,
    pub quit_btn: Option<BtnRect>,
    pub help_btn: Option<BtnRect>,
}

/// What the chrome handler decided to do with the event. The app's caller
/// dispatches on this to mutate its own state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChromeAction {
    /// Nothing chrome-related happened.
    None,
    /// User clicked the quit button — the app should set `should_quit = true`.
    Quit,
    /// User clicked the help button — the app should toggle `show_help`.
    ToggleHelp,
    /// User clicked the title bar — start a title-bar drag. The app should
    /// snapshot the cursor position and window position into its own state.
    StartTitleDrag,
    /// User released the title bar — end the drag. The app should clear
    /// its drag state.
    EndTitleDrag,
    /// User clicked in the main content area — start a text-selection drag.
    /// The app should store `(col, row)` as the selection start.
    StartTextSelection { col: u16, row: u16 },
    /// User dragged while text-selection is active — extend the selection.
    ExtendTextSelection { col: u16, row: u16 },
    /// User released after a drag > 1 cell horizontally OR >= 1 vertically
    /// (the "micro-jitter guard"). The app should mark the selection as
    /// "pending copy". Otherwise, the app should clear the selection.
    EndTextSelection { copied: bool },
    /// Mouse-wheel scroll while the markdown viewer is open. The app
    /// should bump its `markdown_scroll` by `delta` lines.
    ScrollMarkdown { delta: i32 },
}

/// Top-level chrome handler. Apps call this first; if it returns
/// `ChromeAction::None`, the app's own mouse logic runs (e.g. clicking
/// into a list to select a row).
///
/// `markdown_open` is true if the markdown viewer is currently showing —
/// in that case scroll-wheel events produce `ScrollMarkdown`; otherwise
/// they're `None`.
pub fn handle_chrome_mouse(
    layout: ChromeLayout,
    event: MouseEvent,
    markdown_open: bool,
) -> ChromeAction {
    let (col, row) = (event.column, event.row);

    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            if let Some(btn) = layout.quit_btn {
                if btn.contains(row, col) {
                    return ChromeAction::Quit;
                }
            }
            if let Some(btn) = layout.help_btn {
                if btn.contains(row, col) {
                    return ChromeAction::ToggleHelp;
                }
            }
            if row >= layout.title_rows_start && row <= layout.title_rows_end {
                return ChromeAction::StartTitleDrag;
            }
            ChromeAction::StartTextSelection { col, row }
        }

        MouseEventKind::Drag(MouseButton::Left) => {
            // We can't tell here whether a title drag or text selection is
            // active — that's app state. So we emit ExtendTextSelection;
            // the app's caller routes based on its own drag_active flag.
            // (In practice apps will check `if app.drag.is_active()` first
            // and skip calling into this module for drags.)
            ChromeAction::ExtendTextSelection { col, row }
        }

        MouseEventKind::Up(MouseButton::Left) => {
            // The app's caller is expected to compute the "was it a real
            // drag?" check from its own (start, end) snapshot. We can't
            // compute it here because the layout doesn't carry the start.
            ChromeAction::EndTextSelection { copied: false }
        }

        MouseEventKind::ScrollUp => {
            if markdown_open {
                ChromeAction::ScrollMarkdown { delta: -3 }
            } else {
                ChromeAction::None
            }
        }
        MouseEventKind::ScrollDown => {
            if markdown_open {
                ChromeAction::ScrollMarkdown { delta: 3 }
            } else {
                ChromeAction::None
            }
        }

        _ => ChromeAction::None,
    }
}

/// End-of-text-selection helper. Apps call this with the saved
/// `(start_col, start_row)` and the current `(end_col, end_row)` to decide
/// whether the release should mark the selection as "pending copy" (the
/// common micro-jitter-guard rule across 4 of the 5 apps).
///
/// Returns `copied = true` if the drag was > 1 cell horizontally OR
/// > 0 cells vertically.
pub fn text_selection_release_decision(
    start: (u16, u16),
    end: (u16, u16),
) -> bool {
    let dx = (start.0 as i32 - end.0 as i32).abs();
    let dy = (start.1 as i32 - end.1 as i32).abs();
    dx > 1 || dy > 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{MouseButton, MouseEvent, MouseEventKind};

    fn ev(kind: MouseEventKind, col: u16, row: u16) -> MouseEvent {
        MouseEvent { kind, column: col, row, modifiers: crossterm::event::KeyModifiers::empty() }
    }

    fn layout_with_buttons() -> ChromeLayout {
        ChromeLayout {
            term_size: (100, 35),
            title_rows_start: 0,
            title_rows_end: 2,
            quit_btn: Some(BtnRect { y: 1, x_start: 90, x_end: 96 }),
            help_btn: Some(BtnRect { y: 1, x_start: 80, x_end: 86 }),
        }
    }

    #[test]
    fn click_quit_button() {
        let action = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::Down(MouseButton::Left), 92, 1),
            false,
        );
        assert_eq!(action, ChromeAction::Quit);
    }

    #[test]
    fn click_help_button() {
        let action = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::Down(MouseButton::Left), 82, 1),
            false,
        );
        assert_eq!(action, ChromeAction::ToggleHelp);
    }

    #[test]
    fn click_title_bar_starts_drag() {
        let action = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::Down(MouseButton::Left), 50, 2),
            false,
        );
        assert_eq!(action, ChromeAction::StartTitleDrag);
    }

    #[test]
    fn click_content_starts_selection() {
        let action = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::Down(MouseButton::Left), 50, 20),
            false,
        );
        assert_eq!(action, ChromeAction::StartTextSelection { col: 50, row: 20 });
    }

    #[test]
    fn drag_extends_selection() {
        let action = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::Drag(MouseButton::Left), 60, 25),
            false,
        );
        assert_eq!(action, ChromeAction::ExtendTextSelection { col: 60, row: 25 });
    }

    #[test]
    fn scroll_with_markdown_open() {
        let up = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::ScrollUp, 0, 0),
            true,
        );
        assert_eq!(up, ChromeAction::ScrollMarkdown { delta: -3 });
        let down = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::ScrollDown, 0, 0),
            true,
        );
        assert_eq!(down, ChromeAction::ScrollMarkdown { delta: 3 });
    }

    #[test]
    fn scroll_without_markdown_ignored() {
        let up = handle_chrome_mouse(
            layout_with_buttons(),
            ev(MouseEventKind::ScrollUp, 0, 0),
            false,
        );
        assert_eq!(up, ChromeAction::None);
    }

    #[test]
    fn text_selection_release_no_drag() {
        assert!(!text_selection_release_decision((10, 5), (10, 5)));
    }

    #[test]
    fn text_selection_release_horizontal_drag() {
        assert!(text_selection_release_decision((10, 5), (12, 5)));
    }

    #[test]
    fn text_selection_release_vertical_drag() {
        assert!(text_selection_release_decision((10, 5), (10, 6)));
    }

    #[test]
    fn text_selection_release_tiny_horizontal_only_no_copy() {
        // < 2 cells horizontal, no vertical = no copy
        assert!(!text_selection_release_decision((10, 5), (11, 5)));
    }

    #[test]
    fn btn_rect_contains() {
        let btn = BtnRect { y: 1, x_start: 10, x_end: 20 };
        assert!(btn.contains(1, 10));
        assert!(btn.contains(1, 19));
        assert!(!btn.contains(1, 20));
        assert!(!btn.contains(0, 15));
        assert!(!btn.contains(2, 15));
    }

    #[test]
    fn empty_layout_falls_through_to_selection() {
        let layout = ChromeLayout {
            term_size: (100, 35),
            // Set title_rows_start > title_rows_end so no row can be in range.
            title_rows_start: 5,
            title_rows_end: 2,
            quit_btn: None,
            help_btn: None,
        };
        let action = handle_chrome_mouse(
            layout,
            ev(MouseEventKind::Down(MouseButton::Left), 50, 0),
            false,
        );
        // Row 0 is below the (impossible) title range → falls through
        // to text selection
        assert!(matches!(action, ChromeAction::StartTextSelection { .. }));
    }
}

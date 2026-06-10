//! Window and terminal management for foreground applications.
//!
//! **Taxonomy Classification**: Execution State (Lifecycle - Foreground) + Platform (Native).

#[path = "window_drag_to_move.rs"]
pub mod drag_to_move;
#[path = "window_position.rs"]
pub mod position;
#[path = "window_relaunch.rs"]
pub mod relaunch;
#[path = "window_style.rs"]
pub mod style;
#[path = "window_types.rs"]
pub mod types;
#[path = "window_visibility.rs"]
pub mod visibility;

pub use types::{
    RECT, MONITORINFO, COORD, SMALL_RECT, CONSOLE_SELECTION_INFO, POINT,
};
pub use position::{
    get_console_rect, get_window_rect, set_window_pos, center_console_window, query_cursor_pos,
};
#[allow(deprecated)] // Intentional: Re-exporting legacy relaunch helpers for backward compatibility with older apps
pub use relaunch::{relaunch_in_conhost_if_needed, should_relaunch_in_conhost, relaunch_in_conhost};
pub use visibility::{hide_console_at_startup, is_console_focused, show_console_window};
pub use style::{BorderlessConsole, ConsoleTitleGuard};

// Re-export SingleInstanceGuard from guard module to preserve API compatibility
pub use crate::apps::guard::SingleInstanceGuard;
pub use drag_to_move::WindowDrag;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_window_types_impls() {
        let rect = RECT::default();
        assert_eq!(rect.left, 0);
        assert_eq!(rect.right, 0);

        let pt1 = POINT { x: 10, y: 20 };
        let pt2 = pt1;
        assert_eq!(pt1, pt2);

        let coord = COORD::default();
        assert_eq!(coord.x, 0);

        let small_rect = SMALL_RECT::default();
        assert_eq!(small_rect.left, 0);

        let sel_info = CONSOLE_SELECTION_INFO::default();
        assert_eq!(sel_info.dwFlags, 0);

        let monitor_info = MONITORINFO::default();
        assert_eq!(monitor_info.cbSize, 0);
    }
}


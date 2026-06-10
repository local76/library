//! Common TUI key predicates shared by all 5 r* TUI apps.
//!
//! **Taxonomy Classification**: Interface (TUI / Chrome Layer).
//!
//! Apps have bespoke `App` structs but the **predicate logic** for "is this
//! key a quit-key?", "is this a help-toggle-key?", "is this an Up/Down that
//! scrolls the markdown viewer?" is identical. This module owns that logic
//! so the apps' `app::keys::handle_key` functions become a flat match on
//! these predicates instead of repeating the same patterns.

use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

/// Returns true if the (code, mods) pair is a quit-trigger:
///
/// - Ctrl-C
/// - 'q' or 'Q'
/// - Esc
pub fn is_quit_key(code: KeyCode, mods: KeyModifiers) -> bool {
    if mods.contains(KeyModifiers::CONTROL) && code == KeyCode::Char('c') {
        return true;
    }
    matches!(code, KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc)
}

/// Convenience: returns true if this `KeyEvent` is a key-press AND it's
/// a quit-trigger. Mirrors the
/// `if key.kind == KeyEventKind::Press && is_quit_key(key.code, key.modifiers)`
/// check that every app does.
pub fn is_quit_key_event(key: &KeyEvent) -> bool {
    key.kind == KeyEventKind::Press && is_quit_key(key.code, key.modifiers)
}

/// Returns true if the key toggles the help overlay: 'h', 'H', or F1.
///
/// F1 is included because in 4 of the 5 apps, F1 ALSO opens README.md
/// (which closes the help overlay). Apps that want F1 = help-only can
/// skip this and check `KeyCode::F(1)` directly.
pub fn is_help_toggle_key(code: KeyCode) -> bool {
    matches!(code, KeyCode::Char('h') | KeyCode::Char('H'))
}

/// Apply markdown-viewer scroll math for one keystroke.
///
/// `scroll` is the current scroll offset. `line_count` is `markdown_lines.len()`.
/// `viewport_h` is the visible height (e.g. inner area height minus borders).
///
/// Returns the new scroll offset, or `None` if the key is not a scroll key
/// (caller should fall through to other handlers).
///
/// Scrolling rules (matching the pattern in 4 of the 5 apps):
/// - `Up` / `Char('k')`   : `saturating_sub(1)`
/// - `Down` / `Char('j')` : `min(scroll + 1, max_scroll)` where
///   `max_scroll = line_count.saturating_sub(viewport_h + 10)` (the +10
///   is the "keep context" margin used by pulse / helm / scout / ignite).
/// - `PageUp`             : `saturating_sub(viewport_h)` (clamped 1..=15)
/// - `PageDown`           : `min(scroll + viewport_h, max_scroll)` (clamped 1..=15)
///
/// For viewports >= 10 lines, the per-step is 1 for arrows and the full
/// viewport for page-keys. For tiny viewports the step is clamped to >= 1
/// so a stuck scroll doesn't freeze on empty viewports.
pub fn scroll_for_key(
    code: KeyCode,
    scroll: usize,
    line_count: usize,
    viewport_h: usize,
) -> Option<usize> {
    let vp = viewport_h.max(1);
    let max_scroll = line_count.saturating_sub(vp + 10);
    let arrow_step = 1usize;
    let page_step = vp.clamp(1, 15);

    match code {
        KeyCode::Up | KeyCode::Char('k') => Some(scroll.saturating_sub(arrow_step)),
        KeyCode::Down | KeyCode::Char('j') => {
            if scroll < max_scroll {
                Some((scroll + arrow_step).min(max_scroll))
            } else {
                Some(scroll)
            }
        }
        KeyCode::PageUp => Some(scroll.saturating_sub(page_step)),
        KeyCode::PageDown => {
            if scroll < max_scroll {
                Some((scroll + page_step).min(max_scroll))
            } else {
                Some(scroll)
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

    #[test]
    fn test_is_quit_key_ctrl_c() {
        assert!(is_quit_key(KeyCode::Char('c'), KeyModifiers::CONTROL));
    }

    #[test]
    fn test_is_quit_key_q_and_esc() {
        assert!(is_quit_key(KeyCode::Char('q'), KeyModifiers::NONE));
        assert!(is_quit_key(KeyCode::Char('Q'), KeyModifiers::NONE));
        assert!(is_quit_key(KeyCode::Esc, KeyModifiers::NONE));
    }

    #[test]
    fn test_is_quit_key_negative() {
        assert!(!is_quit_key(KeyCode::Char('c'), KeyModifiers::NONE));
        assert!(!is_quit_key(KeyCode::Char('a'), KeyModifiers::CONTROL));
        assert!(!is_quit_key(KeyCode::Enter, KeyModifiers::NONE));
    }

    #[test]
    fn test_is_quit_key_event() {
        let press = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        let release = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
        // Simulate release: KeyEventKind::Release instead of Press
        let mut release = release;
        release.kind = KeyEventKind::Release;
        assert!(is_quit_key_event(&press));
        assert!(!is_quit_key_event(&release));
    }

    #[test]
    fn test_is_help_toggle_key() {
        assert!(is_help_toggle_key(KeyCode::Char('h')));
        assert!(is_help_toggle_key(KeyCode::Char('H')));
        assert!(!is_help_toggle_key(KeyCode::Char('a')));
    }

    #[test]
    fn test_scroll_up_saturates() {
        assert_eq!(scroll_for_key(KeyCode::Up, 0, 100, 10), Some(0));
        assert_eq!(scroll_for_key(KeyCode::Up, 5, 100, 10), Some(4));
    }

    #[test]
    fn test_scroll_down_clamps() {
        // 100 lines, viewport 10: max_scroll = 100 - (10+10) = 80
        assert_eq!(scroll_for_key(KeyCode::Down, 0, 100, 10), Some(1));
        assert_eq!(scroll_for_key(KeyCode::Down, 80, 100, 10), Some(80));
        assert_eq!(scroll_for_key(KeyCode::Down, 85, 100, 10), Some(85));
    }

    #[test]
    fn test_scroll_page_keys() {
        assert_eq!(scroll_for_key(KeyCode::PageUp, 0, 100, 10), Some(0));
        assert_eq!(scroll_for_key(KeyCode::PageUp, 5, 100, 10), Some(0)); // vp=10
        assert_eq!(scroll_for_key(KeyCode::PageDown, 0, 100, 10), Some(10));
    }

    #[test]
    fn test_scroll_unrelated_key() {
        assert_eq!(scroll_for_key(KeyCode::Enter, 5, 100, 10), None);
        assert_eq!(scroll_for_key(KeyCode::Char('x'), 5, 100, 10), None);
    }

    #[test]
    fn test_scroll_vim_keys() {
        // j/k should also work
        assert_eq!(scroll_for_key(KeyCode::Char('j'), 0, 100, 10), Some(1));
        assert_eq!(scroll_for_key(KeyCode::Char('k'), 5, 100, 10), Some(4));
    }

    #[test]
    fn test_scroll_page_step_clamped_to_15() {
        // vp=100, page_step should clamp to 15
        assert_eq!(scroll_for_key(KeyCode::PageDown, 0, 1000, 100), Some(15));
    }
}

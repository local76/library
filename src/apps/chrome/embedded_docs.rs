//! F1..F7 documentation bindings + lookup helpers.
//!
//! **Taxonomy Classification**: Interface (Chrome Layer).
//!
//! The 5 apps (pulse, helm, scout, ignite, trance) all expose a markdown
//! viewer bound to F1..F7 opening README, SUPPORT, LICENSE, COPYRIGHT,
//! PRIVACY, SECURITY, CONTRIBUTING respectively. The actual `include_str!`
//! of those 7 files happens in each app (the files live in each app's repo
//! root, not in library). This module owns only the **mapping** and the
//! **key-handler** that drives the viewer.

use crossterm::event::KeyCode;

/// Filenames bound to F1..F7 in display order.
pub const DOC_FILES: &[&str] = &[
    "README.md",
    "SUPPORT.md",
    "LICENSE.md",
    "COPYRIGHT.md",
    "PRIVACY.md",
    "SECURITY.md",
    "CONTRIBUTING.md",
];

/// Total number of F-key docs.
pub const DOC_COUNT: u8 = 7;

/// Returns the doc filename for F-key `n` (1..=7), or None otherwise.
pub fn doc_for_f_key(n: u8) -> Option<&'static str> {
    if n >= 1 && n <= DOC_COUNT {
        Some(DOC_FILES[(n - 1) as usize])
    } else {
        None
    }
}

/// Returns the doc filename if `code` is one of F1..F7.
pub fn is_doc_f_key(code: KeyCode) -> Option<&'static str> {
    if let KeyCode::F(n) = code {
        doc_for_f_key(n)
    } else {
        None
    }
}

/// Convenience: returns Some(filename) if this key is F1..F7. Otherwise None.
///
/// The caller is responsible for parsing the file's content (e.g. with
/// `library::ui::markdown::parse_markdown_to_lines`) and storing the lines
/// into the app's own `markdown_lines: Vec<Line<'static>>` field. This keeps
/// the helper field-free: no app-state ownership, no `MarkdownViewerState`
/// migration required.
pub fn open_embedded_markdown(code: KeyCode) -> Option<&'static str> {
    is_doc_f_key(code)
}

/// Look up a doc filename by exact name. Returns None if `name` is not in
/// the F1..F7 list. This is the helper the 5 apps' `keys.rs` files use to
/// avoid duplicating the `DOC_FILES` array (Library helper for the B7
/// cross-crate drift fix).
pub fn doc(name: &str) -> Option<&'static str> {
    DOC_FILES.iter().find(|&&f| f == name).copied()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::KeyCode;

    #[test]
    fn test_doc_for_f_key_in_range() {
        assert_eq!(doc_for_f_key(1), Some("README.md"));
        assert_eq!(doc_for_f_key(2), Some("SUPPORT.md"));
        assert_eq!(doc_for_f_key(3), Some("LICENSE.md"));
        assert_eq!(doc_for_f_key(4), Some("COPYRIGHT.md"));
        assert_eq!(doc_for_f_key(5), Some("PRIVACY.md"));
        assert_eq!(doc_for_f_key(6), Some("SECURITY.md"));
        assert_eq!(doc_for_f_key(7), Some("CONTRIBUTING.md"));
    }

    #[test]
    fn test_doc_for_f_key_out_of_range() {
        assert_eq!(doc_for_f_key(0), None);
        assert_eq!(doc_for_f_key(8), None);
        assert_eq!(doc_for_f_key(255), None);
    }

    #[test]
    fn test_is_doc_f_key_dispatches() {
        assert_eq!(is_doc_f_key(KeyCode::F(1)), Some("README.md"));
        assert_eq!(is_doc_f_key(KeyCode::F(7)), Some("CONTRIBUTING.md"));
        assert_eq!(is_doc_f_key(KeyCode::F(8)), None);
        assert_eq!(is_doc_f_key(KeyCode::Char('a')), None);
    }

    #[test]
    fn test_open_embedded_markdown() {
        assert_eq!(open_embedded_markdown(KeyCode::F(3)), Some("LICENSE.md"));
        assert_eq!(open_embedded_markdown(KeyCode::Enter), None);
    }

    #[test]
    fn test_doc_files_table() {
        assert_eq!(DOC_FILES.len(), 7);
        assert_eq!(DOC_FILES[0], "README.md");
        assert_eq!(DOC_FILES[6], "CONTRIBUTING.md");
    }
}

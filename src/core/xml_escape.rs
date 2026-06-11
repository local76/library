//! Strict XML 1.0 escape utility.
//!
//! Replaces the duplicated `escape_xml` functions that were scattered across
//! `app-scout` and `notification.rs`. The replacement is XML 1.0 strict:
//! - `&` → `&amp;`
//! - `<` → `&lt;`
//! - `>` → `&gt;`
//! - `"` → `&quot;`
//! - `'` → `&apos;`
//! - NUL (`\0`) → U+FFFD (XML 1.0 forbids NUL)
//! - Other C0 control chars (`\x01`..`\x08`, `\x0B`, `\x0C`, `\x0E`..`\x1F`) → hex-escaped
//! - `\xFFFE` and `\xFFFF` → U+FFFD (XML 1.0 non-characters)
//!
//! This is the centralized implementation that `app-scout` and
//! `apps::notification` now use. (Library helper for the B1 / B2 drift fix.)

/// Escape a string for safe inclusion in XML 1.0 element or attribute content.
pub fn escape(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            '\0' => out.push('\u{FFFD}'),
            '\x01'..='\x08' | '\x0B' | '\x0C' | '\x0E'..='\x1F' => {
                out.push_str(&format!("&#x{:X};", c as u32));
            }
            '\u{FFFE}' | '\u{FFFF}' => out.push('\u{FFFD}'),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ampersand() {
        assert_eq!(escape("a&b"), "a&amp;b");
    }

    #[test]
    fn test_lt_gt() {
        assert_eq!(escape("<x>"), "&lt;x&gt;");
    }

    #[test]
    fn test_quote() {
        assert_eq!(escape(r#"a"b'c"#), "a&quot;b&apos;c");
    }

    #[test]
    fn test_nul_replaced() {
        assert_eq!(escape("a\0b"), "a\u{FFFD}b");
    }

    #[test]
    fn test_control_hex_escaped() {
        assert_eq!(escape("a\x01b"), "a&#x1;b");
    }

    #[test]
    fn test_noncharacter_replaced() {
        assert_eq!(escape("a\u{FFFE}b"), "a\u{FFFD}b");
    }
}

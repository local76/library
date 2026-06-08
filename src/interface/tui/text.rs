//! Text wrapping and paragraph alignment utility helpers.
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer).
//!
//! Helps custom terminal grids, screensavers, and widgets wrap and align paragraphs
//! cleanly without spilling outside of their dedicated rendering boundaries.

/// Supported text alignments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

/// Wraps text into lines that do not exceed `max_width` characters, wrapping at word boundaries.
/// Maintains existing explicit newlines from the input.
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    if text.is_empty() {
        return Vec::new();
    }
    if max_width == 0 {
        return vec![text.to_string()];
    }
    
    let mut lines = Vec::new();
    for paragraph in text.split('\n') {
        let mut current_line = String::new();
        let mut current_len = 0;
        for word in paragraph.split_whitespace() {
            let word_len = word.chars().count();
            if current_line.is_empty() {
                if word_len > max_width {
                    // Word is longer than line width, force split it using characters
                    let chars: Vec<char> = word.chars().collect();
                    let mut start = 0;
                    while start < chars.len() {
                        let end = (start + max_width).min(chars.len());
                        lines.push(chars[start..end].iter().collect());
                        start = end;
                    }
                } else {
                    current_line.push_str(word);
                    current_len = word_len;
                }
            } else if current_len + 1 + word_len <= max_width {
                current_line.push(' ');
                current_line.push_str(word);
                current_len += 1 + word_len;
            } else {
                lines.push(current_line);
                current_line = word.to_string();
                current_len = word_len;
                if current_len > max_width {
                    // Force split using characters
                    let chars: Vec<char> = current_line.chars().collect();
                    let mut start = 0;
                    while start < chars.len() {
                        let end = (start + max_width).min(chars.len());
                        lines.push(chars[start..end].iter().collect());
                        start = end;
                    }
                    current_line.clear();
                    current_len = 0;
                }
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        } else if paragraph.is_empty() {
            // Keep empty lines from source
            lines.push(String::new());
        }
    }
    lines
}

/// Aligns a single line of text to the specified width using padding.
/// If the text is longer than `width`, it will be truncated.
pub fn align_line(line: &str, width: usize, alignment: TextAlignment) -> String {
    let line_len = line.chars().count();
    if line_len >= width {
        return line.chars().take(width).collect();
    }
    
    let extra_spaces = width - line_len;
    match alignment {
        TextAlignment::Left => {
            format!("{}{}", line, " ".repeat(extra_spaces))
        }
        TextAlignment::Right => {
            format!("{}{}", " ".repeat(extra_spaces), line)
        }
        TextAlignment::Center => {
            let left_pad = extra_spaces / 2;
            let right_pad = extra_spaces - left_pad;
            format!("{}{}{}", " ".repeat(left_pad), line, " ".repeat(right_pad))
        }
    }
}

/// Helper to determine the visual character column width for Unicode/emojis.
pub fn char_width(c: char) -> usize {
    let cp = c as u32;
    if cp >= 0x1F000 {
        2
    } else if cp == 0xFE0F {
        0
    } else {
        1
    }
}

/// Helper to count only the printable/visible character columns in an ANSI-escaped string.
pub fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += char_width(c);
        }
    }
    len
}

/// Helper to split an ANSI-escaped string at a specific visual character column.
pub fn visible_split(s: &str, split_at: usize) -> (String, String) {
    let mut visible_count = 0;
    let mut in_escape = false;
    let mut split_byte_idx = s.len();

    for (byte_idx, c) in s.char_indices() {
        if visible_count >= split_at && !in_escape {
            split_byte_idx = byte_idx;
            break;
        }
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            visible_count += char_width(c);
        }
    }

    let left = s[..split_byte_idx].to_string();
    let right = s[split_byte_idx..].to_string();
    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text() {
        let text = "Hello world from a very long line of text that should definitely be wrapped.";
        let wrapped = wrap_text(text, 15);
        for line in &wrapped {
            assert!(line.len() <= 15, "Line too long: '{}' (len {})", line, line.len());
        }
        assert_eq!(wrapped[0], "Hello world");
    }

    #[test]
    fn test_align_line() {
        let line = "abc";
        assert_eq!(align_line(line, 5, TextAlignment::Left), "abc  ");
        assert_eq!(align_line(line, 5, TextAlignment::Right), "  abc");
        assert_eq!(align_line(line, 5, TextAlignment::Center), " abc ");
    }

    #[test]
    fn test_wrap_unicode_word() {
        // Japanese text: "こんにちは世界" (Hello World)
        // 7 characters, 21 bytes
        let text = "こんにちは世界";
        let wrapped = wrap_text(text, 4);
        assert_eq!(wrapped.len(), 2);
        assert_eq!(wrapped[0], "こんにち");
        assert_eq!(wrapped[1], "は世界");
    }
}

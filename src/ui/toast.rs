use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Widget},
};

/// Category of console visual toast notifications.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastKind {
    Success,
    Warning,
    Error,
    Info,
}

/// An overlay widget to render dynamic visual alerts directly inside a console frame layout.
pub struct ToastBox<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub kind: ToastKind,
    pub accent_color: Color,
    pub dim_color: Color,
    pub text_color: Color,
}

impl<'a> ToastBox<'a> {
    pub fn new(
        title: &'a str,
        message: &'a str,
        kind: ToastKind,
        accent_color: Color,
        dim_color: Color,
        text_color: Color,
    ) -> Self {
        Self {
            title,
            message,
            kind,
            accent_color,
            dim_color,
            text_color,
        }
    }
}

impl<'a> Widget for ToastBox<'a> {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        if area.height < 3 || area.width < 10 {
            return;
        }

        // Determine border and status colors based on ToastKind
        let (border_color, icon) = match self.kind {
            ToastKind::Success => (Color::Rgb(0, 255, 127), "✔️"),
            ToastKind::Error => (Color::Rgb(255, 85, 85), "❌"),
            ToastKind::Warning => (Color::Rgb(250, 210, 50), "⚠️"),
            ToastKind::Info => (self.accent_color, "ℹ️"),
        };

        // Render block with borders
        let block_title = format!(" {} {} ", icon, self.title);
        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(block_title, Style::default().fg(border_color).add_modifier(Modifier::BOLD)));
        let inner_area = block.inner(area);
        block.render(area, buf);

        // Render message text (wrapped to fit inner_area)
        let words: Vec<&str> = self.message.split_whitespace().collect();
        let mut lines = Vec::new();
        let mut current_line = String::new();
        for word in words {
            if current_line.is_empty() {
                current_line.push_str(word);
            } else if current_line.len() + 1 + word.len() <= inner_area.width as usize {
                current_line.push(' ');
                current_line.push_str(word);
            } else {
                lines.push(current_line);
                current_line = word.to_string();
            }
        }
        if !current_line.is_empty() {
            lines.push(current_line);
        }

        for (idx, line) in lines.iter().enumerate() {
            if idx >= inner_area.height as usize {
                break;
            }
            let cx = inner_area.x;
            let cy = inner_area.y + idx as u16;
            
            let truncated: String = line.chars().take(inner_area.width as usize).collect();
            for (char_idx, c) in truncated.chars().enumerate() {
                buf[(cx + char_idx as u16, cy)]
                    .set_char(c)
                    .set_fg(self.text_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;

    #[test]
    fn test_toast_box_rendering() {
        let colors_accent = Color::Cyan;
        let colors_dim = Color::Gray;
        let colors_text = Color::White;

        // Success Toast
        let toast_success = ToastBox::new("SuccessTitle", "SuccessMsg", ToastKind::Success, colors_accent, colors_dim, colors_text);
        let mut buf = Buffer::empty(Rect::new(0, 0, 30, 5));
        toast_success.render(Rect::new(0, 0, 30, 5), &mut buf);
        
        let mut found_success_icon = false;
        let mut found_msg = false;
        for y in 0..5 {
            for x in 0..30 {
                let cell = &buf[(x, y)];
                if cell.symbol().contains('✔') || cell.symbol().contains("✔️") {
                    found_success_icon = true;
                }
                if cell.symbol() == "S" {
                    found_msg = true;
                }
            }
        }
        assert!(found_success_icon, "Success icon ✔️ should be rendered");
        assert!(found_msg, "Message text should be rendered");

        // Error Toast
        let toast_error = ToastBox::new("ErrorTitle", "ErrorMsg", ToastKind::Error, colors_accent, colors_dim, colors_text);
        let mut buf_err = Buffer::empty(Rect::new(0, 0, 30, 5));
        toast_error.render(Rect::new(0, 0, 30, 5), &mut buf_err);
        let mut found_error_icon = false;
        for y in 0..5 {
            for x in 0..30 {
                if buf_err[(x, y)].symbol().contains('❌') || buf_err[(x, y)].symbol().contains('X') {
                    found_error_icon = true;
                }
            }
        }
        assert!(found_error_icon, "Error icon ❌ should be rendered");
    }
}

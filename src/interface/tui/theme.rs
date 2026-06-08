//! Theme coloring utility and factory for ratatui-based TUIs.
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer).

use ratatui::style::Color;

/// Theme color definitions for styling TUI panels and text.
#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub border: Color,
    pub border_active: Color,
    pub text_main: Color,
    pub text_dim: Color,
    pub accent: Color,
    pub username: Color,
    pub help_btn: Color,
    pub quit_btn: Color,
    pub warning: Color,
    pub success: Color,
    pub selection_bg: Color,
}

/// Factory function to retrieve light or dark theme presets.
pub fn get_theme(dark: bool, accent_color: Color) -> ThemeColors {
    if dark {
        ThemeColors {
            border: Color::Rgb(68, 68, 84),
            border_active: accent_color,
            text_main: Color::Rgb(248, 248, 242),
            text_dim: Color::Rgb(136, 136, 153),
            accent: accent_color,
            username: Color::Rgb(255, 215, 0),
            help_btn: Color::Rgb(250, 210, 50),
            quit_btn: Color::Rgb(255, 85, 85),
            warning: Color::Rgb(255, 85, 85),
            success: Color::Rgb(0, 255, 127),
            selection_bg: Color::Rgb(0, 120, 215),
        }
    } else {
        ThemeColors {
            border: Color::Rgb(180, 180, 190),
            border_active: accent_color,
            text_main: Color::Rgb(40, 42, 54),
            text_dim: Color::Rgb(100, 100, 115),
            accent: accent_color,
            username: Color::Rgb(218, 165, 32),
            help_btn: Color::Rgb(204, 153, 0),
            quit_btn: Color::Rgb(200, 50, 50),
            warning: Color::Rgb(200, 50, 50),
            success: Color::Rgb(0, 180, 90),
            selection_bg: Color::Rgb(180, 215, 255),
        }
    }
}

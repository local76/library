//! Theme coloring utility and factory for ratatui-based TUIs.
//!
//! **Taxonomy Classification**: Interface (Presentation Layer).

use ratatui::style::Color;

/// Theme color definitions for styling console panels and text.
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
    pub selection_fg: Color,
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
            quit_btn: Color::Rgb(255, 85, 85), // Red
            warning: Color::Rgb(255, 165, 0),  // Amber/Orange
            success: Color::Rgb(0, 255, 127),
            selection_bg: Color::Rgb(0, 120, 215),
            selection_fg: Color::White,
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
            quit_btn: Color::Rgb(200, 50, 50), // Red
            warning: Color::Rgb(220, 100, 0),  // Amber/Orange
            success: Color::Rgb(0, 180, 90),
            selection_bg: Color::Rgb(180, 215, 255),
            selection_fg: Color::Rgb(40, 42, 54),
        }
    }
}

/// Parse color from a 7-character hex string (e.g. "#ff0000"), falling back to default cyan if invalid.
pub fn accent_color_from_hex(hex: &str) -> Color {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap_or(245);
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap_or(255);
        Color::Rgb(r, g, b)
    } else {
        Color::Rgb(0, 245, 255)
    }
}

/// Build a `ThemeColors` from the user's `theme_mode` preference and the
/// system accent color in one call.
///
/// `theme_mode` is the same string the apps store in their config:
/// - `"dark"`  → force dark theme
/// - `"light"` → force light theme
/// - `"auto"`  (or anything else) → query the OS for dark/light preference
///
/// The accent color is always read from the OS (DWM on Windows, XDG on
/// Linux); apps that need a user-overridden accent should call
/// `get_theme(dark, accent_color_from_hex(override_hex))` directly.
///
/// This is the helper every console app's "init theme" / "refresh theme" path
/// uses — it replaces the 3-line dance of `match theme_mode.as_str()`,
/// `query_dark_mode`, and `query_accent_color` that pulse, helm, scout, and
/// ignite each had duplicated in their `App::new` / `App::refresh_theme`.
#[cfg(feature = "sys-info")]
pub fn current_theme(theme_mode: &str) -> ThemeColors {
    let dark = match theme_mode {
        "dark" => true,
        "light" => false,
        _ => crate::toolkit::sys_info::query_dark_mode(),
    };
    let (r, g, b) = crate::toolkit::sys_info::query_accent_color();
    get_theme(dark, Color::Rgb(r, g, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_colors_coverage() {
        let accent = Color::Cyan;

        // 1. Test Dark Theme
        let dark = get_theme(true, accent);
        assert_eq!(dark.accent, accent);
        assert_eq!(dark.success, Color::Rgb(0, 255, 127));
        assert_eq!(dark.selection_bg, Color::Rgb(0, 120, 215));
        assert_eq!(dark.warning, Color::Rgb(255, 165, 0));
        assert_eq!(dark.quit_btn, Color::Rgb(255, 85, 85));
        // Verify quit_btn and warning are different semantic colors
        assert_ne!(dark.warning, dark.quit_btn);

        // 2. Test Light Theme
        let light = get_theme(false, accent);
        assert_eq!(light.accent, accent);
        assert_eq!(light.success, Color::Rgb(0, 180, 90));
        assert_eq!(light.selection_bg, Color::Rgb(180, 215, 255));
        assert_eq!(light.warning, Color::Rgb(220, 100, 0));
        assert_eq!(light.quit_btn, Color::Rgb(200, 50, 50));
        // Verify quit_btn and warning are different semantic colors
        assert_ne!(light.warning, light.quit_btn);
    }

    #[cfg(feature = "sys-info")]
    #[test]
    fn test_current_theme_force_dark() {
        // "dark" string always produces a dark theme (no OS query)
        let t = current_theme("dark");
        // Dark theme has light text and a vivid accent
        assert_eq!(t.text_main, Color::Rgb(248, 248, 242));
    }

    #[cfg(feature = "sys-info")]
    #[test]
    fn test_current_theme_force_light() {
        let t = current_theme("light");
        // Light theme has dark text
        assert_eq!(t.text_main, Color::Rgb(40, 42, 54));
    }
}

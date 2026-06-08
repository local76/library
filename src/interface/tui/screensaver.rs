//! Reusable screensaver trait and renderer.
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer) + Role (Application Software).
//!
//! # Focus & Active States
//! - **Focused**: Controls visual emphasis. Focused screensavers render with full brightness. Unfocused screensavers dim to 50% brightness to denote background status.
//! - **Active**: Controls CPU/resource utilization. Active screensavers update physics and animate normally. Inactive screensavers pause ticks to conserve CPU cycles.
//!
//! Enabled with the `effects` feature. Generalizes TUI-based visual effects
//! into a first-class `Screensaver` trait with initialization, physics update,
//! grid drawing, active/focused hooks, and a helper `ScreensaverRenderer`.

use crate::core::TerminalCell;

/// A trait representing a TUI-based screensaver with a structured lifecycle.
pub trait Screensaver: ScreensaverState + ScreensaverEffect {}

pub trait ScreensaverState {
    fn active(&self) -> bool;
    fn set_active(&mut self, active: bool);
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
}

pub trait ScreensaverEffect {
    fn init(&mut self, cols: usize, rows: usize);
    fn update(&mut self, dt: f32, cols: usize, rows: usize);
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize);
}

impl<T: ScreensaverState + ScreensaverEffect + ?Sized> Screensaver for T {}

/// Helper utility that manages the execution and rendering of a Screensaver.
/// Handles buffer management, calling update/draw lifecycle hooks, checking
/// dimensions, and automatically dimming rendering when the screensaver lacks focus.
pub struct ScreensaverRenderer {
    cols: usize,
    rows: usize,
    grid: Vec<TerminalCell>,
    pub dim_factor: u8,
    was_focused: Option<bool>,
}

impl ScreensaverRenderer {
    /// Creates a new ScreensaverRenderer with specified grid dimensions and dim factor (0-255).
    pub fn new(cols: usize, rows: usize, dim_factor: u8) -> Self {
        Self {
            cols,
            rows,
            grid: vec![TerminalCell::default(); cols * rows],
            dim_factor,
            was_focused: None,
        }
    }

    /// Resize the internal grid/buffer if the dimensions changed.
    pub fn resize(&mut self, cols: usize, rows: usize) {
        if cols != self.cols || rows != self.rows {
            self.cols = cols;
            self.rows = rows;
            self.grid = vec![TerminalCell::default(); cols * rows];
            self.was_focused = None;
        }
    }

    /// Update and render the screensaver onto the internal grid.
    /// Automatically manages clearing the buffer and dimming when the screensaver is unfocused.
    pub fn tick<S: Screensaver + ?Sized>(&mut self, saver: &mut S, dt: f32) {
        let active = saver.active();
        let focused = saver.focused();

        if !active && self.was_focused == Some(focused) {
            return;
        }

        if active {
            saver.update(dt, self.cols, self.rows);
        }

        // Clear grid
        for cell in &mut self.grid {
            *cell = TerminalCell::default();
        }

        saver.draw(&mut self.grid, self.cols, self.rows);

        // If not focused, dim the drawn cells according to the dim_factor
        if !focused {
            let dim = self.dim_factor;
            for cell in &mut self.grid {
                cell.fg.0 = ((cell.fg.0 as u16 * dim as u16) >> 8) as u8;
                cell.fg.1 = ((cell.fg.1 as u16 * dim as u16) >> 8) as u8;
                cell.fg.2 = ((cell.fg.2 as u16 * dim as u16) >> 8) as u8;
            }
        }

        self.was_focused = Some(focused);
    }

    /// Access the rendered grid/buffer.
    pub fn grid(&self) -> &[TerminalCell] {
        &self.grid
    }

    /// Access the grid/buffer mutably.
    pub fn grid_mut(&mut self) -> &mut [TerminalCell] {
        &mut self.grid
    }

    /// Get current column count.
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get current row count.
    pub fn rows(&self) -> usize {
        self.rows
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockScreensaver {
        active: bool,
        focused: bool,
        init_called: bool,
        update_called: bool,
        draw_called: bool,
    }

    impl ScreensaverState for MockScreensaver {
        fn active(&self) -> bool {
            self.active
        }

        fn set_active(&mut self, active: bool) {
            self.active = active;
        }

        fn set_focused(&mut self, focused: bool) {
            self.focused = focused;
        }

        fn focused(&self) -> bool {
            self.focused
        }
    }

    impl ScreensaverEffect for MockScreensaver {
        fn init(&mut self, _cols: usize, _rows: usize) {
            self.init_called = true;
        }

        fn update(&mut self, _dt: f32, _cols: usize, _rows: usize) {
            self.update_called = true;
        }

        fn draw(&mut self, grid: &mut [TerminalCell], _cols: usize, _rows: usize) {
            self.draw_called = true;
            if !grid.is_empty() {
                grid[0] = TerminalCell {
                    ch: 'M',
                    fg: (255, 255, 255),
                    bg: (0, 0, 0),
                    bold: true,
                };
            }
        }
    }

    #[test]
    fn test_screensaver_lifecycle_and_renderer() {
        let mut renderer = ScreensaverRenderer::new(10, 5, 128);
        assert_eq!(renderer.cols(), 10);
        assert_eq!(renderer.rows(), 5);
        assert_eq!(renderer.grid().len(), 50);

        let mut saver = MockScreensaver {
            active: true,
            focused: true,
            init_called: false,
            update_called: false,
            draw_called: false,
        };

        // Tick once
        renderer.tick(&mut saver, 0.1);
        assert!(saver.update_called);
        assert!(saver.draw_called);
        // Grid should have drawn the character with original brightness
        assert_eq!(renderer.grid()[0].ch, 'M');
        assert_eq!(renderer.grid()[0].fg, (255, 255, 255));

        // Test unfocused dimming
        saver.set_focused(false);
        renderer.tick(&mut saver, 0.1);
        // Grid should have drawn the character with dimmed brightness
        assert_eq!(renderer.grid()[0].ch, 'M');
        assert_eq!(renderer.grid()[0].fg, (127, 127, 127)); // 255 / 2 = 127

        // Test resizing
        renderer.resize(20, 10);
        assert_eq!(renderer.cols(), 20);
        assert_eq!(renderer.rows(), 10);
        assert_eq!(renderer.grid().len(), 200);
    }

    #[test]
    fn test_effects_implementing_screensaver() {
        use crate::interface::tui::effects::MatrixRain;

        let mut rain = MatrixRain::new(10, 5, 0.5);
        assert!(rain.active());
        assert!(rain.focused());

        rain.set_active(false);
        assert!(!rain.active());

        rain.set_focused(false);
        assert!(!rain.focused());

        // Test Screensaver::init works on MatrixRain
        rain.init(15, 10);
        assert_eq!(rain.drops.len(), 7); // (15 * 0.5).max(1) = 7.5 -> 7
    }
}

use crate::core::{LcgRng, TerminalCell};
use super::RainDrop;

/// Rain effect from rPour.
/// Classification: Interface (TUI).
pub struct RainEffect {
    pub(crate) drops: Vec<RainDrop>,
    /// First-class active flag.
    active: bool,
    focused: bool,
    rng: LcgRng,
}

impl RainEffect {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut rng = LcgRng::new(777);
        let drops = (0..(cols / 2).max(1)).map(|_| RainDrop {
            x: rng.next_range(0.0, cols as f32),
            y: rng.next_range(0.0, rows as f32),
            speed: rng.next_range(0.5, 1.5),
            length: 1,
        }).collect();
        Self { drops, active: true, focused: true, rng }
    }

    pub fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        if !self.active {
            return;
        }
        for drop in &mut self.drops {
            drop.y += drop.speed * dt * 15.0;
            if drop.y > rows as f32 {
                drop.y = 0.0;
                drop.x = self.rng.next_range(0.0, cols as f32);
            }
        }
    }

    pub fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        for drop in &self.drops {
            let y = drop.y as usize;
            let x = drop.x as usize;
            if y < rows && x < cols {
                let idx = y * cols + x;
                grid[idx] = TerminalCell { ch: '|', fg: (150, 200, 255), bg: (0,0,0), bold: false };
            }
        }
    }
}

impl crate::interface::tui::screensaver::ScreensaverState for RainEffect {
    fn active(&self) -> bool {
        self.active
    }
    fn set_active(&mut self, active: bool) {
        self.active = active;
    }
    fn focused(&self) -> bool {
        self.focused
    }
    fn set_focused(&mut self, focused: bool) {
        self.focused = focused;
    }
}

impl crate::interface::tui::screensaver::ScreensaverEffect for RainEffect {
    fn init(&mut self, cols: usize, rows: usize) {
        *self = Self::new(cols, rows);
    }
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        self.update(dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        if self.active {
            RainEffect::draw(self, grid, cols, rows);
        }
    }
}

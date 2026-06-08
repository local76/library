use crate::core::{LcgRng, TerminalCell};
use super::Particle;

/// Fire effect from rFire.
/// Classification: Interface (TUI).
pub struct FireEffect {
    pub particles: Vec<Particle>,
    /// First-class active flag for use in effect preview boxes.
    active: bool,
    focused: bool,
    rng: LcgRng,
}

impl FireEffect {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut rng = LcgRng::new(111);
        let count = ((cols * rows) / 5).max(10);
        let particles = (0..count).map(|_| Particle {
            x: rng.next_range(0.0, cols as f32),
            y: rng.next_range(0.0, rows as f32),
            vx: rng.next_range(-0.2, 0.2),
            vy: rng.next_range(-1.0, -0.3),
            ch: '^',
            color: (255, 100, 0),
            life: rng.next_range(0.8, 1.5),
        }).collect();
        Self { particles, active: true, focused: true, rng }
    }

    pub fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        if !self.active {
            return;
        }
        for p in &mut self.particles {
            p.x += p.vx * dt * 10.0;
            p.y += p.vy * dt * 10.0;
            p.life -= dt;
            p.vy -= 0.1 * dt;
            if p.life <= 0.0 {
                p.x = self.rng.next_range(0.0, cols as f32);
                p.y = self.rng.next_range(0.0, rows as f32);
                p.life = self.rng.next_range(0.8, 1.5);
                p.vy = self.rng.next_range(-1.0, -0.3);
            }
        }
    }

    pub fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        for p in &self.particles {
            let x = p.x as usize;
            let y = p.y as usize;
            if x < cols && y < rows {
                let idx = y * cols + x;
                if idx < grid.len() {
                    grid[idx] = TerminalCell {
                        ch: p.ch,
                        fg: (255, (p.life * 150.0) as u8, 0),
                        bg: (0, 0, 0),
                        bold: true,
                    };
                }
            }
        }
    }
}

impl crate::interface::tui::screensaver::ScreensaverState for FireEffect {
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

impl crate::interface::tui::screensaver::ScreensaverEffect for FireEffect {
    fn init(&mut self, cols: usize, rows: usize) {
        *self = Self::new(cols, rows);
    }
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        self.update(dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        if self.active {
            FireEffect::draw(self, grid, cols, rows);
        }
    }
}

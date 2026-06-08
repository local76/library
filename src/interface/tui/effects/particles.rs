//! Example simple particle system for fire / sparks etc.

use crate::core::{LcgRng, TerminalCell};
use super::Particle;

/// Example simple particle system for fire / sparks etc.
pub struct SimpleParticles {
    pub particles: Vec<Particle>,
    /// First-class active flag. When false the effect is paused (no update, draw still renders current state dimmed by caller if desired).
    active: bool,
    focused: bool,
    rng: LcgRng,
}

impl SimpleParticles {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut rng = LcgRng::new(12345);
        let count = ((cols * rows) / 3).max(10);
        let particles = (0..count).map(|_| Particle {
            x: rng.next_range(0.0, cols as f32),
            y: rng.next_range(0.0, rows as f32),
            vx: rng.next_range(-1.0, 1.0),
            vy: rng.next_range(-2.0, -0.5),
            ch: '*',
            color: (255, 200, 50),
            life: rng.next_range(0.5, 2.0),
        }).collect();

        Self { particles, active: true, focused: true, rng }
    }

    pub fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        if !self.active {
            return;
        }
        for p in &mut self.particles {
            p.x += p.vx * dt * 30.0;
            p.y += p.vy * dt * 30.0;
            p.life -= dt;
            p.vy += 1.5 * dt; // gravity
            if p.life <= 0.0 {
                // respawn
                p.x = self.rng.next_range(0.0, cols as f32);
                p.y = self.rng.next_range(0.0, rows as f32);
                p.vx = self.rng.next_range(-1.0, 1.0);
                p.vy = self.rng.next_range(-2.0, -0.5);
                p.life = self.rng.next_range(0.5, 2.0);
            }
        }
    }

    pub fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        for p in &self.particles {
            let x = p.x as usize;
            let y = p.y as usize;
            if x < cols && y < rows {
                let idx = y * cols + x;
                if idx < grid.len() && p.life > 0.0 {
                    let alpha = (p.life * 200.0) as u8;
                    grid[idx] = TerminalCell {
                        ch: p.ch,
                        fg: (p.color.0.min(alpha), p.color.1.min(alpha), p.color.2.min(alpha)),
                        bg: (0, 0, 0),
                        bold: true,
                    };
                }
            }
        }
    }
}

impl crate::interface::tui::screensaver::ScreensaverState for SimpleParticles {
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

impl crate::interface::tui::screensaver::ScreensaverEffect for SimpleParticles {
    fn init(&mut self, cols: usize, rows: usize) {
        *self = Self::new(cols, rows);
    }
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        self.update(dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        if self.active {
            SimpleParticles::draw(self, grid, cols, rows);
        }
    }
}

#[inline]
fn inv_sqrt(x: f32) -> f32 {
    x.sqrt().recip()
}

/// Gravity/particle system from rLife (rIdle-scenes).
/// Classification: Interface (TUI).
pub struct GravityParticles {
    pub particles: Vec<Particle>,
    pub gravity_centers: Vec<GravityCenter>,
    /// First-class active flag.
    active: bool,
    focused: bool,
    #[allow(dead_code)]
    rng: LcgRng,
}

#[derive(Clone, Copy, Debug)]
pub struct GravityCenter {
    pub x: f32,
    pub y: f32,
    pub mass: f32,
}

impl GravityParticles {
    pub fn new(cols: usize, rows: usize) -> Self {
        let mut rng = LcgRng::new(999);
        let count = ((cols * rows) / 4).max(10);
        let particles = (0..count).map(|_| Particle {
            x: rng.next_range(0.0, cols as f32),
            y: rng.next_range(0.0, rows as f32),
            vx: rng.next_range(-0.5, 0.5),
            vy: rng.next_range(-0.5, 0.5),
            ch: '.',
            color: (100, 150, 255),
            life: 10.0,
        }).collect();
        let gravity_centers = vec![
            GravityCenter { x: (cols as f32) * 0.5, y: (rows as f32) * 0.5, mass: 5.0 },
        ];
        Self { particles, gravity_centers, active: true, focused: true, rng }
    }

    pub fn update(&mut self, dt: f32, _cols: usize, _rows: usize) {
        if !self.active {
            return;
        }
        for p in &mut self.particles {
            for gc in &self.gravity_centers {
                let dx = gc.x - p.x;
                let dy = gc.y - p.y;
                let sq_dist = dx*dx + dy*dy;
                let inv_dist = if sq_dist <= 1.0 {
                    1.0
                } else {
                    inv_sqrt(sq_dist)
                };
                p.vx += dx * inv_dist * gc.mass * dt * 0.1;
                p.vy += dy * inv_dist * gc.mass * dt * 0.1;
            }
            p.x += p.vx * dt * 20.0;
            p.y += p.vy * dt * 20.0;
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
                        fg: p.color,
                        bg: (0, 0, 0),
                        bold: false,
                    };
                }
            }
        }
    }
}

impl crate::interface::tui::screensaver::ScreensaverState for GravityParticles {
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

impl crate::interface::tui::screensaver::ScreensaverEffect for GravityParticles {
    fn init(&mut self, cols: usize, rows: usize) {
        *self = Self::new(cols, rows);
    }
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        self.update(dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        if self.active {
            GravityParticles::draw(self, grid, cols, rows);
        }
    }
}

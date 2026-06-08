//! Retro terminal effects, games, and visual primitives.
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer) + Role (Application Software).
//!
//! # Focus & Active States
//! - **Focused**: Controls visual emphasis. Focused effects render with full brightness and detail. Unfocused effects are dimmed or de-emphasized to preserve background contrast.
//! - **Active**: Controls CPU/resource utilization. Active effects update and animate physics normally. Inactive effects pause updates and render empty cells, reducing CPU usage to zero.

pub use crate::core::{LcgRng, TerminalCell};

/// Trait representing a standard TUI-based visual effect.
/// Can be used to dynamically run or swap screensavers/effects.
pub trait TuiEffect {
    /// Update the physics / logic of the effect.
    fn update(&mut self, dt: f32, cols: usize, rows: usize);
    /// Draw the visual elements of the effect into a TerminalCell grid.
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize);
}



/// Blanket implementation: any type implementing Screensaver automatically implements TuiEffect.
impl<T: crate::interface::tui::screensaver::Screensaver> TuiEffect for T {
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        crate::interface::tui::screensaver::ScreensaverEffect::update(self, dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        // Clear grid first (centralized clear for TuiEffect consumers)
        for cell in grid.iter_mut() {
            *cell = TerminalCell::default();
        }
        crate::interface::tui::screensaver::ScreensaverEffect::draw(self, grid, cols, rows);
    }
}

/// A very basic particle for retro effects.
#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub ch: char,
    pub color: (u8, u8, u8),
    pub life: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct RainDrop {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub length: usize,
}

pub mod matrix;
pub mod particles;
pub mod rain;
pub mod fire;
pub mod logo;

pub use matrix::MatrixRain;
pub use particles::{SimpleParticles, GravityParticles, GravityCenter};
pub use rain::RainEffect;
pub use fire::FireEffect;
pub use logo::{render_logo_block, get_system_info};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::tui::screensaver::ScreensaverState;

    #[test]
    fn test_effects_active_flag_prevents_update() {
        // MatrixRain
        let mut rain = MatrixRain::new(10, 5, 0.5);
        rain.set_active(false);
        let y_positions: Vec<f32> = rain.drops.iter().map(|d| d.y).collect();
        rain.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = rain.drops.iter().map(|d| d.y).collect();
        assert_eq!(y_positions, y_positions_after);

        // FireEffect
        let mut fire = FireEffect::new(8, 4);
        fire.set_active(false);
        let y_positions: Vec<f32> = fire.particles.iter().map(|p| p.y).collect();
        fire.update(0.1, 8, 4);
        let y_positions_after: Vec<f32> = fire.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);

        // SimpleParticles
        let mut parts = SimpleParticles::new(10, 5);
        parts.set_active(false);
        let y_positions: Vec<f32> = parts.particles.iter().map(|p| p.y).collect();
        parts.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = parts.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);

        // RainEffect
        let mut rain_effect = RainEffect::new(10, 5);
        rain_effect.set_active(false);
        let y_positions: Vec<f32> = rain_effect.drops.iter().map(|d| d.y).collect();
        rain_effect.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = rain_effect.drops.iter().map(|d| d.y).collect();
        assert_eq!(y_positions, y_positions_after);

        // GravityParticles
        let mut gravity = GravityParticles::new(10, 5);
        gravity.set_active(false);
        let y_positions: Vec<f32> = gravity.particles.iter().map(|p| p.y).collect();
        gravity.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = gravity.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);
    }

    #[test]
    fn test_tui_effect_trait_active_default() {
        let rain = MatrixRain::new(5, 3, 0.3);
        assert!(rain.active());
        let fire = FireEffect::new(4, 3);
        assert!(fire.active());
        let rain_effect = RainEffect::new(5, 3);
        assert!(rain_effect.active());
        let gravity = GravityParticles::new(5, 3);
        assert!(gravity.active());
    }

    #[test]
    fn test_draw_effects_no_panic() {
        let mut grid = vec![TerminalCell::default(); 50];
        
        let mut rain = MatrixRain::new(10, 5, 0.5);
        TuiEffect::draw(&mut rain, &mut grid, 10, 5);

        let mut fire = FireEffect::new(10, 5);
        TuiEffect::draw(&mut fire, &mut grid, 10, 5);

        let mut parts = SimpleParticles::new(10, 5);
        TuiEffect::draw(&mut parts, &mut grid, 10, 5);

        let mut rain_effect = RainEffect::new(10, 5);
        TuiEffect::draw(&mut rain_effect, &mut grid, 10, 5);

        let mut gravity = GravityParticles::new(10, 5);
        TuiEffect::draw(&mut gravity, &mut grid, 10, 5);
    }

    #[test]
    fn test_effects_inactive_rendering_is_empty() {
        let cols = 10;
        let rows = 5;
        let mut grid = vec![TerminalCell::default(); cols * rows];

        macro_rules! test_inactive_draw {
            ($effect:expr) => {
                let mut eff = $effect;
                // Pre-fill grid with some non-default content
                for cell in &mut grid {
                    cell.ch = 'X';
                    cell.fg = (123, 123, 123);
                }
                eff.update(0.1, cols, rows);
                eff.set_active(false);
                TuiEffect::draw(&mut eff, &mut grid, cols, rows);
                for cell in &grid {
                    assert_eq!(cell.ch, '\0');
                    assert_eq!(cell.fg, (0, 0, 0));
                    assert_eq!(cell.bg, (0, 0, 0));
                }
            };
        }

        test_inactive_draw!(MatrixRain::new(cols, rows, 0.5));
        test_inactive_draw!(FireEffect::new(cols, rows));
        test_inactive_draw!(SimpleParticles::new(cols, rows));
        test_inactive_draw!(RainEffect::new(cols, rows));
        test_inactive_draw!(GravityParticles::new(cols, rows));
    }
}

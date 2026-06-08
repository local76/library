//! Retro terminal effects, games, and visual primitives.
//!
//! **Taxonomy Classification**: Interface (TUI / Presentation Layer) + Role (Application Software).
//!
//! # Effect Naming
//!
//! All effects follow a **Verb × Noun × Style × Palette** taxonomy:
//!
//! - **Verb**: motion type (`Falling`, `Rising`, `Flowing`, `Pulled`, `Pulsing`).
//! - **Noun**: visual unit (`Glyphs`, `Particles`, `Droplets`, `Comets`, `Blocks`, `Waves`).
//! - **Style**: render treatment (`Solid`, `Trailing`, `Flared`).
//! - **Palette**: color source (`Monochrome`, `Accent`, `Heat`).
//!
//! Type name = `Verb` + `Noun` (PascalCase). File name = snake_case of the same.
//!
//! # Focus & Active States
//! - **Focused**: Controls visual emphasis. Focused effects render with full brightness and detail. Unfocused effects are dimmed or de-emphasized to preserve background contrast.
//! - **Active**: Controls CPU/resource utilization. Active effects update and animate physics normally. Inactive effects pause updates and render empty cells, reducing CPU usage to zero.

pub use crate::core::{LcgRng, TerminalCell};
pub use crate::interface::tui::screensaver::{Screensaver, ScreensaverEffect, ScreensaverRenderer, ScreensaverState};

pub mod dimensions;

pub use dimensions::{accent_color, heat_color, resolve_color, Density, Direction, Palette, Speed, Style};

/// Trait representing a standard TUI-based visual effect.
/// Can be used to dynamically run or swap screensavers/effects.
pub trait TuiEffect {
    /// Update the physics / logic of the effect.
    fn update(&mut self, dt: f32, cols: usize, rows: usize);
    /// Draw the visual elements of the effect into a TerminalCell grid.
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize);
}

/// Blanket implementation: any type implementing Screensaver automatically implements TuiEffect.
impl<T: Screensaver> TuiEffect for T {
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        ScreensaverEffect::update(self, dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        for cell in grid.iter_mut() {
            *cell = TerminalCell::default();
        }
        ScreensaverEffect::draw(self, grid, cols, rows);
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

pub mod falling_glyphs;
pub mod flowing_particles;
pub mod pulled_particles;
pub mod falling_droplets;
pub mod rising_flames;
pub mod falling_comets;
pub mod pulsing_glyphs;
pub mod pulsing_waves;
pub mod flowing_blocks;
pub mod pulled_blocks;
pub mod rising_glyphs;
pub mod pulsing_particles;
pub mod logo;

pub use falling_glyphs::FallingGlyphs;
pub use flowing_particles::FlowingParticles;
pub use pulled_particles::{PulledParticles, GravityCenter};
pub use falling_droplets::FallingDroplets;
pub use rising_flames::RisingFlames;
pub use falling_comets::FallingComets;
pub use pulsing_glyphs::PulsingGlyphs;
pub use pulsing_waves::PulsingWaves;
pub use flowing_blocks::{FlowingBlocks, FlowingBlock, SHAPES};
pub use pulled_blocks::{PulledBlocks, BlockParticle};
pub use rising_glyphs::{RisingGlyphs, RisingGlyph};
pub use pulsing_particles::PulsingParticles;
pub use logo::{render_logo_block, get_system_info};

/// Display names for the built-in effects, in catalog order.
pub const EFFECT_NAMES: &[&str] = &[
    "Falling Glyphs",
    "Flowing Particles",
    "Pulled Particles",
    "Falling Droplets",
    "Rising Flames",
    "Falling Comets",
    "Pulsing Glyphs",
    "Pulsing Waves",
    "Flowing Blocks",
    "Pulled Blocks",
    "Rising Glyphs",
    "Pulsing Particles",
];

/// Factory to construct a Boxed Screensaver based on its index.
pub fn make_effect(
    index: usize,
    cols: usize,
    rows: usize,
) -> Box<dyn Screensaver> {
    let mut saver: Box<dyn Screensaver> = match index {
        0 => Box::new(FallingGlyphs::new(cols, rows, 0.35)),
        1 => Box::new(FlowingParticles::new(cols, rows)),
        2 => Box::new(PulledParticles::new(cols, rows)),
        3 => Box::new(FallingDroplets::new(cols, rows)),
        4 => Box::new(RisingFlames::new(cols, rows)),
        5 => Box::new(FallingComets::new(cols, rows)),
        6 => Box::new(PulsingGlyphs::new(cols, rows)),
        7 => Box::new(PulsingWaves::new(cols, rows)),
        8 => Box::new(FlowingBlocks::new(cols, rows)),
        9 => Box::new(PulledBlocks::new(cols, rows)),
        10 => Box::new(RisingGlyphs::new(cols, rows)),
        _ => Box::new(PulsingParticles::new(cols, rows)),
    };
    saver.init(cols, rows);
    saver
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_effects_active_flag_prevents_update() {
        let mut rain = FallingGlyphs::new(10, 5, 0.5);
        rain.set_active(false);
        let y_positions: Vec<f32> = rain.drops.iter().map(|d| d.y).collect();
        rain.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = rain.drops.iter().map(|d| d.y).collect();
        assert_eq!(y_positions, y_positions_after);

        let mut fire = RisingFlames::new(8, 4);
        fire.set_active(false);
        let y_positions: Vec<f32> = fire.particles.iter().map(|p| p.y).collect();
        fire.update(0.1, 8, 4);
        let y_positions_after: Vec<f32> = fire.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);

        let mut parts = FlowingParticles::new(10, 5);
        parts.set_active(false);
        let y_positions: Vec<f32> = parts.particles.iter().map(|p| p.y).collect();
        parts.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = parts.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);

        let mut rain_effect = FallingDroplets::new(10, 5);
        rain_effect.set_active(false);
        let y_positions: Vec<f32> = rain_effect.drops.iter().map(|d| d.y).collect();
        rain_effect.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = rain_effect.drops.iter().map(|d| d.y).collect();
        assert_eq!(y_positions, y_positions_after);

        let mut gravity = PulledParticles::new(10, 5);
        gravity.set_active(false);
        let y_positions: Vec<f32> = gravity.particles.iter().map(|p| p.y).collect();
        gravity.update(0.1, 10, 5);
        let y_positions_after: Vec<f32> = gravity.particles.iter().map(|p| p.y).collect();
        assert_eq!(y_positions, y_positions_after);
    }

    #[test]
    fn test_tui_effect_trait_active_default() {
        let rain = FallingGlyphs::new(5, 3, 0.3);
        assert!(rain.active());
        let fire = RisingFlames::new(4, 3);
        assert!(fire.active());
        let rain_effect = FallingDroplets::new(5, 3);
        assert!(rain_effect.active());
        let gravity = PulledParticles::new(5, 3);
        assert!(gravity.active());
    }

    #[test]
    fn test_draw_effects_no_panic() {
        let mut grid = vec![TerminalCell::default(); 50];

        let mut rain = FallingGlyphs::new(10, 5, 0.5);
        TuiEffect::draw(&mut rain, &mut grid, 10, 5);

        let mut fire = RisingFlames::new(10, 5);
        TuiEffect::draw(&mut fire, &mut grid, 10, 5);

        let mut parts = FlowingParticles::new(10, 5);
        TuiEffect::draw(&mut parts, &mut grid, 10, 5);

        let mut rain_effect = FallingDroplets::new(10, 5);
        TuiEffect::draw(&mut rain_effect, &mut grid, 10, 5);

        let mut gravity = PulledParticles::new(10, 5);
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

        test_inactive_draw!(FallingGlyphs::new(cols, rows, 0.5));
        test_inactive_draw!(RisingFlames::new(cols, rows));
        test_inactive_draw!(FlowingParticles::new(cols, rows));
        test_inactive_draw!(FallingDroplets::new(cols, rows));
        test_inactive_draw!(PulledParticles::new(cols, rows));
    }

    #[test]
    fn test_dimension_defaults() {
        assert_eq!(Style::default(), Style::Solid);
        assert_eq!(Palette::default(), Palette::Monochrome(255, 255, 255));
        let fx = FallingGlyphs::new(10, 5, 0.5);
        assert_eq!(fx.style, Style::Trailing);
        assert_eq!(fx.palette, Palette::GREEN);

        let fx = RisingFlames::new(10, 5);
        assert_eq!(fx.palette, Palette::HEAT);
    }

    #[test]
    fn test_with_style_and_palette_builders() {
        let fx = FallingGlyphs::new(10, 5, 0.5)
            .with_style(Style::Flared)
            .with_palette(Palette::Accent);
        assert_eq!(fx.style, Style::Flared);
        assert_eq!(fx.palette, Palette::Accent);
    }

    #[test]
    fn test_falling_comets_lifecycle() {
        let mut comets = FallingComets::new(20, 10);
        assert!(comets.active());
        assert_eq!(comets.palette, Palette::WHITE);
        comets.update(0.05, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        comets.draw(&mut grid, 20, 10);
        // No panic, grid was rendered. Inactive erases via screensaver logic.
        comets.set_active(false);
        comets.draw(&mut grid, 20, 10);
        // After inactive, no new cells written. Particles still may have positions.
        assert!(!comets.particles.is_empty());
    }

    #[test]
    fn test_pulsing_glyphs_lifecycle() {
        let mut glyphs = PulsingGlyphs::new(20, 10);
        assert_eq!(glyphs.palette, Palette::ACCENT);
        glyphs.update(0.05, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        glyphs.draw(&mut grid, 20, 10);
        // After update, internal time advanced (we can't read it directly, but no panic is enough)
        assert!(!glyphs.glyphs.is_empty());
    }

    #[test]
    fn test_pulsing_waves_lifecycle() {
        let mut waves = PulsingWaves::new(20, 10);
        assert_eq!(waves.palette, Palette::HEAT);
        waves.update(0.1, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        waves.draw(&mut grid, 20, 10);
        assert!(!waves.lines.is_empty());
    }

    #[test]
    fn test_flowing_blocks_lifecycle() {
        let mut blocks = FlowingBlocks::new(30, 10);
        assert_eq!(blocks.palette, Palette::ACCENT);
        assert!(!blocks.blocks.is_empty());
        blocks.update(0.05, 30, 10);
        let mut grid = vec![TerminalCell::default(); 300];
        blocks.draw(&mut grid, 30, 10);
    }

    #[test]
    fn test_pulled_blocks_lifecycle() {
        let mut blocks = PulledBlocks::new(20, 10);
        assert_eq!(blocks.palette, Palette::BLUE);
        blocks.update(0.1, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        blocks.draw(&mut grid, 20, 10);
        assert!(!blocks.particles.is_empty());
    }

    #[test]
    fn test_speed_field_and_builder() {
        let fx = FallingGlyphs::new(20, 5, 0.5)
            .with_speed(Speed::Fast);
        assert_eq!(fx.speed, Speed::Fast);
        assert_eq!(Speed::Normal.multiplier(), 1.0);
    }

    #[test]
    fn test_direction_field_and_builder() {
        let fx = FallingDroplets::new(20, 5)
            .with_direction(Direction::Up);
        assert_eq!(fx.direction, Direction::Up);
    }

    #[test]
    fn test_density_field_and_builder() {
        let fx = FlowingParticles::new(20, 5)
            .with_density(Density::Dense);
        assert_eq!(fx.density_setting, Density::Dense);
    }

    #[test]
    fn test_rising_glyphs_lifecycle() {
        let mut g = RisingGlyphs::new(20, 10);
        assert_eq!(g.palette, Palette::HEAT);
        g.update(0.05, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        g.draw(&mut grid, 20, 10);
        assert!(!g.glyphs.is_empty());
    }

    #[test]
    fn test_pulsing_particles_lifecycle() {
        let mut p = PulsingParticles::new(20, 10);
        assert_eq!(p.palette, Palette::ACCENT);
        p.update(0.1, 20, 10);
        let mut grid = vec![TerminalCell::default(); 200];
        p.draw(&mut grid, 20, 10);
        assert!(!p.particles.is_empty());
    }
}

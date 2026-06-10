use crate::core::TerminalCell;
use super::super::update::LifeEffect;
use super::super::types::to_screen;

pub fn draw_particles_and_trails(
    effect: &LifeEffect,
    grid: &mut [TerminalCell],
    cols: usize,
    rows: usize,
    dim: f32,
) {
    if dim <= 0.001 {
        return;
    }
    // Trails
    for p in &effect.particles {
        let hist_len = p.history.len();
        for (k, &(hx, hy)) in p.history.iter().enumerate() {
            let (sx, sy) = to_screen(
                hx as f32,
                hy as f32,
                effect.universe_cx,
                effect.universe_cy,
                effect.zoom,
                cols,
                rows,
            );
            if sx >= 0 && sx < cols as i32 && sy >= 0 && sy < rows as i32 {
                let idx = sy as usize * cols + sx as usize;
                if grid[idx].ch == ' ' {
                    let t = (k + 1) as f32 / (hist_len + 1) as f32;
                    let intensity = t * 0.35 * dim;
                    let tr = (p.color.0 as f32 * intensity) as u8;
                    let tg = (p.color.1 as f32 * intensity) as u8;
                    let tb = (p.color.2 as f32 * intensity) as u8;
                    grid[idx] = TerminalCell {
                        ch: '·',
                        fg: (tr, tg, tb),
                        bg: (0, 0, 0),
                        bold: false,
                    };
                }
            }
        }
    }

    // Particle Core
    for p in &effect.particles {
        let (sx, sy) = to_screen(
            p.x,
            p.y,
            effect.universe_cx,
            effect.universe_cy,
            effect.zoom,
            cols,
            rows,
        );
        if sx >= 0 && sx < cols as i32 && sy >= 0 && sy < rows as i32 {
            let idx = sy as usize * cols + sx as usize;
            if grid[idx].ch == ' ' || grid[idx].ch == '·' {
                let tr = (p.color.0 as f32 * dim) as u8;
                let tg = (p.color.1 as f32 * dim) as u8;
                let tb = (p.color.2 as f32 * dim) as u8;
                grid[idx] = TerminalCell {
                    ch: p.ch,
                    fg: (tr, tg, tb),
                    bg: (0, 0, 0),
                    bold: dim > 0.35,
                };
            }
        }
    }
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UniverseState {
    Darkness,
    BigBang,
    Expansion,
    Accretion,
    Singularity,
    Collapse,
}

pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
    pub color: (u8, u8, u8),
    pub ch: char,
    pub history: Vec<(i32, i32)>,
}

pub struct GravityCenter {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub mass: f32,
    pub color: (u8, u8, u8),
    pub active: bool,
    pub is_black_hole: bool,
    pub birth_timer: f32,
}

pub struct LogoPixel {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub origin_x: f32,
    pub origin_y: f32,
    pub ch: char,
    pub exc: f32,
    pub active: bool,
}

pub fn to_screen(ux: f32, uy: f32, universe_cx: f32, universe_cy: f32, zoom: f32, cols: usize, rows: usize) -> (i32, i32) {
    let cx = cols as f32 / 2.0;
    let cy = rows as f32 / 2.0;
    let sx = cx + (ux - universe_cx) * zoom;
    let sy = cy + (uy - universe_cy) * zoom;
    (sx.round() as i32, sy.round() as i32)
}

pub fn to_universe(sx: f32, sy: f32, universe_cx: f32, universe_cy: f32, zoom: f32, cols: usize, rows: usize) -> (f32, f32) {
    let cx = cols as f32 / 2.0;
    let cy = rows as f32 / 2.0;
    let ux = universe_cx + (sx - cx) / zoom;
    let uy = universe_cy + (sy - cy) / zoom;
    (ux, uy)
}

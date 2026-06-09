pub struct Confetti {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: (u8, u8, u8),
    pub ch: char,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub phase: f32,
    pub ch: char,
    pub excitation: f32,
    pub angle_to_disco: f32,
    pub dist_to_disco: f32,
    pub color: (u8, u8, u8),
}

pub const NEON_COLORS: &[(u8, u8, u8)] = &[
    (255, 0, 128),  // Neon Pink
    (0, 255, 255),  // Neon Cyan
    (255, 255, 0),  // Neon Yellow
    (50, 255, 50),  // Neon Green
    (180, 0, 255),  // Neon Purple
    (255, 127, 0),  // Neon Orange
];

pub const CONFETTI_CHARS: &[char] = &['*', '+', 'o', 'x', '•'];

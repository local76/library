pub struct Spark {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub max_life: f32,
}

pub struct LogoCell {
    pub x: usize,
    pub y: usize,
    pub ch: char,
    pub temp: f32,
}

pub struct Star {
    pub x: f32,
    pub y: f32,
    pub phase: f32,
    pub ch: char,
    pub excitation: f32,
    pub excited_color: (u8, u8, u8),
}

pub struct VolcanicGlob {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
}

pub fn get_palette(_accent: (u8, u8, u8)) -> [(u8, u8, u8); 36] {
    let mut palette = [(0, 0, 0); 36];
    palette[0] = (0, 0, 0);
    for (i, color) in palette.iter_mut().enumerate().skip(1) {
        if i < 12 {
            // Dark red to bright red
            let t = i as f32 / 12.0;
            *color = (
                (200.0 * t) as u8,
                0,
                0,
            );
        } else if i < 24 {
            // Bright red to vibrant orange/gold
            let t = (i - 12) as f32 / 12.0;
            *color = (
                (200.0 + 55.0 * t) as u8,
                (140.0 * t) as u8,
                0,
            );
        } else if i < 32 {
            // Orange/gold to bright yellow
            let t = (i - 24) as f32 / 8.0;
            *color = (
                255,
                (140.0 + 90.0 * t) as u8,
                (50.0 * t) as u8,
            );
        } else {
            // Bright yellow to white-hot
            let t = (i - 32) as f32 / 3.0;
            *color = (
                255,
                (230.0 + 25.0 * t) as u8,
                (50.0 + 190.0 * t) as u8,
            );
        }
    }
    palette
}

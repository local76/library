use crate::core::{LcgRng, TerminalCell};
use super::RainDrop;

/// Simple Matrix-style rain effect as an example of a reusable effect primitive.
/// Renders into a slice of TerminalCell (caller provides the grid).
pub struct MatrixRain {
    pub drops: Vec<RainDrop>,
    pub char_pool: Vec<char>,
    pub density: f32,
    /// First-class active flag for focus/tab UIs. When false, update is a no-op
    /// and draw clears the grid (effect looks paused/dimmed).
    active: bool,
    focused: bool,
    rng: LcgRng,
}

impl MatrixRain {
    pub fn new(cols: usize, rows: usize, density: f32) -> Self {
        let mut rng = LcgRng::new(0xC0FFEE);
        let mut drops = Vec::new();
        let num_drops = ((cols as f32) * density).max(1.0) as usize;

        for _ in 0..num_drops {
            drops.push(RainDrop {
                x: rng.next_usize(cols) as f32,
                y: rng.next_usize(rows) as f32,
                speed: rng.next_range(0.5, 2.5),
                length: rng.next_usize(8) + 3,
            });
        }

        // Classic katakana + digits for the "data rain"
        let char_pool: Vec<char> = "ｦｧｨｩｪｫｬｭｮｯｰｱｲｳｴｵｶｷｸｹｺｻｼｽｾｿﾀﾁﾂﾃﾄﾅﾆﾇﾈﾉﾊﾋﾌﾍﾎﾏﾐﾑﾒﾓﾔﾕﾖﾗﾙﾚﾛﾜﾝ0123456789".chars().collect();

        Self {
            drops,
            char_pool,
            density,
            active: true,
            focused: true,
            rng,
        }
    }

    pub fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        if !self.active {
            return;
        }
        for drop in &mut self.drops {
            drop.y += drop.speed * dt * 20.0;
            if drop.y > rows as f32 + drop.length as f32 {
                drop.y = - (drop.length as f32);
                drop.x = self.rng.next_usize(cols) as f32;
                drop.speed = self.rng.next_range(0.5, 2.5);
            }
        }
    }

    /// Render the rain into the provided grid (column-major or row-major as per caller convention).
    /// Simple implementation: assumes grid is rows * cols in row-major order.
    pub fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        if !self.active {
            return;
        }

        for drop in &self.drops {
            for i in 0..drop.length {
                let y = (drop.y - i as f32) as isize;
                if y >= 0 && y < rows as isize {
                    let x = drop.x as usize;
                    if x < cols {
                        let idx = (y as usize) * cols + x;
                        if idx < grid.len() {
                            let ch = if i == 0 {
                                '█'
                            } else {
                                let pool_idx = self.rng.next_usize(self.char_pool.len());
                                self.char_pool[pool_idx]
                            };
                            let intensity = 255 - (i as u8 * 20).min(200);
                            grid[idx] = TerminalCell {
                                ch,
                                fg: (0, intensity, 80),
                                bg: (0, 0, 0),
                                bold: i < 3,
                            };
                        }
                    }
                }
            }
        }
    }
}

impl crate::interface::tui::screensaver::ScreensaverState for MatrixRain {
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

impl crate::interface::tui::screensaver::ScreensaverEffect for MatrixRain {
    fn init(&mut self, cols: usize, rows: usize) {
        *self = Self::new(cols, rows, self.density);
    }
    fn update(&mut self, dt: f32, cols: usize, rows: usize) {
        self.update(dt, cols, rows);
    }
    fn draw(&mut self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        MatrixRain::draw(self, grid, cols, rows);
    }
}

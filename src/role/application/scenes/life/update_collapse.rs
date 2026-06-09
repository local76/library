use super::effect::LifeEffect;
use super::types::{Particle, to_screen};

pub fn update_singularity(eff: &mut LifeEffect, delta: f32, cols: usize, rows: usize) {
    let cols_f = cols as f32;
    let rows_f = rows as f32;
    let cx = cols_f / 2.0;
    let cy = rows_f / 2.0;
    let dir = if eff.spin_clockwise { 1.0f32 } else { -1.0f32 };

    if !eff.seeds.is_empty() {
        let seed = &mut eff.seeds[0];
        let dx_c = cx - seed.x;
        let dy_c = cy - seed.y;
        let dist_c = (dx_c * dx_c + dy_c * dy_c).sqrt().max(0.1);
        
        let mut sfx = (dx_c / dist_c) * 1.8;
        let mut sfy = (dy_c / dist_c) * 0.9;
        
        let orbit_force = 1.8f32;
        sfx += (dy_c / dist_c) * orbit_force * dir;
        sfy += (-dx_c / dist_c) * orbit_force * 0.45 * dir;
        
        seed.vx += sfx * delta;
        seed.vy += sfy * delta;
        seed.vx *= 1.0 - (delta * 0.15);
        seed.vy *= 1.0 - (delta * 0.15);
        
        seed.x += seed.vx * delta;
        seed.y += seed.vy * delta;
    }
    
    let (bh_x, bh_y) = if !eff.seeds.is_empty() {
        (eff.seeds[0].x, eff.seeds[0].y)
    } else {
        (cx, cy)
    };

    for p in &mut eff.particles {
        let dx = bh_x - p.x;
        let dy = bh_y - p.y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.1);
        
        let pull = 110.0 / (dist + 2.0);
        let tangent = 45.0 / (dist.sqrt() + 1.0);

        p.vx += ((dx / dist) * pull + (dy / dist) * tangent * dir) * delta;
        p.vy += ((dy / dist) * pull * 0.45 - (dx / dist) * tangent * 0.45 * dir) * delta;

        p.vx *= 1.0 - (delta * 1.5);
        p.vy *= 1.0 - (delta * 1.5);

        p.x += p.vx * delta;
        p.y += p.vy * delta;

        let cx_i = p.x.round() as i32;
        let cy_i = p.y.round() as i32;
        p.history.push((cx_i, cy_i));
        if p.history.len() > 4 { p.history.remove(0); }
    }

    eff.particles.retain(|p| {
        let dx = bh_x - p.x;
        let dy = bh_y - p.y;
        dx * dx + dy * dy > 2.0
    });

    if eff.particles.len() < 200 && !eff.seeds.is_empty() && eff.rng.next_bool(0.15) {
        let seed = &eff.seeds[0];
        let dist = eff.rng.next_range(3.2, 7.5);
        let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
        let px = seed.x + angle.cos() * dist;
        let py = seed.y + angle.sin() * dist * 0.45;
        
        let speed = (seed.mass * 18.0 / dist).sqrt();
        let tx = -angle.sin();
        let ty = angle.cos();
        
        let vx = tx * speed * dir;
        let vy = ty * speed * 0.45 * dir;
        
        eff.particles.push(Particle {
            x: px,
            y: py,
            vx,
            vy,
            mass: eff.rng.next_range(0.4, 0.8),
            color: (160, 80, 255),
            ch: if eff.rng.next_bool(0.5) { '·' } else { '.' },
            history: Vec::new(),
        });
    }

    for lp in &mut eff.logo_pixels {
        if !lp.active { continue; }
        let (bh_sx, bh_sy) = to_screen(bh_x, bh_y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
        let dx = bh_sx as f32 - lp.x;
        let dy = bh_sy as f32 - lp.y;
        let dist = (dx*dx + dy*dy).sqrt().max(0.1);
        
        lp.exc = (1.0 - dist / 12.0).clamp(0.0, 1.0);
        
        if dist > 1.6 {
            let pull = 18.0 / (dist + 2.0);
            let tangent = 12.0 / (dist.sqrt() + 1.0);
            
            lp.vx += ((dx / dist) * pull + (dy / dist) * tangent * dir) * 4.5 * delta;
            lp.vy += ((dy / dist) * pull - (dx / dist) * tangent * 0.45 * dir) * 4.5 * delta;
            
            lp.vx *= 1.0 - (1.5 * delta);
            lp.vy *= 1.0 - (1.5 * delta);
            
            lp.x += lp.vx * delta;
            lp.y += lp.vy * delta;
        } else {
            lp.active = false;
        }
    }
}

pub fn update_collapse(eff: &mut LifeEffect, delta: f32, cols: usize, rows: usize) {
    let cols_f = cols as f32;
    let rows_f = rows as f32;
    let cx = cols_f / 2.0;
    let cy = rows_f / 2.0;

    if !eff.seeds.is_empty() {
        let seed = &mut eff.seeds[0];
        seed.x += (cx - seed.x) * 4.0 * delta;
        seed.y += (cy - seed.y) * 4.0 * delta;
    }

    let bh_x = if !eff.seeds.is_empty() { eff.seeds[0].x } else { cx };
    let bh_y = if !eff.seeds.is_empty() { eff.seeds[0].y } else { cy };
    let dir = if eff.spin_clockwise { 1.0f32 } else { -1.0f32 };
    
    for p in &mut eff.particles {
        let dx = bh_x - p.x;
        let dy = bh_y - p.y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.1);
        
        let pull = (220.0 + eff.state_timer * 120.0) / (dist + 1.0);
        let tangent = (60.0 - eff.state_timer * 18.0).max(0.0) / (dist.sqrt() + 1.0);
        
        p.vx += ((dx / dist) * pull + (dy / dist) * tangent * dir) * delta;
        p.vy += ((dy / dist) * pull * 0.45 - (dx / dist) * tangent * 0.45 * dir) * delta;
        
        let drag = 2.5 + eff.state_timer * 2.0;
        p.vx *= 1.0 - (delta * drag);
        p.vy *= 1.0 - (delta * drag);
        
        p.x += p.vx * delta;
        p.y += p.vy * delta;
        
        let cx_i = p.x.round() as i32;
        let cy_i = p.y.round() as i32;
        p.history.push((cx_i, cy_i));
        if p.history.len() > 4 { p.history.remove(0); }
    }
    
    eff.particles.retain(|p| {
        let dx = bh_x - p.x;
        let dy = bh_y - p.y;
        dx * dx + dy * dy > 1.5
    });
    
    for lp in &mut eff.logo_pixels {
        if !lp.active { continue; }
        let (bh_sx, bh_sy) = to_screen(bh_x, bh_y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
        let dx = bh_sx as f32 - lp.x;
        let dy = bh_sy as f32 - lp.y;
        let dist = (dx*dx + dy*dy).sqrt().max(0.1);
        
        lp.exc = (1.0 - dist / 8.0).clamp(0.0, 1.0);
        
        if dist > 1.2 {
            let pull = (45.0 + eff.state_timer * 25.0) / (dist + 1.0);
            let tangent = (16.0 - eff.state_timer * 4.0).max(0.0) / (dist.sqrt() + 1.0);
            
            lp.vx += ((dx / dist) * pull + (dy / dist) * tangent * dir) * 6.0 * delta;
            lp.vy += ((dy / dist) * pull - (dx / dist) * tangent * 0.45 * dir) * 6.0 * delta;
            
            let drag = 1.5 + eff.state_timer * 1.5;
            lp.vx *= 1.0 - (drag * delta);
            lp.vy *= 1.0 - (drag * delta);
            
            lp.x += lp.vx * delta;
            lp.y += lp.vy * delta;
        } else {
            lp.active = false;
        }
    }
}

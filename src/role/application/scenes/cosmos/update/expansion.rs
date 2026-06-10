use crate::role::application::palette::query_current_palette;
use super::LifeEffect;
use super::super::types::{Particle, to_universe};
use super::accretion_helpers::{handle_seed_merges, handle_logo_character_drift, handle_nebular_stellar_ignition};

pub fn update_expansion(eff: &mut LifeEffect, delta: f32, _cols: usize, _rows: usize) {
    let dir = if eff.spin_clockwise { 1.0f32 } else { -1.0f32 };
    let progress = (eff.state_timer / 7.0).min(1.0);
    
    for p in &mut eff.particles {
        let dx = eff.universe_cx - p.x;
        let dy = eff.universe_cy - p.y;
        let dist = (dx * dx + dy * dy).sqrt().max(0.1);
        
        let pull = 10.0 * progress / (dist + 6.0);
        let tangent = 15.0 * progress / (dist.sqrt() + 2.0);
        
        p.vx += ((dx / dist) * pull + (dy / dist) * tangent * dir) * delta;
        p.vy += ((dy / dist) * pull * 0.45 - (dx / dist) * tangent * 0.45 * dir) * delta;

        p.vx *= 1.0 - (delta * 0.35);
        p.vy *= 1.0 - (delta * 0.35);
        p.x += p.vx * delta;
        p.y += p.vy * delta;

        let cx_i = p.x.round() as i32;
        let cy_i = p.y.round() as i32;
        p.history.push((cx_i, cy_i));
        if p.history.len() > 4 { p.history.remove(0); }
    }

    let progress = (eff.state_timer / 8.0).min(1.0);
    let k = 0.5 + progress * 5.5;
    let drag = 1.0 + progress * 2.0;
    
    for lp in &mut eff.logo_pixels {
        if lp.active {
            let dx = lp.origin_x - lp.x;
            let dy = lp.origin_y - lp.y;
            
            lp.vx += dx * k * delta;
            lp.vy += dy * k * delta;
            
            lp.vx *= 1.0 - (drag * delta);
            lp.vy *= 1.0 - (drag * delta);
            
            lp.x += lp.vx * delta;
            lp.y += lp.vy * delta;

            lp.exc = (lp.exc - 0.4 * delta).max(0.0);
        }
    }
}

pub fn update_accretion(eff: &mut LifeEffect, delta: f32, cols: usize, rows: usize) {
    let dir = if eff.spin_clockwise { 1.0f32 } else { -1.0f32 };
    let seeds_len = eff.seeds.len();
    let cx = cols as f32 / 2.0;
    let cy = rows as f32 / 2.0;
    
    // Gravity centers drift
    for i in 0..seeds_len {
        if !eff.seeds[i].active { continue; }
        let mut sfx = 0.0f32;
        let mut sfy = 0.0f32;
        for j in 0..seeds_len {
            if i == j || !eff.seeds[j].active { continue; }
            let dx = eff.seeds[j].x - eff.seeds[i].x;
            let dy = eff.seeds[j].y - eff.seeds[i].y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt().max(0.1);
            let force = (eff.seeds[j].mass * 12.0) / (dist_sq + 15.0);
            sfx += (dx / dist) * force;
            sfy += (dy / dist) * force;
        }
        let dx_c = cx - eff.seeds[i].x;
        let dy_c = cy - eff.seeds[i].y;
        let dist_c = (dx_c * dx_c + dy_c * dy_c).sqrt().max(0.1);
        sfx += (dx_c / dist_c) * 1.8;
        sfy += (dy_c / dist_c) * 0.9;

        let orbit_force = 3.5f32 * (1.0 - (eff.state_timer / 15.0).min(0.85));
        sfx += (dy_c / dist_c) * orbit_force * dir;
        sfy += (-dx_c / dist_c) * orbit_force * 0.45 * dir;
        for lp in &eff.logo_pixels {
            if !lp.active { continue; }
            let (lp_ux, lp_uy) = to_universe(lp.x, lp.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
            let dx = lp_ux - eff.seeds[i].x;
            let dy = lp_uy - eff.seeds[i].y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt().max(0.1);
            let force = 0.50 / (dist_sq + 8.0);
            sfx += (dx / dist) * force;
            sfy += (dy / dist) * force;
        }

        eff.seeds[i].vx += sfx * delta;
        eff.seeds[i].vy += sfy * delta;
        eff.seeds[i].vx *= 1.0 - (delta * 0.25);
        eff.seeds[i].vy *= 1.0 - (delta * 0.25);

        eff.seeds[i].x += eff.seeds[i].vx * delta;
        eff.seeds[i].y += eff.seeds[i].vy * delta;
    }

    // Proximity shatters
    let mut seed_explosions = Vec::new();
    for seed in &mut eff.seeds {
        if !seed.active || seed.is_black_hole { continue; }
        
        for lp in &mut eff.logo_pixels {
            if !lp.active { continue; }
            
            let (lp_ux, lp_uy) = to_universe(lp.x, lp.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
            let dx = lp_ux - seed.x;
            let dy = lp_uy - seed.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < 2.0 {
                seed.active = false;
                lp.exc = 1.0;
                seed_explosions.push((seed.x, seed.y, seed.color, seed.vx, seed.vy));
                break;
            }
        }
    }

    for (sx, sy, color, _vx, _vy) in seed_explosions {
        let ox = sx - eff.universe_cx;
        let oy = sy - eff.universe_cy;
        let o_len = (ox * ox + oy * oy).sqrt().max(0.1);
        let dir_x = ox / o_len;
        let dir_y = oy / o_len;

        let count = 40;
        for _ in 0..count {
            let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
            let speed = eff.rng.next_range(60.0, 110.0);
            eff.particles.push(Particle {
                x: sx,
                y: sy,
                vx: (dir_x * 0.8 + angle.cos() * 0.2) * speed,
                vy: (dir_y * 0.8 + angle.sin() * 0.2) * speed * 0.45,
                mass: 0.6,
                color: if eff.rng.next_bool(0.3) { (255, 255, 255) } else { color },
                ch: if eff.rng.next_bool(0.4) { '*' } else if eff.rng.next_bool(0.5) { '+' } else { '·' },
                history: Vec::new(),
            });
        }
    }

    // Star merge check
    handle_seed_merges(eff, delta, dir, seeds_len);

    // Particles gravitate to active seeds
    for p in &mut eff.particles {
        let mut fx = 0.0f32;
        let mut fy = 0.0f32;
        for seed in &eff.seeds {
            if !seed.active { continue; }
            let dx = seed.x - p.x;
            let dy = seed.y - p.y;
            let dist_sq = dx * dx + dy * dy;
            let dist = dist_sq.sqrt().max(0.1);
            
            let mass_multiplier = if seed.is_black_hole { 1.8 } else { 1.0 };
            let force = (seed.mass * 22.0 * mass_multiplier) / (dist_sq + 18.0);
            fx += (dx / dist) * force;
            fy += (dy / dist) * force;
        }

        for lp in &eff.logo_pixels {
            if !lp.active { continue; }
            let (lp_ux, lp_uy) = to_universe(lp.x, lp.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
            let dx = lp_ux - p.x;
            let dy = lp_uy - p.y;
            let dist_sq = dx * dx + dy * dy;
            if dist_sq < 256.0 {
                let dist = dist_sq.sqrt().max(0.1);
                let force = 0.45 / (dist_sq + 5.0);
                fx += (dx / dist) * force;
                fy += (dy / dist) * force;
            }
        }

        p.vx += (fx * delta) / p.mass;
        p.vy += (fy * delta) / p.mass;
        
        p.vx *= 1.0 - (delta * 0.40);
        p.vy *= 1.0 - (delta * 0.40);

        p.x += p.vx * delta;
        p.y += p.vy * delta;

        let cx_i = p.x.round() as i32;
        let cy_i = p.y.round() as i32;
        p.history.push((cx_i, cy_i));
        if p.history.len() > 4 { p.history.remove(0); }
    }

    // Accrete particles
    let mut new_sparks = Vec::new();
    let current_total_particles = eff.particles.len();
    let palette = query_current_palette();
    let accent = palette.accent;
    eff.particles.retain_mut(|p| {
        for lp in &eff.logo_pixels {
            if lp.active {
                let (lp_ux, lp_uy) = to_universe(lp.x, lp.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
                let dx = lp_ux - p.x;
                let dy = lp_uy - p.y;
                if dx * dx + dy * dy < 1.44 {
                    if current_total_particles + new_sparks.len() < 400 {
                        let ox = p.x - eff.universe_cx;
                        let oy = p.y - eff.universe_cy;
                        let o_len = (ox * ox + oy * oy).sqrt().max(0.1);
                        let dir_x = ox / o_len;
                        let dir_y = oy / o_len;

                        let spark_count = eff.rng.next_range(2.0, 4.0) as usize;
                        for _ in 0..spark_count {
                            let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                            let speed = eff.rng.next_range(50.0, 95.0);
                            new_sparks.push(Particle {
                                x: p.x,
                                y: p.y,
                                vx: (dir_x * 0.75 + angle.cos() * 0.25) * speed,
                                vy: (dir_y * 0.75 + angle.sin() * 0.25) * speed * 0.45,
                                mass: 0.5,
                                color: (accent.0.saturating_add(60), accent.1.saturating_add(60), 255),
                                ch: if eff.rng.next_bool(0.5) { '*' } else { '+' },
                                history: Vec::new(),
                            });
                        }
                    }
                    return false;
                }
            }
        }

        for seed in &mut eff.seeds {
            if seed.active {
                let dx = seed.x - p.x;
                let dy = seed.y - p.y;
                let dist_sq = dx * dx + dy * dy;
                if seed.is_black_hole {
                    if dist_sq < 2.25 {
                        if current_total_particles + new_sparks.len() < 350 {
                            let spark_count = eff.rng.next_range(2.0, 4.0) as usize;
                            for _ in 0..spark_count {
                                let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                                let speed = eff.rng.next_range(12.0, 24.0);
                                new_sparks.push(Particle {
                                    x: seed.x + angle.cos() * 1.6,
                                    y: seed.y + angle.sin() * 1.6 * 0.45,
                                    vx: angle.cos() * speed,
                                    vy: angle.sin() * speed * 0.45,
                                    mass: 0.5,
                                    color: (180, 100, 255),
                                    ch: if eff.rng.next_bool(0.5) { '+' } else { '·' },
                                    history: Vec::new(),
                                });
                            }
                        }
                        return false;
                    }
                } else {
                    if dist_sq < 1.44 {
                        seed.mass += 0.08;
                        if eff.rng.next_bool(0.4) && current_total_particles + new_sparks.len() < 350 {
                            let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                            let speed = eff.rng.next_range(8.0, 18.0);
                            new_sparks.push(Particle {
                                x: seed.x + angle.cos() * 1.3,
                                y: seed.y + angle.sin() * 1.3 * 0.45,
                                vx: angle.cos() * speed,
                                vy: angle.sin() * speed * 0.45,
                                mass: 0.4,
                                color: (255, 230, 150),
                                ch: '·',
                                history: Vec::new(),
                            });
                        }
                        return false;
                    }
                }
            }
        }
        true
    });
    eff.particles.extend(new_sparks);

    // Spawn orbital replenishment particles around black holes
    if eff.particles.len() < 250 {
        for seed in &eff.seeds {
            if seed.active && seed.is_black_hole && eff.rng.next_bool(0.12) {
                let dist = eff.rng.next_range(3.2, 7.5);
                let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                let px = seed.x + angle.cos() * dist;
                let py = seed.y + angle.sin() * dist * 0.45;
                
                let speed = (seed.mass * 12.0 / dist).sqrt();
                let tx = -angle.sin();
                let ty = angle.cos();
                
                let vx = tx * speed * dir;
                let vy = ty * speed * 0.45 * dir;
                
                let p_color = (
                    (seed.color.0 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                    (seed.color.1 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                    (seed.color.2 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                );
                eff.particles.push(Particle {
                    x: px,
                    y: py,
                    vx,
                    vy,
                    mass: eff.rng.next_range(0.4, 0.8),
                    color: p_color,
                    ch: if eff.rng.next_bool(0.5) { '·' } else { '.' },
                    history: Vec::new(),
                });
            }
        }
    }

    // Logo character drift
    handle_logo_character_drift(eff, delta, dir, cols, rows);

    // Stellar Ignition
    handle_nebular_stellar_ignition(eff, dir);
}


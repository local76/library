use crate::role::application::palette::query_current_palette;
use super::LifeEffect;
use super::super::types::{Particle, GravityCenter, to_screen};

pub fn handle_seed_merges(eff: &mut LifeEffect, _delta: f32, _dir: f32, seeds_len: usize) {
    let mut spawn_sparks = Vec::new();
    for i in 0..seeds_len {
        if !eff.seeds[i].active { continue; }
        for j in (i+1)..seeds_len {
            if !eff.seeds[j].active { continue; }
            let dx = eff.seeds[j].x - eff.seeds[i].x;
            let dy = eff.seeds[j].y - eff.seeds[i].y;
            let merge_dist = 3.5 + (eff.seeds[i].mass + eff.seeds[j].mass) * 0.12;
            if dx*dx + dy*dy < merge_dist * merge_dist {
                // Merge j into i
                let m_i = eff.seeds[i].mass;
                let m_j = eff.seeds[j].mass;
                let new_mass = m_i + m_j;
                eff.seeds[i].x = (eff.seeds[i].x * m_i + eff.seeds[j].x * m_j) / new_mass;
                eff.seeds[i].y = (eff.seeds[i].y * m_i + eff.seeds[j].y * m_j) / new_mass;
                eff.seeds[i].vx = (eff.seeds[i].vx * m_i + eff.seeds[j].vx * m_j) / new_mass;
                eff.seeds[i].vy = (eff.seeds[i].vy * m_i + eff.seeds[j].vy * m_j) / new_mass;
                eff.seeds[i].mass = new_mass;
                eff.seeds[j].active = false;

                let was_i_bh = eff.seeds[i].is_black_hole;
                let was_j_bh = eff.seeds[j].is_black_hole;
                
                let merger_type = if was_i_bh && was_j_bh {
                    3
                } else if was_i_bh || was_j_bh {
                    2
                } else if new_mass >= 8.5 {
                    1
                } else {
                    0
                };
                
                let c_i = eff.seeds[i].color;
                let c_j = eff.seeds[j].color;
                let blended_color = (
                    (((c_i.0 as u16 + c_j.0 as u16) / 2) as u8),
                    (((c_i.1 as u16 + c_j.1 as u16) / 2) as u8),
                    (((c_i.2 as u16 + c_j.2 as u16) / 2) as u8),
                );

                spawn_sparks.push((eff.seeds[i].x, eff.seeds[i].y, merger_type, blended_color));

                if merger_type == 1 || merger_type == 2 || merger_type == 3 {
                    eff.seeds[i].is_black_hole = true;
                    eff.seeds[i].color = (130, 50, 240);
                    eff.seeds[i].birth_timer = 0.0;
                }
            }
        }
    }

    for (sx, sy, merger_type, color) in spawn_sparks {
        if merger_type == 3 {
            eff.grav_wave_timer = 1.2;
            eff.grav_wave_cx = sx;
            eff.grav_wave_cy = sy;
        }
        match merger_type {
            0 => {
                let count = 25;
                for _ in 0..count {
                    let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                    let speed = eff.rng.next_range(15.0, 32.0);
                    eff.particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed * 0.45,
                        mass: 0.6,
                        color: (255, 235, 180),
                        ch: if eff.rng.next_bool(0.5) { '*' } else { '+' },
                        history: Vec::new(),
                    });
                }
            }
            1 => {
                for p in &mut eff.particles {
                    let dx = p.x - sx;
                    let dy = p.y - sy;
                    let dist_sq = dx * dx + dy * dy;
                    let dist = dist_sq.sqrt().max(0.1);
                    if dist < 22.0 {
                        let push = (22.0 - dist) * 5.0;
                        p.vx += (dx / dist) * push;
                        p.vy += (dy / dist) * push * 0.45;
                    }
                }
                for _ in 0..50 {
                    let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                    let speed = eff.rng.next_range(25.0, 48.0);
                    eff.particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed * 0.45,
                        mass: 0.8,
                        color: (255, 120, 50),
                        ch: '░',
                        history: Vec::new(),
                    });
                }
                for _ in 0..25 {
                    let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                    let speed = eff.rng.next_range(15.0, 30.0);
                    eff.particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed * 0.45,
                        mass: 0.5,
                        color: (255, 255, 255),
                        ch: '*',
                        history: Vec::new(),
                    });
                }
            }
            2 => {
                let count = 40;
                let flare_color = (
                    color.0.saturating_add(60),
                    color.1.saturating_add(60),
                    255
                );
                for _ in 0..count {
                    let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                    let speed = eff.rng.next_range(20.0, 40.0);
                    eff.particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed * 0.45,
                        mass: 0.5,
                        color: flare_color,
                        ch: if eff.rng.next_bool(0.5) { '+' } else { '·' },
                        history: Vec::new(),
                    });
                }
            }
            3 => {
                for p in &mut eff.particles {
                    let dx = p.x - sx;
                    let dy = p.y - sy;
                    let dist_sq = dx * dx + dy * dy;
                    let dist = dist_sq.sqrt().max(0.1);
                    if dist < 32.0 {
                        let push = (32.0 - dist) * 7.5;
                        p.vx += (dx / dist) * push;
                        p.vy += (dy / dist) * push * 0.45;
                    }
                }
                for _ in 0..65 {
                    let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                    let speed = eff.rng.next_range(35.0, 65.0);
                    eff.particles.push(Particle {
                        x: sx,
                        y: sy,
                        vx: angle.cos() * speed,
                        vy: angle.sin() * speed * 0.45,
                        mass: 0.7,
                        color: (160, 80, 255),
                        ch: if eff.rng.next_bool(0.4) { '╬' } else if eff.rng.next_bool(0.5) { '═' } else { '─' },
                        history: Vec::new(),
                    });
                }
            }
            _ => {}
        }
    }
}

pub fn handle_logo_character_drift(eff: &mut LifeEffect, delta: f32, dir: f32, cols: usize, rows: usize) {
    let palette = query_current_palette();
    let accent = palette.accent;
    let mut spawned_logo_fragments = Vec::new();

    for lp in &mut eff.logo_pixels {
        if !lp.active { continue; }
        lp.exc = (lp.exc - 1.2 * delta).max(0.0);
        for p in &eff.particles {
            let (p_sx, p_sy) = to_screen(p.x, p.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
            let dx = p_sx as f32 - lp.x;
            let dy = (p_sy as f32 - lp.y) * 2.0;
            if dx*dx + dy*dy < 4.0 {
                lp.exc = 1.0;
            }
        }

        let mut total_bh_weight = 0.0f32;
        let mut fx_bh = 0.0f32;
        let mut fy_bh = 0.0f32;

        for seed in &eff.seeds {
            if seed.active && seed.is_black_hole {
                let (bh_sx, bh_sy) = to_screen(seed.x, seed.y, eff.universe_cx, eff.universe_cy, eff.zoom, cols, rows);
                let dx = bh_sx as f32 - lp.x;
                let dy = bh_sy as f32 - lp.y;
                let dist_sq = dx * dx + dy * dy;
                let dist = dist_sq.sqrt().max(0.1);
                
                if dist < 12.0 {
                    lp.exc = 1.0;
                    
                    if dist > 1.8 {
                        let weight = 1.0 - (dist / 12.0);
                        total_bh_weight = total_bh_weight.max(weight);
                        
                        let pull = (seed.mass * 18.0) / (dist_sq + 6.0);
                        let tangent = (seed.mass * 12.0) / (dist.sqrt() + 2.0);
                        fx_bh += ((dx / dist) * pull + (dy / dist) * tangent * dir) * weight;
                        fy_bh += ((dy / dist) * pull - (dx / dist) * tangent * 0.45 * dir) * weight;
                    } else {
                        lp.active = false;
                        
                        for _ in 0..10 {
                            let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                            let speed = eff.rng.next_range(16.0, 32.0);
                            spawned_logo_fragments.push(Particle {
                                x: seed.x,
                                y: seed.y,
                                vx: angle.cos() * speed,
                                vy: angle.sin() * speed * 0.45,
                                mass: 0.5,
                                color: (
                                    (accent.0 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                                    (accent.1 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                                    (accent.2 as i16 + eff.rng.next_range(-20.0, 20.0) as i16).clamp(0, 255) as u8,
                                ),
                                ch: lp.ch,
                                history: Vec::new(),
                            });
                        }
                    }
                }
            }
        }

        let dx_spring = lp.origin_x - lp.x;
        let dy_spring = lp.origin_y - lp.y;
        let k = 5.0;
        
        let spring_weight = 1.0 - total_bh_weight;
        let fx_spring = dx_spring * k * spring_weight;
        let fy_spring = dy_spring * k * spring_weight;

        lp.vx += (fx_spring + fx_bh) * delta;
        lp.vy += (fy_spring + fy_bh) * delta;

        let drag = 2.0;
        lp.vx *= 1.0 - (drag * delta);
        lp.vy *= 1.0 - (drag * delta);
        
        lp.x += lp.vx * delta;
        lp.y += lp.vy * delta;
    }
    eff.particles.extend(spawned_logo_fragments);
}

pub fn handle_nebular_stellar_ignition(eff: &mut LifeEffect, dir: f32) {
    if eff.state_timer > 1.0 && eff.particles.len() > 40 && eff.rng.next_bool(0.10) {
        let p_idx = eff.rng.next_range(0.0, eff.particles.len() as f32) as usize;
        let target_x = eff.particles[p_idx].x;
        let target_y = eff.particles[p_idx].y;
        
        let mut neighbors = Vec::new();
        for k in 0..eff.particles.len() {
            let dx = eff.particles[k].x - target_x;
            let dy = eff.particles[k].y - target_y;
            if dx * dx + dy * dy < 20.25 {
                neighbors.push(k);
            }
        }
        
        if neighbors.len() >= 12 {
            let mut sum_x = 0.0f32;
            let mut sum_y = 0.0f32;
            let mut sum_vx = 0.0f32;
            let mut sum_vy = 0.0f32;
            let mut sum_r = 0u32;
            let mut sum_g = 0u32;
            let mut sum_b = 0u32;
            
            for &idx in &neighbors {
                let p = &eff.particles[idx];
                sum_x += p.x;
                sum_y += p.y;
                sum_vx += p.vx;
                sum_vy += p.vy;
                sum_r += p.color.0 as u32;
                sum_g += p.color.1 as u32;
                sum_b += p.color.2 as u32;
            }
            
            let count_f = neighbors.len() as f32;
            let avg_x = sum_x / count_f;
            let avg_y = sum_y / count_f;
            let avg_vx = sum_vx / count_f;
            let avg_vy = sum_vy / count_f;
            let avg_color = (
                (sum_r / neighbors.len() as u32) as u8,
                (sum_g / neighbors.len() as u32) as u8,
                (sum_b / neighbors.len() as u32) as u8,
            );
            
            let dx = avg_x - eff.universe_cx;
            let dy = avg_y - eff.universe_cy;
            let dist = (dx * dx + dy * dy).sqrt().max(0.1);
            
            let tx = -dy / dist;
            let ty = dx / dist;
            
            let orbit_speed = (180.0 / dist).sqrt().clamp(4.0, 18.0);
            let orb_vx = tx * orbit_speed * dir;
            let orb_vy = ty * orbit_speed * 0.45 * dir;
            
            let new_vx = avg_vx * 0.2 + orb_vx * 0.8;
            let new_vy = avg_vy * 0.2 + orb_vy * 0.8;

            let new_star = GravityCenter {
                x: avg_x,
                y: avg_y,
                vx: new_vx,
                vy: new_vy,
                mass: (count_f * 0.35).clamp(1.5, 6.0),
                color: avg_color,
                active: true,
                is_black_hole: false,
                birth_timer: 0.0,
            };
            eff.seeds.push(new_star);
            
            let mut to_remove = vec![false; eff.particles.len()];
            for &idx in &neighbors {
                to_remove[idx] = true;
            }
            
            let mut i = 0;
            eff.particles.retain(|_| {
                let keep = !to_remove[i];
                i += 1;
                keep
            });
            
            let spark_color = (avg_color.0.saturating_add(80), avg_color.1.saturating_add(80), 255);
            for _ in 0..15 {
                let angle = eff.rng.next_range(0.0, std::f32::consts::TAU);
                let speed = eff.rng.next_range(12.0, 24.0);
                eff.particles.push(Particle {
                    x: avg_x,
                    y: avg_y,
                    vx: avg_vx + angle.cos() * speed,
                    vy: avg_vy + angle.sin() * speed * 0.45,
                    mass: 0.5,
                    color: spark_color,
                    ch: '+',
                    history: Vec::new(),
                });
            }
        }
    }
}

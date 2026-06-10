//! RGB controller and SDK server command routing.
//!
//! **Taxonomy Classification**: System Role (Purpose - Application Software).

use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
use std::io::Write;

use super::rgb_protocol::{RgbColor, OpenRGBClient, find_restore_profile, OpenRGBConfig, OPENRGB_MAGIC, CMD_LOAD_PROFILE, HEADER_SIZE};

pub enum RgbCommand {
    SetColor(RgbColor),
    SetDeviceColor(u32, RgbColor), // u32 target device type
    Flash {
        color: RgbColor,
        duration: Duration,
    },
    SetActive(bool),
}

pub struct RgbController {
    tx: Option<Sender<RgbCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl Default for RgbController {
    fn default() -> Self {
        Self::new()
    }
}

impl RgbController {
    /// Starts the background OpenRGB thread with default config and returns a controller handle.
    pub fn new() -> Self {
        Self::new_with_config(OpenRGBConfig::default())
    }

    /// Starts the background OpenRGB thread with custom config and returns a controller handle.
    pub fn new_with_config(config: OpenRGBConfig) -> Self {
        let (tx, rx) = channel();
        let thread_handle = thread::spawn(move || {
            run_rgb_thread(rx, config);
        });
        Self {
            tx: Some(tx),
            thread_handle: Some(thread_handle),
        }
    }

    /// Set all connected RGB devices to the specified color.
    pub fn set_color(&self, color: RgbColor) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(RgbCommand::SetColor(color));
        }
    }

    /// Set all connected RGB devices of a specific type to the specified color.
    pub fn set_device_color(&self, device_type: u32, color: RgbColor) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(RgbCommand::SetDeviceColor(device_type, color));
        }
    }

    /// Trigger a temporary flash of a color (e.g. for lightning strikes),
    /// which will smoothly fade back to the ambient/previous color.
    pub fn flash(&self, color: RgbColor, duration: Duration) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(RgbCommand::Flash { color, duration });
        }
    }

    /// Pause or resume active RGB operations.
    /// When inactive, the controller restores the devices' initial colors to allow normal
    /// ambient system lighting when the application is minimized or lacks focus.
    pub fn set_active(&self, active: bool) {
        if let Some(tx) = &self.tx {
            let _ = tx.send(RgbCommand::SetActive(active));
        }
    }
}

impl Drop for RgbController {
    fn drop(&mut self) {
        self.tx = None;
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

struct ActiveFlash {
    start_time: Instant,
    end_time: Instant,
    start_color: RgbColor,
    target_color: RgbColor,
}

fn lerp(start: u8, end: u8, t: f32) -> u8 {
    let s = start as f32;
    let e = end as f32;
    (s + (e - s) * t).clamp(0.0, 255.0) as u8
}

fn write_all_devices(client: &mut OpenRGBClient, color: RgbColor) -> Result<(), std::io::Error> {
    let targets: Vec<(u32, u16)> = client.devices.iter().map(|d| (d.index, d.num_leds)).collect();
    for (index, num_leds) in targets {
        let colors = vec![color; num_leds as usize];
        client.update_leds(index, &colors)?;
    }
    Ok(())
}

fn write_device_type(client: &mut OpenRGBClient, device_type: u32, color: RgbColor) -> Result<(), std::io::Error> {
    let targets: Vec<(u32, u16)> = client.devices.iter()
        .filter(|d| d.device_type == device_type)
        .map(|d| (d.index, d.num_leds))
        .collect();
    for (index, num_leds) in targets {
        let colors = vec![color; num_leds as usize];
        client.update_leds(index, &colors)?;
    }
    Ok(())
}

fn run_rgb_thread(rx: Receiver<RgbCommand>, config: OpenRGBConfig) {
    let mut client: Option<OpenRGBClient> = None;
    let mut last_connect_attempt = Instant::now() - Duration::from_secs(10);
    let mut current_color = RgbColor::BLACK;
    let mut active_flash: Option<ActiveFlash> = None;
    let mut is_active = true;

    loop {
        if is_active && client.is_none() && last_connect_attempt.elapsed() > Duration::from_secs(5) {
            last_connect_attempt = Instant::now();
            if let Ok(c) = OpenRGBClient::connect_with_config(&config) {
                client = Some(c);
                // Write current active color to devices immediately upon connection
                if active_flash.is_none() {
                    let _ = write_all_devices(client.as_mut().unwrap(), current_color);
                }
            }
        }

        let timeout = if !is_active {
            Duration::from_millis(500)
        } else if active_flash.is_some() {
            Duration::from_millis(16) // Smooth lerping at ~60fps
        } else if client.is_none() {
            Duration::from_secs(2)
        } else {
            Duration::from_millis(100)
        };

        match rx.recv_timeout(timeout) {
            Ok(RgbCommand::SetActive(active)) => {
                if is_active != active {
                    is_active = active;
                    if !is_active {
                        // Restore initial colors when deactivating
                        if let Some(c) = &mut client {
                            let targets: Vec<(u32, Vec<RgbColor>)> = c.devices.iter()
                                .map(|d| (d.index, d.initial_colors.clone()))
                                .collect();
                            for (index, colors) in targets {
                                let _ = c.update_leds(index, &colors);
                            }
                            let _ = c.stream.flush();
                        }
                    } else {
                        // Re-apply current color when activating
                        if let Some(c) = &mut client {
                            if active_flash.is_none() {
                                let _ = write_all_devices(c, current_color);
                            }
                        }
                    }
                }
            }
            Ok(RgbCommand::SetColor(color)) => {
                current_color = color;
                if is_active && active_flash.is_none() {
                    if let Some(c) = &mut client {
                        if write_all_devices(c, color).is_err() {
                            client = None;
                        }
                    }
                }
            }
            Ok(RgbCommand::SetDeviceColor(device_type, color)) => {
                if is_active && active_flash.is_none() {
                    if let Some(c) = &mut client {
                        if write_device_type(c, device_type, color).is_err() {
                            client = None;
                        }
                    }
                }
            }
            Ok(RgbCommand::Flash { color, duration }) => {
                if is_active {
                    let now = Instant::now();
                    active_flash = Some(ActiveFlash {
                        start_time: now,
                        end_time: now + duration,
                        start_color: current_color,
                        target_color: color,
                    });
                    if let Some(c) = &mut client {
                        if write_all_devices(c, color).is_err() {
                            client = None;
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                if let Some(flash) = &active_flash {
                    let now = Instant::now();
                    if now >= flash.end_time {
                        let orig = flash.start_color;
                        active_flash = None;
                        if let Some(c) = &mut client {
                            if write_all_devices(c, orig).is_err() {
                                client = None;
                            }
                        }
                    } else {
                        let total_dur = flash.end_time.duration_since(flash.start_time).as_secs_f32();
                        let elapsed = now.duration_since(flash.start_time).as_secs_f32();
                        let t = if total_dur > 0.0 {
                            (elapsed / total_dur).clamp(0.0, 1.0)
                        } else {
                            1.0
                        };
                        let r = lerp(flash.target_color.r, flash.start_color.r, t);
                        let g = lerp(flash.target_color.g, flash.start_color.g, t);
                        let b = lerp(flash.target_color.b, flash.start_color.b, t);
                        let lerped = RgbColor { r, g, b };
                        if let Some(c) = &mut client {
                            if write_all_devices(c, lerped).is_err() {
                                client = None;
                            }
                        }
                    }
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                if let Some(mut c) = client {
                    // Always restore the initial colors of all devices first
                    let targets: Vec<(u32, Vec<RgbColor>)> = c.devices.iter()
                        .map(|d| (d.index, d.initial_colors.clone()))
                        .collect();
                    for (index, colors) in targets {
                        let _ = c.update_leds(index, &colors);
                    }

                    // Optionally also trigger the profile load (if one is configured/present)
                    if let Some(profile) = find_restore_profile() {
                        let name_bytes = profile.as_bytes();
                        let name_len = name_bytes.len() as u16;
                        let mut payload = Vec::new();
                        payload.extend_from_slice(&name_len.to_le_bytes());
                        payload.extend_from_slice(name_bytes);

                        let mut header = [0u8; HEADER_SIZE];
                        header[0..4].copy_from_slice(OPENRGB_MAGIC);
                        header[4..8].copy_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // Global device index
                        header[8..12].copy_from_slice(&CMD_LOAD_PROFILE.to_le_bytes()); // LOAD_PROFILE ID
                        header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());

                        let _ = c.stream.write_all(&header);
                        let _ = c.stream.write_all(&payload);
                    }

                    // Flush to allow OpenRGB to process the final packet
                    let _ = c.stream.flush();
                }
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_controller_commands() {
        let controller = RgbController::new();
        controller.set_active(true);
        controller.set_color(RgbColor { r: 255, g: 0, b: 0 });
        controller.set_device_color(1, RgbColor { r: 0, g: 255, b: 0 });
        controller.flash(RgbColor { r: 0, g: 0, b: 255 }, Duration::from_millis(50));
        controller.flash(RgbColor { r: 0, g: 0, b: 255 }, Duration::from_millis(0)); // Test zero duration (no panic)
        controller.set_active(false);
    }
}

pub fn is_openrgb_enabled() -> bool {
    std::env::args().any(|arg| arg == "--enable-openrgb" || arg == "/rgb")
}


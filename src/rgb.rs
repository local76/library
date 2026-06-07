use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const BLACK: Self = Self { r: 0, g: 0, b: 0 };
    pub const WHITE: Self = Self { r: 255, g: 255, b: 255 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

#[derive(Debug, Clone)]
pub struct OpenRGBDevice {
    pub index: u32,
    pub device_type: u32,
    pub name: String,
    pub num_leds: u16,
    pub initial_colors: Vec<RgbColor>,
}

pub enum RgbCommand {
    SetColor(RgbColor),
    SetDeviceColor(u32, RgbColor), // u32 target device type
    Flash {
        color: RgbColor,
        duration: Duration,
    },
}

pub struct RgbController {
    tx: Option<Sender<RgbCommand>>,
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl RgbController {
    /// Starts the background OpenRGB thread and returns a controller handle.
    pub fn new() -> Self {
        let (tx, rx) = channel();
        let thread_handle = thread::spawn(move || {
            run_rgb_thread(rx);
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
}

impl Drop for RgbController {
    fn drop(&mut self) {
        self.tx = None;
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

struct OpenRGBClient {
    stream: TcpStream,
    devices: Vec<OpenRGBDevice>,
}

impl OpenRGBClient {
    fn connect() -> Result<Self, std::io::Error> {
        let addr = SocketAddr::from(([127, 0, 0, 1], 6742));
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(500))?;
        stream.set_read_timeout(Some(Duration::from_millis(500)))?;
        stream.set_write_timeout(Some(Duration::from_millis(500)))?;

        // 1. Request Protocol Version (Command ID 40)
        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(b"ORGB");
        header[4..8].copy_from_slice(&0u32.to_le_bytes());
        header[8..12].copy_from_slice(&40u32.to_le_bytes());
        header[12..16].copy_from_slice(&4u32.to_le_bytes());
        stream.write_all(&header)?;
        stream.write_all(&1u32.to_le_bytes())?; // Negotiating protocol version 1

        let mut resp_header = [0u8; 16];
        stream.read_exact(&mut resp_header)?;
        if &resp_header[0..4] != b"ORGB" {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid magic"));
        }
        let resp_size = u32::from_le_bytes(resp_header[12..16].try_into().unwrap());
        if resp_size == 4 {
            let mut resp_payload = [0u8; 4];
            stream.read_exact(&mut resp_payload)?;
        } else if resp_size > 0 {
            let mut temp = vec![0u8; resp_size as usize];
            stream.read_exact(&mut temp)?;
        }

        // 2. Set Client Name (Command ID 50)
        let name = "rIdle\0";
        let name_bytes = name.as_bytes();
        let name_len = name_bytes.len() as u16;
        let mut payload = Vec::new();
        payload.extend_from_slice(&name_len.to_le_bytes());
        payload.extend_from_slice(name_bytes);

        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(b"ORGB");
        header[4..8].copy_from_slice(&0u32.to_le_bytes());
        header[8..12].copy_from_slice(&50u32.to_le_bytes());
        header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());
        stream.write_all(&header)?;
        stream.write_all(&payload)?;

        // 3. Request Controller Count (Command ID 0)
        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(b"ORGB");
        header[4..8].copy_from_slice(&0u32.to_le_bytes());
        header[8..12].copy_from_slice(&0u32.to_le_bytes());
        header[12..16].copy_from_slice(&0u32.to_le_bytes());
        stream.write_all(&header)?;

        let mut resp_header = [0u8; 16];
        stream.read_exact(&mut resp_header)?;
        let resp_size = u32::from_le_bytes(resp_header[12..16].try_into().unwrap());
        let count = if resp_size == 4 {
            let mut resp_payload = [0u8; 4];
            stream.read_exact(&mut resp_payload)?;
            u32::from_le_bytes(resp_payload)
        } else {
            0
        };

        // 4. Request Controller Data for each index (Command ID 1)
        let mut devices = Vec::new();
        for idx in 0..count {
            let mut header = [0u8; 16];
            header[0..4].copy_from_slice(b"ORGB");
            header[4..8].copy_from_slice(&idx.to_le_bytes());
            header[8..12].copy_from_slice(&1u32.to_le_bytes());
            header[12..16].copy_from_slice(&4u32.to_le_bytes());
            stream.write_all(&header)?;
            stream.write_all(&1u32.to_le_bytes())?;

            let mut resp_header = [0u8; 16];
            stream.read_exact(&mut resp_header)?;
            let resp_size = u32::from_le_bytes(resp_header[12..16].try_into().unwrap());
            let mut dev_payload = vec![0u8; resp_size as usize];
            stream.read_exact(&mut dev_payload)?;

            if let Ok(device) = parse_device_payload(idx, &dev_payload) {
                devices.push(device);
            }
        }

        Ok(Self { stream, devices })
    }

    fn update_leds(&mut self, device_index: u32, colors: &[RgbColor]) -> Result<(), std::io::Error> {
        let num_colors = colors.len() as u16;
        let mut payload = Vec::new();
        payload.extend_from_slice(&(4 + 2 + num_colors as u32 * 4).to_le_bytes());
        payload.extend_from_slice(&num_colors.to_le_bytes());
        for c in colors {
            payload.push(c.r);
            payload.push(c.g);
            payload.push(c.b);
            payload.push(0);
        }

        let mut header = [0u8; 16];
        header[0..4].copy_from_slice(b"ORGB");
        header[4..8].copy_from_slice(&device_index.to_le_bytes());
        header[8..12].copy_from_slice(&1050u32.to_le_bytes());
        header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());

        self.stream.write_all(&header)?;
        self.stream.write_all(&payload)?;
        Ok(())
    }
}

fn parse_device_payload(index: u32, data: &[u8]) -> Result<OpenRGBDevice, &'static str> {
    let mut cursor = 0;

    let read_u16 = |cur: &mut usize| -> Result<u16, &'static str> {
        if *cur + 2 > data.len() { return Err("EOF u16"); }
        let val = u16::from_le_bytes(data[*cur..*cur+2].try_into().unwrap());
        *cur += 2;
        Ok(val)
    };

    let read_u32 = |cur: &mut usize| -> Result<u32, &'static str> {
        if *cur + 4 > data.len() { return Err("EOF u32"); }
        let val = u32::from_le_bytes(data[*cur..*cur+4].try_into().unwrap());
        *cur += 4;
        Ok(val)
    };

    let read_i32 = |cur: &mut usize| -> Result<i32, &'static str> {
        if *cur + 4 > data.len() { return Err("EOF i32"); }
        let val = i32::from_le_bytes(data[*cur..*cur+4].try_into().unwrap());
        *cur += 4;
        Ok(val)
    };

    let read_string = |cur: &mut usize| -> Result<String, &'static str> {
        let len = read_u16(cur)? as usize;
        if len == 0 { return Ok(String::new()); }
        if *cur + len > data.len() { return Err("EOF String"); }
        let s_bytes = &data[*cur..*cur + len];
        *cur += len;
        let clean_len = if len > 0 && s_bytes[len - 1] == 0 { len - 1 } else { len };
        let s = String::from_utf8_lossy(&s_bytes[..clean_len]).into_owned();
        Ok(s)
    };

    let _data_size = read_u32(&mut cursor)?;
    let device_type = read_u32(&mut cursor)?;
    let name = read_string(&mut cursor)?;
    let _vendor = read_string(&mut cursor)?;
    let _description = read_string(&mut cursor)?;
    let _version = read_string(&mut cursor)?;
    let _serial = read_string(&mut cursor)?;
    let _location = read_string(&mut cursor)?;

    let num_modes = read_u16(&mut cursor)?;
    let _active_mode = read_i32(&mut cursor)?;

    for _ in 0..num_modes {
        let _m_name = read_string(&mut cursor)?;
        let _m_value = read_i32(&mut cursor)?;
        let _m_flags = read_u32(&mut cursor)?;
        let _m_speed_min = read_u32(&mut cursor)?;
        let _m_speed_max = read_u32(&mut cursor)?;
        let _m_colors_min = read_u32(&mut cursor)?;
        let _m_colors_max = read_u32(&mut cursor)?;
        let _m_speed = read_u32(&mut cursor)?;
        let _m_direction = read_u32(&mut cursor)?;
        let _m_color_mode = read_u32(&mut cursor)?;
        let colors_len = read_u16(&mut cursor)? as usize;
        if cursor + colors_len * 4 > data.len() { return Err("EOF Mode Colors"); }
        cursor += colors_len * 4;
    }

    let num_zones = read_u16(&mut cursor)?;
    for _ in 0..num_zones {
        let _z_name = read_string(&mut cursor)?;
        let _z_type = read_u32(&mut cursor)?;
        let _z_leds_min = read_u32(&mut cursor)?;
        let _z_leds_max = read_u32(&mut cursor)?;
        let _z_leds_count = read_u32(&mut cursor)?;
        let matrix_len = read_u16(&mut cursor)? as usize;
        if cursor + matrix_len > data.len() { return Err("EOF Zone Matrix"); }
        cursor += matrix_len;
    }

    let num_leds = read_u16(&mut cursor)?;
    for _ in 0..num_leds {
        let _l_name = read_string(&mut cursor)?;
        let _l_value = read_u32(&mut cursor)?;
    }

    let num_colors = read_u16(&mut cursor)?;
    let mut initial_colors = Vec::new();
    for _ in 0..num_colors {
        if cursor + 4 > data.len() { return Err("EOF Colors"); }
        let r = data[cursor];
        let g = data[cursor + 1];
        let b = data[cursor + 2];
        cursor += 4;
        initial_colors.push(RgbColor { r, g, b });
    }

    Ok(OpenRGBDevice {
        index,
        device_type,
        name,
        num_leds: num_colors,
        initial_colors,
    })
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

fn run_rgb_thread(rx: Receiver<RgbCommand>) {
    let mut client: Option<OpenRGBClient> = None;
    let mut last_connect_attempt = Instant::now() - Duration::from_secs(10);
    let mut current_color = RgbColor::BLACK;
    let mut active_flash: Option<ActiveFlash> = None;

    loop {
        if client.is_none() {
            if last_connect_attempt.elapsed() > Duration::from_secs(5) {
                last_connect_attempt = Instant::now();
                if let Ok(c) = OpenRGBClient::connect() {
                    client = Some(c);
                    // Write current active color to devices immediately upon connection
                    if active_flash.is_none() {
                        let _ = write_all_devices(client.as_mut().unwrap(), current_color);
                    }
                }
            }
        }

        let timeout = if active_flash.is_some() {
            Duration::from_millis(16) // Smooth lerping at ~60fps
        } else if client.is_none() {
            Duration::from_secs(2)
        } else {
            Duration::from_millis(100)
        };

        match rx.recv_timeout(timeout) {
            Ok(RgbCommand::SetColor(color)) => {
                current_color = color;
                if active_flash.is_none() {
                    if let Some(c) = &mut client {
                        if write_all_devices(c, color).is_err() {
                            client = None;
                        }
                    }
                }
            }
            Ok(RgbCommand::SetDeviceColor(device_type, color)) => {
                if active_flash.is_none() {
                    if let Some(c) = &mut client {
                        if write_device_type(c, device_type, color).is_err() {
                            client = None;
                        }
                    }
                }
            }
            Ok(RgbCommand::Flash { color, duration }) => {
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
                        let t = (elapsed / total_dur).clamp(0.0, 1.0);
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

                        let mut header = [0u8; 16];
                        header[0..4].copy_from_slice(b"ORGB");
                        header[4..8].copy_from_slice(&0xFFFFFFFFu32.to_le_bytes()); // Global device index
                        header[8..12].copy_from_slice(&150u32.to_le_bytes()); // LOAD_PROFILE ID
                        header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());

                        let _ = c.stream.write_all(&header);
                        let _ = c.stream.write_all(&payload);
                    }

                    // Flush and sleep to allow OpenRGB to process the final packet
                    let _ = c.stream.flush();
                    std::thread::sleep(std::time::Duration::from_millis(150));
                }
                break;
            }
        }
    }
}

fn get_openrgb_dir() -> Option<std::path::PathBuf> {
    #[cfg(target_os = "windows")]
    {
        let appdata = std::env::var("APPDATA").ok()?;
        Some(std::path::PathBuf::from(appdata).join("OpenRGB"))
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(xdg) = std::env::var("XDG_CONFIG_HOME") {
            Some(std::path::PathBuf::from(xdg).join("OpenRGB"))
        } else {
            let home = std::env::var("HOME").ok()?;
            Some(std::path::PathBuf::from(home).join(".config").join("OpenRGB"))
        }
    }
}

fn find_restore_profile() -> Option<String> {
    // 1. Try registry configuration first
    #[cfg(target_os = "windows")]
    {
        use winreg::enums::HKEY_CURRENT_USER;
        use winreg::RegKey;
        if let Ok(hkcu) = RegKey::predef(HKEY_CURRENT_USER).open_subkey(r#"Software\Windows-Screensavers\Settings"#) {
            if let Ok(profile) = hkcu.get_value::<String, _>("OpenRGBRestoreProfile") {
                if !profile.is_empty() {
                    return Some(profile);
                }
            }
        }
    }

    // 2. Scan OpenRGB folder for profiles
    let dir = get_openrgb_dir()?;
    if !dir.exists() { return None; }

    // Check standard profiles
    for preferred in &["default", "restore", "normal"] {
        let p_path = dir.join(format!("{}.orp", preferred));
        if p_path.exists() {
            return Some(preferred.to_string());
        }
    }

    // Fall back to the first profile found
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "orp") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    return Some(stem.to_string());
                }
            }
        }
    }

    None
}

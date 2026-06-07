use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RgbColor {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct OpenRGBDevice {
    index: u32,
    device_type: u32,
    name: String,
    num_leds: u16,
    initial_colors: Vec<RgbColor>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================================================");
    println!("   OpenRGB Connection & Hardware Check");
    println!("===================================================\n");

    let addr = SocketAddr::from(([127, 0, 0, 1], 6742));
    let mut stream = TcpStream::connect_timeout(&addr, Duration::from_millis(800))?;
    stream.set_read_timeout(Some(Duration::from_millis(800)))?;
    stream.set_write_timeout(Some(Duration::from_millis(800)))?;

    println!("[1/4] Negotiating protocol version...");
    let mut header = [0u8; 16];
    header[0..4].copy_from_slice(b"ORGB");
    header[4..8].copy_from_slice(&0u32.to_le_bytes());
    header[8..12].copy_from_slice(&40u32.to_le_bytes());
    header[12..16].copy_from_slice(&4u32.to_le_bytes());
    stream.write_all(&header)?;
    stream.write_all(&1u32.to_le_bytes())?;

    let mut resp_header = [0u8; 16];
    stream.read_exact(&mut resp_header)?;
    let resp_size = u32::from_le_bytes(resp_header[12..16].try_into().unwrap());
    if resp_size == 4 {
        let mut resp_payload = [0u8; 4];
        stream.read_exact(&mut resp_payload)?;
        println!("      Negotiated Version: {}", u32::from_le_bytes(resp_payload));
    } else if resp_size > 0 {
        let mut temp = vec![0u8; resp_size as usize];
        stream.read_exact(&mut temp)?;
    }

    println!("[2/4] Registering client name...");
    let name = "rIdleCheck\0";
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

    println!("[3/4] Requesting device count...");
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
    println!("      Found {} RGB device(s) connected to OpenRGB.", count);

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

        match parse_device_payload(idx, &dev_payload) {
            Ok(device) => {
                let type_str = match device.device_type {
                    0 => "Motherboard",
                    1 => "DRAM (RAM)",
                    2 => "GPU",
                    3 => "Cooler",
                    4 => "LED Strip",
                    5 => "Keyboard",
                    6 => "Mouse",
                    7 => "Mousemat",
                    8 => "Headset",
                    12 => "Speaker",
                    _ => "Other/Unknown",
                };
                println!("        -> [{}] {} (Type: {}, LEDs: {})", device.index, device.name, type_str, device.num_leds);
                devices.push(device);
            }
            Err(e) => {
                println!("        -> Error parsing device [{}]: {}", idx, e);
            }
        }
    }

    if count > 0 {
        println!("\n[4/4] Flashing all devices white for 1 second...");
        for dev in &devices {
            println!("        Flashing device {} (index {})...", dev.name, dev.index);
            let num_colors = dev.num_leds;
            let mut payload = Vec::new();
            payload.extend_from_slice(&(4 + 2 + num_colors as u32 * 4).to_le_bytes());
            payload.extend_from_slice(&num_colors.to_le_bytes());
            for _ in 0..num_colors {
                payload.push(255); // R
                payload.push(255); // G
                payload.push(255); // B
                payload.push(0);   // padding
            }

            let mut header = [0u8; 16];
            header[0..4].copy_from_slice(b"ORGB");
            header[4..8].copy_from_slice(&dev.index.to_le_bytes());
            header[8..12].copy_from_slice(&1050u32.to_le_bytes());
            header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());

            stream.write_all(&header)?;
            stream.write_all(&payload)?;
        }

        println!("      Sleeping for 1 second...");
        std::thread::sleep(Duration::from_secs(1));

        println!("      Restoring initial colors...");
        for dev in &devices {
            println!("        Restoring device {} (index {})...", dev.name, dev.index);
            let num_colors = dev.num_leds;
            let mut payload = Vec::new();
            payload.extend_from_slice(&(4 + 2 + num_colors as u32 * 4).to_le_bytes());
            payload.extend_from_slice(&num_colors.to_le_bytes());
            for color in &dev.initial_colors {
                payload.push(color.r);
                payload.push(color.g);
                payload.push(color.b);
                payload.push(0);
            }

            let mut header = [0u8; 16];
            header[0..4].copy_from_slice(b"ORGB");
            header[4..8].copy_from_slice(&dev.index.to_le_bytes());
            header[8..12].copy_from_slice(&1050u32.to_le_bytes());
            header[12..16].copy_from_slice(&(payload.len() as u32).to_le_bytes());

            stream.write_all(&header)?;
            stream.write_all(&payload)?;
        }
    }

    println!("\nVerification completed successfully!");
    Ok(())
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

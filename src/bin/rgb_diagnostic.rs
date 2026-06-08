use std::time::Duration;
use rcommon::role::application::rgb::{OpenRGBClient, RgbColor, device_type_name};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===================================================");
    println!("   OpenRGB Connection & Hardware Check");
    println!("===================================================\n");

    println!("[1/2] Connecting and retrieving devices...");
    let mut client = OpenRGBClient::connect()?;
    println!("      Found {} RGB device(s) connected to OpenRGB.", client.devices.len());

    let mut targets = Vec::new();
    for device in &client.devices {
        let type_str = device_type_name(device.device_type);
        println!("        -> [{}] {} (Type: {}, LEDs: {})", device.index, device.name, type_str, device.num_leds);
        targets.push((device.index, device.name.clone(), device.num_leds, device.initial_colors.clone()));
    }

    if !targets.is_empty() {
        println!("\n[2/2] Flashing all devices white for 1 second...");
        // Flash all devices white
        for (index, name, num_leds, _) in &targets {
            println!("        Flashing device {} (index {})...", name, index);
            let colors = vec![RgbColor::WHITE; *num_leds as usize];
            client.update_leds(*index, &colors)?;
        }

        println!("      Sleeping for 1 second...");
        std::thread::sleep(Duration::from_secs(1));

        println!("      Restoring initial colors...");
        // Restore initial colors
        for (index, name, _, initial_colors) in &targets {
            println!("        Restoring device {} (index {})...", name, index);
            client.update_leds(*index, initial_colors)?;
        }
    }

    println!("\nVerification completed successfully!");
    Ok(())
}

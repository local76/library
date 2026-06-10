//! Wireless network command stubs for non-Windows (nmcli-based Linux).
//!
//! **Taxonomy Classification**: Platform (WLAN / Stub).

use std::process::Command;
use crate::toolkit::wlan::{WlanNetwork, RadioState};

fn get_wifi_interface() -> String {
    if let Ok(output) = Command::new("nmcli")
        .args(["-t", "-f", "DEVICE,TYPE", "device"])
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 2 && parts[1].trim() == "wifi" {
                return parts[0].trim().to_string();
            }
        }
    }
    "wlan0".to_string()
}

pub fn query_wifi_networks(force_scan: bool) -> Result<Vec<WlanNetwork>, u32> {
    if force_scan {
        let _ = Command::new("nmcli").args(["device", "wifi", "rescan"]).status();
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    let output = Command::new("nmcli")
        .args(["-t", "-f", "SSID,SIGNAL,ACTIVE,SECURITY", "device", "wifi", "list"])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut networks = Vec::new();
            let saved_profiles = query_saved_profiles().unwrap_or_default();
            
            for line in stdout.lines() {
                if line.trim().is_empty() {
                    continue;
                }
                
                let mut parts = Vec::new();
                let mut current = String::new();
                let mut chars = line.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '\\' && chars.peek() == Some(&':') {
                        current.push(':');
                        chars.next();
                    } else if c == ':' {
                        parts.push(current.clone());
                        current.clear();
                    } else {
                        current.push(c);
                    }
                }
                parts.push(current);

                if parts.len() < 4 {
                    continue;
                }

                let ssid = parts[0].clone();
                if ssid.is_empty() {
                    continue;
                }
                let signal_quality = parts[1].parse::<u32>().unwrap_or(0);
                let is_connected = parts[2] == "yes";
                let security_str = parts[3].clone();
                let security_enabled = !security_str.is_empty() && security_str != "--";
                let has_profile = saved_profiles.iter().any(|(name, _)| name == &ssid);

                if let Some(existing) = networks.iter_mut().find(|n: &&mut WlanNetwork| n.ssid == ssid) {
                    if signal_quality > existing.signal_quality {
                        existing.signal_quality = signal_quality;
                        existing.is_connected = is_connected;
                    }
                } else {
                    networks.push(WlanNetwork {
                        ssid,
                        signal_quality,
                        is_connected,
                        has_profile,
                        security_enabled,
                        auth_algorithm: security_str,
                        cipher_algorithm: "Unknown".to_string(),
                        interface_guid: Default::default(),
                    });
                }
            }
            Ok(networks)
        }
        _ => Err(1),
    }
}

pub fn connect_to_wifi(ssid: &str, password: Option<&str>, _net: &WlanNetwork) -> Result<(), String> {
    let mut args = vec!["device", "wifi", "connect", ssid];
    if let Some(pwd) = password {
        if !pwd.is_empty() {
            args.push("password");
            args.push(pwd);
        }
    }
    
    let output = Command::new("nmcli").args(&args).output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn disconnect_wifi(_interface_guid: &super::windows_sys::core::GUID) -> Result<(), String> {
    let iface = get_wifi_interface();
    let output = Command::new("nmcli")
        .args(["device", "disconnect", &iface])
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn delete_wifi_profile(ssid: &str, _interface_guid: &super::windows_sys::core::GUID) -> Result<(), String> {
    let output = Command::new("nmcli")
        .args(["connection", "delete", ssid])
        .output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn get_first_interface_guid() -> Result<super::windows_sys::core::GUID, u32> {
    Ok(Default::default())
}

pub fn query_radio_state(_interface_guid: &super::windows_sys::core::GUID) -> Result<RadioState, u32> {
    let output = Command::new("nmcli").args(["radio", "wifi"]).output();
    match output {
        Ok(out) if out.status.success() => {
            let state = String::from_utf8_lossy(&out.stdout).trim().to_string();
            let software_on = state == "enabled";
            Ok(RadioState { software_on, hardware_on: true })
        }
        _ => Err(1),
    }
}

pub fn set_radio_state(_interface_guid: &super::windows_sys::core::GUID, software_on: bool) -> Result<(), u32> {
    let state = if software_on { "on" } else { "off" };
    let status = Command::new("nmcli")
        .args(["radio", "wifi", state])
        .status();
    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(1),
    }
}

pub fn query_saved_password(ssid: &str) -> Option<String> {
    if let Ok(output) = Command::new("nmcli")
        .args(["-s", "-g", "802-11-wireless-security.psk", "connection", "show", ssid])
        .output()
    {
        if output.status.success() {
            let pwd = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !pwd.is_empty() {
                return Some(pwd);
            }
        }
    }
    None
}

pub fn query_saved_profiles() -> Result<Vec<(String, super::windows_sys::core::GUID)>, u32> {
    let output = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let stdout = String::from_utf8_lossy(&out.stdout);
            let mut profiles = Vec::new();
            for line in stdout.lines() {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 && parts[1].trim() == "802-11-wireless" {
                    profiles.push((parts[0].trim().to_string(), Default::default()));
                }
            }
            Ok(profiles)
        }
        _ => Err(1),
    }
}

pub fn connect_to_enterprise_wifi(
    ssid: &str,
    username: &str,
    password: &str,
    _net: &WlanNetwork,
) -> Result<(), String> {
    let iface = get_wifi_interface();
    let con_name = ssid;
    let _ = Command::new("nmcli").args(["connection", "delete", con_name]).status();

    let output = Command::new("nmcli")
        .args([
            "connection", "add", "type", "wifi",
            "con-name", con_name,
            "ifname", &iface,
            "ssid", ssid,
            "--",
            "wifi-sec.key-mgmt", "wpa-eap",
            "802-1x.identity", username,
            "802-1x.password", password,
            "802-1x.eap", "peap",
            "802-1x.phase2-auth", "mschapv2"
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let up_output = Command::new("nmcli")
                .args(["connection", "up", "id", con_name])
                .output();
            match up_output {
                Ok(up_out) if up_out.status.success() => Ok(()),
                Ok(up_out) => Err(String::from_utf8_lossy(&up_out.stderr).into_owned()),
                Err(e) => Err(e.to_string()),
            }
        }
        Ok(out) => Err(String::from_utf8_lossy(&out.stderr).into_owned()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn connect_to_hidden_wifi(
    ssid: &str,
    password: Option<&str>,
    security_enabled: bool,
    _auth_algo: &str,
    _cipher_algo: &str,
    _interface_guid: &super::windows_sys::core::GUID,
) -> Result<(), String> {
    let mut args = vec!["device", "wifi", "connect", ssid, "hidden", "yes"];
    if security_enabled {
        if let Some(pwd) = password {
            if !pwd.is_empty() {
                args.push("password");
                args.push(pwd);
            }
        }
    }
    
    let output = Command::new("nmcli").args(&args).output();
    match output {
        Ok(out) => {
            if out.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&out.stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

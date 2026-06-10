//! Windows WiFi connection, disconnection, and profile management implementation.
//!
//! **Taxonomy Classification**: Platform (WLAN / Win32 Connect).

use windows_sys::Win32::NetworkManagement::WiFi::*;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use crate::toolkit::wlan::WlanNetwork;

/// Helper to escape XML characters for WiFi profiles.
pub fn escape_xml(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for c in input.chars() {
        match c {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(c),
        }
    }
    escaped
}

pub fn connect_to_wifi(ssid: &str, password: Option<&str>, net: &WlanNetwork) -> Result<(), String> {
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    if !net.has_profile {
        // We need to create an XML profile.
        let xml_profile = if net.security_enabled {
            let pwd = password.ok_or_else(|| "Password required for secured network".to_string())?;
            
            let (xml_auth, xml_cipher) = match net.auth_algorithm.as_str() {
                "WPA2-Personal" | "WPA2PSK" => ("WPA2PSK", "AES"),
                "WPA-Personal" | "WPAPSK" => ("WPAPSK", "TKIP"),
                "WPA3-Personal" | "WPA3SAE" => ("WPA3SAE", "AES"),
                "Open" if net.security_enabled => ("WEP", "WEP"),
                _ => ("WPA2PSK", "AES"), // default fallback
            };

            // Convert SSID to hex string
            let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");

            let escaped_ssid = escape_xml(ssid);
            let escaped_pwd = escape_xml(pwd);

            format!(
                r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>{}</authentication>
                <encryption>{}</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
            <sharedKey>
                <keyType>passPhrase</keyType>
                <protected>false</protected>
                <keyMaterial>{}</keyMaterial>
            </sharedKey>
        </security>
    </MSM>
</WLANProfile>"#,
                escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher, escaped_pwd
            )
        } else {
            // Open, unsecured network
            let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
            let escaped_ssid = escape_xml(ssid);
            format!(
                r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>open</authentication>
                <encryption>none</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
        </security>
    </MSM>
</WLANProfile>"#,
                escaped_ssid, ssid_hex, escaped_ssid
            )
        };

        // Set the profile in Windows
        let xml_w: Vec<u16> = xml_profile.encode_utf16().chain(std::iter::once(0)).collect();
        let mut reason_code = 0;
        let res = unsafe {
            WlanSetProfile(
                handle,
                &net.interface_guid,
                0,
                xml_w.as_ptr(),
                std::ptr::null(),
                1,
                std::ptr::null(),
                &mut reason_code,
            )
        };

        if res != ERROR_SUCCESS {
            unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
            return Err(format!("WlanSetProfile failed: code {}, reason {}", res, reason_code));
        }
    }

    // Now connect to the profile
    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();
    
    // We connect using connection mode: wlan_connection_mode_profile
    let mut conn_params: WLAN_CONNECTION_PARAMETERS = unsafe { std::mem::zeroed() };
    conn_params.wlanConnectionMode = wlan_connection_mode_profile;
    conn_params.strProfile = profile_w.as_ptr();
    conn_params.dot11BssType = dot11_BSS_type_infrastructure;

    let res = unsafe {
        WlanConnect(
            handle,
            &net.interface_guid,
            &conn_params,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanConnect failed: {}", res));
    }

    Ok(())
}

pub fn disconnect_wifi(interface_guid: &windows_sys::core::GUID) -> Result<(), String> {
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let res = unsafe { WlanDisconnect(handle, interface_guid, std::ptr::null()) };
    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanDisconnect failed: {}", res));
    }

    Ok(())
}

pub fn delete_wifi_profile(ssid: &str, interface_guid: &windows_sys::core::GUID) -> Result<(), String> {
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let ssid_w: Vec<u16> = ssid.encode_utf16().chain(std::iter::once(0)).collect();
    let res = unsafe { WlanDeleteProfile(handle, interface_guid, ssid_w.as_ptr(), std::ptr::null()) };
    
    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanDeleteProfile failed: {}", res));
    }

    Ok(())
}

pub fn connect_to_hidden_wifi(
    ssid: &str,
    password: Option<&str>,
    security_enabled: bool,
    auth_algo: &str,
    _cipher_algo: &str,
    interface_guid: &windows_sys::core::GUID,
) -> Result<(), String> {
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    let xml_profile = if security_enabled {
        let pwd = password.ok_or_else(|| "Password required for secured network".to_string())?;
        
        let (xml_auth, xml_cipher) = match auth_algo {
            "WPA2-Personal" | "WPA2PSK" => ("WPA2PSK", "AES"),
            "WPA-Personal" | "WPAPSK" => ("WPAPSK", "TKIP"),
            "WPA3-Personal" | "WPA3SAE" => ("WPA3SAE", "AES"),
            "Open" if security_enabled => ("WEP", "WEP"),
            _ => ("WPA2PSK", "AES"),
        };

        let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
        let escaped_ssid = escape_xml(ssid);
        let escaped_pwd = escape_xml(pwd);

        format!(
            r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
        <nonBroadcast>true</nonBroadcast>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>{}</authentication>
                <encryption>{}</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
            <sharedKey>
                <keyType>passPhrase</keyType>
                <protected>false</protected>
                <keyMaterial>{}</keyMaterial>
            </sharedKey>
        </security>
    </MSM>
</WLANProfile>"#,
            escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher, escaped_pwd
        )
    } else {
        let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
        let escaped_ssid = escape_xml(ssid);
        format!(
            r#"<?xml version="1.0"?>
<WLANProfile xmlns="http://www.microsoft.com/networking/WLAN/profile/v1">
    <name>{}</name>
    <SSIDConfig>
        <SSID>
            <hex>{}</hex>
            <name>{}</name>
        </SSID>
        <nonBroadcast>true</nonBroadcast>
    </SSIDConfig>
    <connectionType>ESS</connectionType>
    <connectionMode>manual</connectionMode>
    <MSM>
        <security>
            <authEncryption>
                <authentication>open</authentication>
                <encryption>none</encryption>
                <useOneX>false</useOneX>
            </authEncryption>
        </security>
    </MSM>
</WLANProfile>"#,
            escaped_ssid, ssid_hex, escaped_ssid
        )
    };

    let xml_w: Vec<u16> = xml_profile.encode_utf16().chain(std::iter::once(0)).collect();
    let mut reason_code = 0;
    let res = unsafe {
        WlanSetProfile(
            handle,
            interface_guid,
            0,
            xml_w.as_ptr(),
            std::ptr::null(),
            1,
            std::ptr::null(),
            &mut reason_code,
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(format!("WlanSetProfile failed: code {}, reason {}", res, reason_code));
    }

    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();
    let mut conn_params: WLAN_CONNECTION_PARAMETERS = unsafe { std::mem::zeroed() };
    conn_params.wlanConnectionMode = wlan_connection_mode_profile;
    conn_params.strProfile = profile_w.as_ptr();
    conn_params.dot11BssType = dot11_BSS_type_infrastructure;

    let res = unsafe {
        WlanConnect(
            handle,
            interface_guid,
            &conn_params,
            std::ptr::null(),
        )
    };

    unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };

    if res != ERROR_SUCCESS {
        return Err(format!("WlanConnect failed: {}", res));
    }

    Ok(())
}

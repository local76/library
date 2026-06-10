//! Windows WPA-Enterprise WiFi connection implementation.
//!
//! **Taxonomy Classification**: Platform (WLAN / Win32 Enterprise).

use windows_sys::Win32::NetworkManagement::WiFi::*;
use windows_sys::Win32::Foundation::ERROR_SUCCESS;
use crate::toolkit::wlan::WlanNetwork;
use super::connect::escape_xml;

pub fn connect_to_enterprise_wifi(
    ssid: &str,
    username: &str,
    password: &str,
    net: &WlanNetwork,
) -> Result<(), String> {
    let mut negotiated_version = 0;
    let mut handle = std::ptr::null_mut();
    
    let res = unsafe { WlanOpenHandle(2, std::ptr::null(), &mut negotiated_version, &mut handle) };
    if res != ERROR_SUCCESS {
        return Err(format!("WlanOpenHandle failed: {}", res));
    }

    let profile_name = ssid.to_string();

    let xml_auth = "WPA2";
    let xml_cipher = "AES";
    let ssid_hex = ssid.as_bytes().iter().map(|b| format!("{:02X}", b)).collect::<Vec<String>>().join("");
    let escaped_ssid = escape_xml(ssid);

    let profile_xml = format!(
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
                <useOneX>true</useOneX>
            </authEncryption>
            <OneX xmlns="http://www.microsoft.com/networking/OneX/v1">
                <authMode>user</authMode>
                <EAPConfig>
                    <EapHostConfig xmlns="http://www.microsoft.com/networking/EapHostConfig">
                        <EapMethod>
                            <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">25</Type>
                            <VendorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</VendorId>
                            <AuthorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</AuthorId>
                            <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</Type>
                        </EapMethod>
                        <Config xmlns="http://www.microsoft.com/networking/EapHostConfig">
                            <EapType xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                                <Type>25</Type>
                                <ConfigP xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                                    <peapEapConnectionProperties xmlns="http://www.microsoft.com/provisioning/mspeapconnectionpropertiesv1">
                                        <PerformServerValidation>false</PerformServerValidation>
                                        <AcceptServerName>false</AcceptServerName>
                                        <FastReconnect>true</FastReconnect>
                                        <InnerEapOptional>false</InnerEapOptional>
                                        <Eap xmlns="http://www.microsoft.com/provisioning/baseeapconnectionpropertiesv1">
                                            <Type>26</Type>
                                            <Config xmlns="http://www.microsoft.com/provisioning/baseeapconnectionpropertiesv1">
                                                <EapType xmlns="http://www.microsoft.com/provisioning/mschapv2connectionpropertiesv1">
                                                    <UseWinLogonCredentials>false</UseWinLogonCredentials>
                                                </EapType>
                                            </Config>
                                        </Eap>
                                    </peapEapConnectionProperties>
                                </ConfigP>
                            </EapType>
                        </Config>
                    </EapHostConfig>
                </EAPConfig>
            </OneX>
        </security>
    </MSM>
</WLANProfile>"#,
        escaped_ssid, ssid_hex, escaped_ssid, xml_auth, xml_cipher
    );

    let xml_w: Vec<u16> = profile_xml.encode_utf16().chain(std::iter::once(0)).collect();
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

    let escaped_user = escape_xml(username);
    let escaped_pwd = escape_xml(password);
    let user_xml = format!(
        r#"<EapHostUserCredentials xmlns="http://www.microsoft.com/networking/EapHostUserCredentials" xmlns:xs="http://www.w3.org/2001/XMLSchema" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
    <EapMethod>
        <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">25</Type>
        <VendorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</VendorId>
        <AuthorId xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</AuthorId>
        <Type xmlns="http://www.microsoft.com/grouping/eapandcert/eapmethodconfig">0</Type>
    </EapMethod>
    <Credentials xmlns="http://www.microsoft.com/networking/EapHostUserCredentials">
        <EapType xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
            <Type>25</Type>
            <ConfigP xmlns="http://www.microsoft.com/grouping/eapandcert/eapdeviceconfig">
                <peapUserProperties xmlns="http://www.microsoft.com/provisioning/mspeapuserpropertiesv1">
                    <RoutingIdentity xsi:nil="true"/>
                    <Eap xmlns="http://www.microsoft.com/provisioning/baseeapuserpropertiesv1">
                        <Type>26</Type>
                        <Config xmlns="http://www.microsoft.com/provisioning/baseeapuserpropertiesv1">
                            <EapType xmlns="http://www.microsoft.com/provisioning/mschapv2userpropertiesv1">
                                <Username>{}</Username>
                                <Password>{}</Password>
                                <LogonDomain></LogonDomain>
                            </EapType>
                        </Config>
                    </Eap>
                </peapUserProperties>
            </ConfigP>
        </EapType>
    </Credentials>
</EapHostUserCredentials>"#,
        escaped_user, escaped_pwd
    );

    let user_w: Vec<u16> = user_xml.encode_utf16().chain(std::iter::once(0)).collect();
    let profile_w: Vec<u16> = profile_name.encode_utf16().chain(std::iter::once(0)).collect();

    #[link(name = "wlanapi")]
    unsafe extern "system" {
        fn WlanSetProfileEapXmlUserData(
            hClientHandle: *mut std::ffi::c_void,
            pInterfaceGuid: *const windows_sys::core::GUID,
            strProfileName: *const u16,
            dwFlags: u32,
            strEapXmlUserData: *const u16,
            pReserved: *mut std::ffi::c_void,
        ) -> u32;
    }

    let res = unsafe {
        WlanSetProfileEapXmlUserData(
            handle,
            &net.interface_guid,
            profile_w.as_ptr(),
            0,
            user_w.as_ptr(),
            std::ptr::null_mut(),
        )
    };

    if res != ERROR_SUCCESS {
        unsafe { WlanCloseHandle(handle, std::ptr::null_mut()) };
        return Err(format!("WlanSetProfileEapXmlUserData failed: code {}", res));
    }

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

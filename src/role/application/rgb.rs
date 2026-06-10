//! RGB Lighting controls and OpenRGB integration.
//!
//! **Taxonomy Classification**: System Role (Purpose - Application Software).

pub mod protocol;
pub mod controller;

pub use protocol::{RgbColor, OpenRGBDevice, parse_device_payload, OpenRGBConfig, OpenRGBClient, device_type_name};
pub use controller::{RgbCommand, RgbController};

pub fn is_openrgb_enabled() -> bool {
    std::env::args().any(|arg| arg == "--enable-openrgb" || arg == "/rgb")
}

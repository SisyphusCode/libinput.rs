use evdev::{Device, EnumerateDevices};
use std::error::Error;
use log::info;

pub struct TrackedHardware {
    pub touchpad: Option<Device>,
    pub trackpoint: Option<Device>,
}

pub fn scan_input_devices() -> Result<Vec<Device>, Box<dyn Error>> {
    let mut tracked = Vec::new();
    for mut device in EnumerateDevices::new() {
        let name = device.name().unwrap_or("Unknown").to_lowercase();
        if name.contains("touchpad") || name.contains("trackpoint") || name.contains("elan") || name.contains("synaptics") {
            info!("Found target hardware: {}", name);
            if let Ok(_) = device.grab() {
                tracked.push(device);
            }
        }
    }
    Ok(tracked)
}

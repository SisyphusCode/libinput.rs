mod config;
mod device;
mod event_loop;
mod virtual_device;
mod gestures;

use std::error::Error;
use log::{info, error};

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    info!("Starting libinput-rs daemon...");

    let config = config::load_config().unwrap_or_default();
    
    let mut devices = device::scan_input_devices()?;
    if devices.is_empty() {
        error!("No suitable input devices found in /dev/input/");
        return Ok(());
    }

    let mut v_device = virtual_device::VirtualDevice::new()?;

    info!("Entering mio event loop...");
    event_loop::run(&mut devices, &mut v_device, &config)?;

    Ok(())
}

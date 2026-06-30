use evdev::Device;
use mio::{Events, Poll, Token, Interest};
use mio::unix::SourceFd;
use std::os::unix::io::AsRawFd;
use std::error::Error;
use crate::virtual_device::VirtualDevice;
use crate::config::InputConfig;

pub fn run(
    devices: &mut Vec<Device>, 
    v_device: &mut VirtualDevice, 
    _config: &InputConfig
) -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);

    for (i, device) in devices.iter().enumerate() {
        let raw_fd = device.as_raw_fd();
        poll.registry().register(&mut SourceFd(&raw_fd), Token(i), Interest::READABLE)?;
    }

    loop {
        poll.poll(&mut events, None)?;
        for event in events.iter() {
            let token_id = event.token().0;
            if let Some(device) = devices.get_mut(token_id) {
                if let Ok(ev_batch) = device.fetch_events() {
                    for ev in ev_batch {
                        v_device.emit_raw(ev)?;
                    }
                }
            }
        }
    }
}

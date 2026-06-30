use evdev::{uinput::VirtualDeviceBuilder, InputEvent, Key, RelativeAxisType, AbsoluteAxisType};
use std::error::Error;

pub struct VirtualDevice {
    device: evdev::uinput::VirtualDevice,
}

impl VirtualDevice {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let mut keys = evdev::AttributeSet::new();
        keys.insert(Key::BTN_LEFT);
        keys.insert(Key::BTN_RIGHT);
        keys.insert(Key::BTN_MIDDLE);

        let mut rel_axes = evdev::AttributeSet::new();
        rel_axes.insert(RelativeAxisType::REL_X);
        rel_axes.insert(RelativeAxisType::REL_Y);
        rel_axes.insert(RelativeAxisType::REL_WHEEL);
        
        let mut abs_axes = evdev::AttributeSet::new();
        abs_axes.insert(AbsoluteAxisType::ABS_X);
        abs_axes.insert(AbsoluteAxisType::ABS_Y);

        let device = VirtualDeviceBuilder::new()?
            .name("libinput-rs Virtual Mouse")
            .with_keys(&keys)?
            .with_relative_axes(&rel_axes)?
            .with_absolute_axes(&abs_axes)?
            .build()?;

        Ok(Self { device })
    }

    pub fn emit_raw(&mut self, event: InputEvent) -> Result<(), Box<dyn Error>> {
        self.device.emit(&[event])?;
        Ok(())
    }
}

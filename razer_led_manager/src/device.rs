use crate::effect::Effect;
use crate::source::DeviceSource;

pub struct DeviceManager<'a> {
    devices: Vec<Device>,
    source: &'a dyn DeviceSource,
}

impl DeviceManager<'_> {
    pub fn new(source: &dyn DeviceSource) -> DeviceManager {
        DeviceManager {
            devices: devices(),
            source,
        }
    }

    pub fn set_device_effect(&self, device_identifier: &str, effect: Effect) {
        // TODO: Handle missing devices
        let device = self.devices
            .iter()
            .find(|d| d.identifier == device_identifier)
            .unwrap();
        // TODO: Check if effect is supported
        let source = self.source
            .find()
            .iter()
            .find(|s| s.product_id == device.product_id)
            .unwrap();

        self.source.send_effect_command(source, effect.command());
    }
}

pub struct Device {
    pub name: &'static str,
    pub identifier: &'static str,
    pub device_type: DeviceType,
    pub supported_effects: Vec<Effect>,
    pub product_id: u16,
}

#[derive(Debug)]
pub enum DeviceType {
    Keyboard,
}

fn devices() -> Vec<Device> {
    vec![Device {
        name: "Ornata Chroma",
        identifier: "ornata_chroma",
        device_type: DeviceType::Keyboard,
        supported_effects: vec![Effect::Static, Effect::Wave],
        product_id: 0x021E,
    }]
}

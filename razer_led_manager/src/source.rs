use crate::effect::EffectCommand;

pub trait DeviceSource {
    fn find(&self) -> &Vec<Device>;

    fn send_effect_command(&self, device: &Device, command: EffectCommand);
}

pub struct Device {
    pub product_id: u16,
}
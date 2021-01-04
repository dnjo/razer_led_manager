mod device;
mod effect;
mod source;

use device::DeviceManager;
use source::DeviceSource;
use crate::source::UsbDeviceSource;
pub use effect::Effect;
use libusb::Context;

pub fn init_manager() -> DeviceManager<UsbDeviceSource> {
    let source = UsbDeviceSource::new(Context::new().unwrap());
    DeviceManager::new(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::{EffectCommand, Effect};
    use crate::source::Device;
    use std::cell::RefCell;

    struct MockDeviceSource<'a> {
        devices: Vec<Device>,
        sent_commands: &'a RefCell<Vec<(u16, EffectCommand)>>,
    }

    impl MockDeviceSource<'_> {
        fn new(devices: Vec<Device>, sent_commands: &RefCell<Vec<(u16, EffectCommand)>>) -> MockDeviceSource {
            MockDeviceSource {
                devices,
                sent_commands
            }
        }
    }

    impl DeviceSource for MockDeviceSource<'_> {
        fn find(&self) -> Vec<Device> {
            self.devices.to_vec()
        }

        fn send_effect_command(&self, device: &Device, command: EffectCommand) {
            self.sent_commands.borrow_mut().push((device.product_id, command));
        }
    }

    #[test]
    fn set_effect_with_device_identifier() {
        let devices = vec![Device {
            product_id: 0x021E,
        }];
        let sent_commands = RefCell::new(vec![]);
        let source = MockDeviceSource::new(devices, &sent_commands);
        let manager = DeviceManager::new(source);
        manager.set_device_effect("ornata_chroma", Effect::Static);

        let sent_commands = sent_commands.borrow();
        assert_eq!(sent_commands.len(), 1);

        let (product_id, command) = sent_commands.get(0).unwrap();
        assert_eq!(product_id, &0x021E);
        assert_eq!(command.command_id, Effect::Static.command().command_id);
    }
}

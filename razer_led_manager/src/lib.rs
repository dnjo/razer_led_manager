mod device;
mod effect;
mod source;

pub use device::DeviceManager;
pub use source::DeviceSource;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::effect::{EffectCommand, Effect};
    use crate::source::Device;
    use std::cell::RefCell;

    struct MockDeviceSource {
        devices: Vec<Device>,
        sent_commands: RefCell<Vec<(u16, EffectCommand)>>,
    }

    impl MockDeviceSource {
        fn new(devices: Vec<Device>) -> MockDeviceSource {
            MockDeviceSource {
                devices,
                sent_commands: RefCell::new(vec![])
            }
        }
    }

    impl DeviceSource for MockDeviceSource {
        fn find(&self) -> &Vec<Device> {
            &self.devices
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
        let source = MockDeviceSource::new(devices);
        let manager = DeviceManager::new(&source);
        manager.set_device_effect("ornata_chroma", Effect::Static);

        let sent_commands = source.sent_commands.borrow();
        assert_eq!(sent_commands.len(), 1);

        let (product_id, command) = sent_commands.get(0).unwrap();
        assert_eq!(product_id, &0x021E);
        assert_eq!(command.command_id, Effect::Static.command().command_id);
    }
}

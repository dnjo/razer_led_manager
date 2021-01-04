extern crate libusb;

use libusb::Context;
use crate::effect::EffectCommand;

const RAZER_VENDOR_ID: u16 = 0x1532;

pub trait DeviceSource {
    fn find(&self) -> Vec<Device>;

    fn send_effect_command(&self, device: &Device, command: EffectCommand);
}

#[derive(Clone)]
pub struct Device {
    pub product_id: u16,
}

pub struct UsbDeviceSource {
    context: Context,
}

impl DeviceSource for UsbDeviceSource {
    fn find(&self) -> Vec<Device> {
        let mut found_devices = Vec::new();
        for device in self.context.devices().unwrap().iter() {
            let device_desc = device.device_descriptor().unwrap();

            if device_desc.vendor_id() == RAZER_VENDOR_ID {
                found_devices.push(Device {
                    product_id: device_desc.product_id()
                });
            }
        }

        found_devices
    }

    fn send_effect_command(&self, device: &Device, command: EffectCommand) {
        for usb_device in self.context.devices().unwrap().iter() {
            let device_desc = usb_device.device_descriptor().unwrap();

            let vendor_id = device_desc.vendor_id();
            let product_id = device_desc.product_id();
            if vendor_id == RAZER_VENDOR_ID && product_id == device.product_id {
                println!("Sending effect command {:?} to device with product ID {:#06X}", command, device.product_id);
            }
        }
    }
}

impl UsbDeviceSource {
    pub fn new(context: Context) -> UsbDeviceSource {
        UsbDeviceSource {
            context
        }
    }
}
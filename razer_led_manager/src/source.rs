extern crate libusb;

use libusb::Context;
use crate::effect::EffectCommand;
use self::libusb::{Direction, RequestType, Recipient, Error};
use std::time::Duration;

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
            if vendor_id != RAZER_VENDOR_ID || product_id != device.product_id {
                continue;
            }
            println!("Sending effect command {:?} to device with product ID {:#06X}", command, device.product_id);

            let handle = usb_device.open().unwrap();
            println!("Opened device");
            let mut report = init_report();
            set_report_crc(&mut report);
            let buf = unsafe { any_as_u8_slice(&report) };
            let request_type = libusb::request_type(Direction::Out, RequestType::Class, Recipient::Interface);
            let result = handle.write_control(request_type, 0x09, 0x300, 0x01, &buf, Duration::from_secs(5));
            match result {
                Ok(res) => println!("Got ok: {}", res),
                Err(err) => println!("Got error: {}", err)
            }

            // let request_type = libusb::request_type(Direction::In, RequestType::Class, Recipient::Interface);
            // let mut response_buffer = [0; 0x5A];
            // let result = handle.read_control(request_type, 0x01, 0x300, 0x01, &mut response_buffer, Duration::from_secs(5));
            // match result {
            //     Ok(_) => {
            //         println!("Response bytes: {:?}", response_buffer);
            //     },
            //     Err(err) => println!("Got error: {}", err)
            // }
        }
    }
}

// https://stackoverflow.com/a/42186553
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::std::slice::from_raw_parts(
        (p as *const T) as *const u8,
        ::std::mem::size_of::<T>(),
    )
}

impl UsbDeviceSource {
    pub fn new(context: Context) -> UsbDeviceSource {
        UsbDeviceSource {
            context
        }
    }
}

fn init_report() -> RazerReport {
    let mut arguments = [0; 80];
    arguments[0] = 0x01;
    arguments[1] = 0x05;
    arguments[2] = 0x01;
    arguments[5] = 0x01;
    arguments[6] = 0xFF;
    arguments[7] = 0x00;
    arguments[8] = 0x00;
    RazerReport {
        status: 0x00,
        transaction_id: 0x3F,
        remaining_packets: 0x00,
        protocol_type: 0x00,
        data_size: 0x09,
        command_class: 0x0F,
        command_id: 0x02,
        arguments,
        crc: 0x00,
        reserved: 0x00
    }
}

fn set_report_crc(report: &mut RazerReport) {
    let mut crc : u8 = 0;
    crc ^= report.remaining_packets as u8;
    crc ^= report.protocol_type;
    crc ^= report.data_size;
    crc ^= report.command_class;
    crc ^= report.command_id;
    for index in 0..report.arguments.len() {
        crc ^= report.arguments[index];
    }
    println!("Setting CRC field to {}", crc);
    report.crc = crc;
}

#[derive(Debug)]
#[repr(packed)]
struct RazerReport {
    status: u8,
    transaction_id: u8,
    remaining_packets: u16,
    protocol_type: u8,
    data_size: u8,
    command_class: u8,
    command_id: u8,
    arguments: [u8; 80],
    crc: u8,
    reserved: u8
}
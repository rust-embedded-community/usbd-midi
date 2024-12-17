//! Example for ESP32-S3.

#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

use esp_backtrace as _;
use esp_println::println;
use usb_device::prelude::*;
use usbd_midi::{
    data::usb_midi::midi_packet_reader::MidiPacketBufferReader, midi_device::MidiClass,
};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[esp_hal::xtensa_lx_rt::entry]
fn main() -> ! {
    let mut config = esp_hal::Config::default();
    config.cpu_clock = esp_hal::clock::CpuClock::Clock240MHz;
    let peripherals = esp_hal::init(config);

    let usb_bus_allocator = esp_hal::otg_fs::UsbBus::new(
        esp_hal::otg_fs::Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19),
        unsafe { &mut *addr_of_mut!(EP_MEMORY) },
    );

    let mut midi_class = MidiClass::new(&usb_bus_allocator, 1, 1).unwrap();

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus_allocator, UsbVidPid(0x16c0, 0x5e4))
        .device_class(0)
        .device_sub_class(0)
        .strings(&[StringDescriptors::default()
            .manufacturer("Music Company")
            .product("MIDI Device")
            .serial_number("12345678")])
        .unwrap()
        .build();

    loop {
        if usb_dev.poll(&mut [&mut midi_class]) {
            let mut buffer = [0; 64];
            if let Ok(size) = midi_class.read(&mut buffer) {
                let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
                for packet in buffer_reader.into_iter() {
                    if let Ok(packet) = packet {
                        println!(
                            "Cable: {:?}, Message: {:?}, Bytes: {:?}",
                            packet.cable_number(),
                            packet.message(),
                            packet.as_message_bytes(),
                        );
                    }
                }
            }
        }
    }
}

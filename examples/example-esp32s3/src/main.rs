//! Example for ESP32-S3.

#![no_std]
#![no_main]

use core::ptr::addr_of_mut;

use esp_backtrace as _;
use esp_hal::{clock, gpio, otg_fs, xtensa_lx_rt, Config};
use esp_println::println;
use heapless::Vec;
use midi_convert::midi_types::{Channel, MidiMessage, Note, Value7};
use midi_convert::{parse::MidiTryParseSlice, render_slice::MidiRenderSlice};
use usb_device::prelude::*;
use usbd_midi::{CableNumber, MidiClass, MidiPacketBufferReader, UsbMidiEventPacket};

static mut EP_MEMORY: [u32; 1024] = [0; 1024];

#[xtensa_lx_rt::entry]
fn main() -> ! {
    let mut config = Config::default();
    config.cpu_clock = clock::CpuClock::Clock240MHz;
    let peripherals = esp_hal::init(config);

    let usb_bus_allocator = otg_fs::UsbBus::new(
        otg_fs::Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19),
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

    let button = gpio::Input::new(peripherals.GPIO0, gpio::Pull::Up);
    let mut last_button_level = button.level();

    let mut sysex_buffer = Vec::<u8, 64>::new();

    loop {
        if usb_dev.poll(&mut [&mut midi_class]) {
            // Receive messages.
            let mut buffer = [0; 64];

            if let Ok(size) = midi_class.read(&mut buffer) {
                let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
                for packet in buffer_reader.into_iter().flatten() {
                    if !packet.is_sysex() {
                        let message = MidiMessage::try_parse_slice(packet.payload_bytes());
                        println!(
                            "Regular Message, cable: {:?}, message: {:?}",
                            packet.cable_number(),
                            message
                        );
                    } else {
                        if packet.is_sysex_start() {
                            println!("SysEx message start");
                            sysex_buffer.clear();
                        }

                        match sysex_buffer.extend_from_slice(packet.payload_bytes()) {
                            Ok(_) => {
                                if packet.is_sysex_end() {
                                    println!("SysEx message end");
                                    println!("Buffered SysEx message: {:?}", sysex_buffer);
                                }
                            }
                            Err(_) => {
                                println!("SysEx buffer overflow.");
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Send messages on button press.
        let button_level = button.level();

        if button_level != last_button_level {
            last_button_level = button_level;

            let mut bytes = [0; 3];

            let message = if button_level == gpio::Level::Low {
                MidiMessage::NoteOn(Channel::C1, Note::C3, Value7::from(100))
            } else {
                MidiMessage::NoteOff(Channel::C1, Note::C3, Value7::from(0))
            };

            message.render_slice(&mut bytes);

            let packet =
                UsbMidiEventPacket::try_from_payload_bytes(CableNumber::Cable0, &bytes).unwrap();
            let result = midi_class.send_packet(packet);

            println!("Send result {:?}", result);
        }
    }
}

# usbd-midi

A simple USB MIDI device class for [usb-device](https://crates.io/crates/usb-device).

Currently this aims to be a very simple implementation, that allows the microcontroller to send or receive MIDI information to/from a host like a desktop computer.

This crate requires the use of a HAL that implements the `usb-device` traits.

## Example

### Receive MIDI

Turn on an LED as long as note C2 is pressed. The example only shows the hardware-independent parts.

```rust ignore
use usb_device::prelude::*;
use usbd_midi::{
    message::{channel::Channel, notes::Note},
    Message,
    MidiClass,
    MidiPacketBufferReader,
};

// Prerequisites, must be setup according to the used board.
let mut led = todo!(); // Must implement `embedded_hal::digital::OutputPin`.
let usb_bus = todo!(); // Must be of type `usb_device::bus::UsbBusAllocator`.

// Create a MIDI class with 1 input and 1 output jack.
let mut midi = MidiClass::new(&usb_bus, 1, 1).unwrap();

let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x5e4))
    .device_class(0)
    .device_sub_class(0)
    .strings(&[StringDescriptors::default()
        .manufacturer("Music Company")
        .product("MIDI Device")
        .serial_number("12345678")])
    .unwrap()
    .build();

loop {
    if !usb_dev.poll(&mut [&mut midi]) {
        continue;
    }

    let mut buffer = [0; 64];

    if let Ok(size) = midi.read(&mut buffer) {
        let buffer_reader = MidiPacketBufferReader::new(&buffer, size);
        for packet in buffer_reader.into_iter() {
            if let Ok(packet) = packet {
                match Message::try_from(&packet).unwrap() {
                    Message::NoteOn(Channel1, Note::C2, ..) => {
                        led.set_low().unwrap();
                    },
                    Message::NoteOff(Channel1, Note::C2, ..) => {
                        led.set_high().unwrap();
                    },
                    _ => {}
                }
            }
        }
    }
}
```

## Using more than one MIDI port

Calling `MidiClass::new(&usb_bus, N, M);` with `N, M >= 1` to provide more
than one input or output port requires the `control-buffer-256` feature of
the `usb-device` crate:

Cargo.toml:

```ignore
usb-device = { version = ">=0.3.2", features = ["control-buffer-256"] }
```

Up to 5 in/out pairs can be used this way until we again run out of buffer
space. Note that exceeding the available buffer space will silently fail
to send the descriptors correctly, no obvious `panic!` will hint the
actual problem.

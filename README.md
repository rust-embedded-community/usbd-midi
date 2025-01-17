# usbd-midi

A USB MIDI device class implementation for [usb-device](https://crates.io/crates/usb-device) based on the [USB Device Class Definition for MIDI Devices](https://www.usb.org/sites/default/files/midi10.pdf) specification.

This class allows the device to exchange MIDI messages with a host like a desktop computer. It requires the use of a driver (e.g. a HAL) that implements the `usb-device` traits.

**NOTE:** only MIDI 1.0 protocol is currently supported.

## Message Types

While the crate focuses on transfer functionality, it provides some basic message types with conversions for convenience. These types are gated behind a `message-types` feature, which is enabled by default.

For more complex use cases, it is recommended to use a specialized crate like [midi-types](https://crates.io/crates/midi-types) or [wmidi](https://crates.io/crates/wmidi) and interface with it by using the raw event packet bytes. The [ESP32-S3 example](examples/example-esp32s3/) shows how to do this in detail.

## Examples

The example below shows some basic usage without any platform-dependent parts. Please refer to the [examples](examples/) directory for code that can be run on real hardware.

### Receive MIDI

Turn on an LED as long as note C2 is pressed.

```rust ignore
use usb_device::prelude::*;
use usbd_midi::{
    message::{Message, Channel, Note},
    UsbMidiClass,
    UsbMidiPacketReader,
};

// Prerequisites, must be setup according to the used board.
let mut led = todo!(); // Must implement `embedded_hal::digital::OutputPin`.
let usb_bus = todo!(); // Must be of type `usb_device::bus::UsbBusAllocator`.

// Create a MIDI class with 1 input and 1 output jack.
let mut midi = UsbMidiClass::new(&usb_bus, 1, 1).unwrap();

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
        let packet_reader = UsbMidiPacketReader::new(&buffer, size);
        for packet in packet_reader.into_iter() {
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

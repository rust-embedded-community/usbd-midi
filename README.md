# usbd-midi

A simple USB MIDI device class for [usb-device](https://crates.io/crates/usb-device).

Currently this aims to be a very simple implementation, that allows the microcontroller to send or receive MIDI information to/from a host like a desktop computer.

This crate requires the use of a HAL that implements the `usb-device` traits.

## Example

### Receive MIDI

Turn on the integrated LED of a STM32 BluePill board as long as C2 is pressed.

```rust
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);
    let mut gpioc = dp.GPIOC.split(&mut rcc.apb2);

    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    led.set_high().unwrap();

    let mut usb_dp = gpioa.pa12.into_push_pull_output(&mut gpioa.crh);

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(&mut gpioa.crh),
    };

    let usb_bus = UsbBus::new(usb);

    let mut midi = MidiClass::new(&usb_bus, 1, 1);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x5e4))
        .product("MIDI Test")
        .device_class(0)
        .device_sub_class(0)
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
                    match packet.message {
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
}
```

## Using more than one MIDI port

Calling `MidiClass::new(&usb_bus, N, M);` with `N, M >= 1` to provide more
than one input or output port requires the `control-buffer-256` feature of
the `usb-device` crate:

Cargo.toml:

```
usb-device = { version = ">=0.3.2", features = ["control-buffer-256"] }
```

Up to 5 in/out pairs can be used this way until we again run out of buffer
space. Note that exceeding the available buffer space will silently fail
to send the descriptors correctly, no obvious `panic!` will hint the
actual problem.

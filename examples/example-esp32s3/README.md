# ESP32-S3 Example

This example was developed and tested on an [ESP32-S3-DevKitC-1](https://docs.espressif.com/projects/esp-dev-kits/en/latest/esp32s3/esp32-s3-devkitc-1/index.html) using the [esp-hal crate](https://crates.io/crates/esp-hal).

It features:

- Sending and receiving of regular MIDI messages.
- Sending and receiving of MIDI System Exclusive messages with buffering.
- Conversion of USB MIDI packets from and to types provided by the [midi-types crate](https://crates.io/crates/midi-types).

It does not provide a fully production-ready setup, especially time-critical tasks like polling the USB bus in an interrupt and managing bus timeouts are out of scope of this example.

## Requirements

To build the example, an installed toolchain for the Xtensa target is required. Please refer to the [Rust on ESP book](https://docs.esp-rs.org/book/) for further instructions.

You can build the example by running:

    cargo build --release

If [espflash](https://crates.io/crates/espflash) is installed, you can flash the example to the board and run it:

    cargo run --release

## Functionality

- Incoming MIDI messages are logged to the console.
- Pressing and releasing the *BOOT* button on the board sends MIDI messages.
- A received *Device Inquiry* SysEx request is responded to the host.

Please note that all chosen vendor and product ids and names are just for demonstration purposes and should not be used with a real product.

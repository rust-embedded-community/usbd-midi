# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `UsbMidiEventPacket::cable_number` function.
- `UsbMidiEventPacket::as_message_bytes` function.
- `UsbMidiEventPacket::as_raw_bytes` function.
- `UsbMidiEventPacket::to_raw_bytes` function.
- `UsbMidiEventPacket::try_from_message_bytes` function.
- `TryFrom<&UsbMidiEventPacket>` implementation for `Message` type.
- `Message::into_packet` function.
- `Message::code_index_number` function.
- Re-exports of common items in crate root.

### Changed

- Renamed `MidiClass::send_message` function to `MidiClass::send_packet`.
- Renamed `midi_device` module to `class`.
- Renamed `usb_midi` module to `packet` and moved it into crate root.
- Renamed `midi_packet_reader` module to `reader`.
- Renamed `usb_midi_event_packet` module to `event_packet`.
- Moved `channel` and `notes` modules into `message` module.
- Moved `message` module to crate root.
- Moved `byte` submodules into `data` module.
- Moved usb descriptor constants into class module and made them private.

### Removed

- `UsbMidiEventPacket::cable_number` field, use function instead.
- `UsbMidiEventPacket::message` field, use `Message::try_from(&UsbMidiEventPacket)` instead.
- `UsbMidiEventPacket::from_midi` function, use `Message::into_packet` instead.
- `CodeIndexNumber::find_from_message` function, use `Message::code_index_number` instead.

## [0.3.0] - 2024-05-27

### Changed

- Updated `usb-device` dependency to 0.3.
- Updated `num_enum` dependency to 0.7.2.
- Extended endpoint descriptors to 9 bytes as stated in specification.

### Removed

- Removed unused `embedded-hal` and `nb` dependencies.

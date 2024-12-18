# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `UsbMidiEventPacket::cable_number` function.
- `UsbMidiEventPacket::message` function.
- `UsbMidiEventPacket::as_message_bytes` function.
- `UsbMidiEventPacket::as_raw_bytes` function.
- `UsbMidiEventPacket::to_raw_bytes` function.
- `UsbMidiEventPacket::from_message_bytes` function.

### Changed

- Renamed `UsbMidiEventPacket::from_midi` function to `UsbMidiEventPacket::from_message`.

### Removed

- `UsbMidiEventPacket::cable_number` field, use function instead.
- `UsbMidiEventPacket::message` field, use function instead.

## [0.3.0] - 2024-05-27

### Changed

- Updated `usb-device` dependency to 0.3.
- Updated `num_enum` dependency to 0.7.2.
- Extended endpoint descriptors to 9 bytes as stated in specification.

### Removed

- Removed unused `embedded-hal` and `nb` dependencies.
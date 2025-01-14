# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

This release focuses on improving the internal message data types and their usage.

### Added

- `Message` enum variants for *System Common* and *System Realtime* messages.
- `U14` primitive value type used by *Pitch Wheel* and *Song Position Pointer* messages.
- Derive `Debug`, `Clone`, `Eq`, and `PartialEq` for `U4`.
- Derive `Debug`, `Clone`, `Eq`, and `PartialEq` for `InvalidU4`.
- Derive `Debug`, `Clone`, `Eq`, and `PartialEq` for `InvalidU7`.
- Derive `Debug`, `Clone`, `Eq`, and `PartialEq` for `Payload`.
- Derive `Debug`, `Clone`, `Eq`, and `PartialEq` for `Raw`.
- Re-exports of common data types in the `message` module.

### Changed

- Changed pitch wheel `Message` variant from `PitchWheelChange(Channel, U7, U7)` to `PitchWheelChange(Channel, U14)`.

## [0.4.0] - 2025-01-03

This release focuses on:

- Increased usability by simplifying the internal module structure.
- Interfacing with third-party crates like `midi-types`.
- Support for System Exclusive messages (SysEx).

**Important:**

- The `message` module containing the `Message` struct and related types is now gated behind the `message-types` feature. This feature is enabled by default.
- The constants `USB_AUDIO_CLASS`, `USB_AUDIOCONTROL_SUBCLASS` and `USB_MIDISTREAMING_SUBCLASS` are now private to prevent them from being used in the device setup. Doing so would lead to incorrect enumeration on certain hosts (e.g. macOS).

### Added

- `UsbMidiEventPacket::cable_number` function.
- `UsbMidiEventPacket::header` function.
- `UsbMidiEventPacket::payload_bytes` function.
- `UsbMidiEventPacket::as_raw_bytes` function.
- `UsbMidiEventPacket::to_raw_bytes` function.
- `UsbMidiEventPacket::try_from_payload_bytes` function.
- `UsbMidiEventPacket::is_sysex` function.
- `UsbMidiEventPacket::is_sysex_start` function.
- `UsbMidiEventPacket::is_sysex_end` function.
- `TryFrom<&UsbMidiEventPacket>` implementation for `Message` type.
- `Message::into_packet` function.
- `Message::code_index_number` function.
- `CodeIndexNumber::try_from_payload` function.
- `CodeIndexNumber::payload_size` function.
- `CableNumber::Cable0` as default value.
- `FromOverFlow<u8> for U4` implementation.
- `FromClamped<u8> for U4` implementation.
- Re-exports of common items in crate root.

### Changed

- Renamed `MidiClass` to `UsbMidiClass`.
- Renamed `UsbMidiClass::send_message` function to `UsbMidiClass::send_packet`.
- Renamed `midi_device` module to `class`.
- Renamed `usb_midi` module to `packet` and moved it into crate root.
- Renamed `midi_packet_reader` module to `reader`.
- Renamed `MidiPacketBufferReader` to `UsbMidiPacketReader`.
- Renamed `MidiPacketParsingError` to `UsbMidiEventPacketError`
- Renamed `MidiReadError` to `UsbMidiReadError`
- Moved `usb_midi_event_packet` code into parent `packet` module.
- Moved `channel` and `notes` modules into `message` module.
- Moved `message` module to crate root.
- Moved `byte` submodules into `message::data` module.
- Moved `from_traits` code into parent `data` module.
- Consolidated separate `InvalidCableNumber` struct into `UsbMidiEventPacketError`.
- Consolidated separate `InvalidCodeIndexNumber` struct into `UsbMidiEventPacketError`.
- Converted `CodeIndexNumber` struct to enum.
- Moved descriptor constants into class module and made them private.
- Set edition to 2021.

### Removed

- `UsbMidiEventPacket::cable_number` field, use function instead.
- `UsbMidiEventPacket::message` field, use `Message::try_from(&UsbMidiEventPacket)` instead.
- `UsbMidiEventPacket::from_midi` function, use `Message::into_packet` instead.
- `CodeIndexNumber::find_from_message` function, use `Message::code_index_number` instead.
- `From<CableNumber> for U4` implementation.
- `From<CodeIndexNumber> for U4` implementation.

## [0.3.0] - 2024-05-27

### Changed

- Updated `usb-device` dependency to 0.3.
- Updated `num_enum` dependency to 0.7.2.
- Extended endpoint descriptors to 9 bytes as stated in specification.

### Removed

- Removed unused `embedded-hal` and `nb` dependencies.

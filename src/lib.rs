//! USB MIDI class implementation for [usb-device](https://crates.io/crates/usb-device).

#![no_std]

pub mod data;
pub mod midi_device;

pub use crate::data::usb_midi::cable_number::CableNumber;
pub use crate::data::usb_midi::midi_packet_reader::MidiPacketBufferReader;
pub use crate::data::usb_midi::usb_midi_event_packet::UsbMidiEventPacket;
pub use crate::midi_device::MidiClass;

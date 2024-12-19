//! USB MIDI class implementation for [usb-device](https://crates.io/crates/usb-device).

#![no_std]

pub mod class;
pub mod data;

pub use crate::class::MidiClass;
pub use crate::data::packet::cable_number::CableNumber;
pub use crate::data::packet::midi_packet_reader::MidiPacketBufferReader;
pub use crate::data::packet::usb_midi_event_packet::UsbMidiEventPacket;

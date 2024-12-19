//! USB MIDI class implementation for [usb-device](https://crates.io/crates/usb-device).

#![no_std]

pub mod class;
pub mod data;
pub mod packet;

pub use crate::class::MidiClass;
pub use crate::packet::cable_number::CableNumber;
pub use crate::packet::reader::MidiPacketBufferReader;
pub use crate::packet::usb_midi_event_packet::UsbMidiEventPacket;

//! USB MIDI class implementation for [usb-device](https://crates.io/crates/usb-device).

#![no_std]

pub mod class;
pub mod data;
pub mod message;
pub mod packet;

pub use crate::class::MidiClass;
pub use crate::message::Message;
pub use crate::packet::cable_number::CableNumber;
pub use crate::packet::event_packet::UsbMidiEventPacket;
pub use crate::packet::reader::MidiPacketBufferReader;

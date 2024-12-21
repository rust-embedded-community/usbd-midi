//! USB MIDI class implementation for [usb-device](https://crates.io/crates/usb-device).

#![no_std]

pub mod class;
pub mod data;
pub mod packet;

#[cfg(feature = "message-types")]
pub mod message;

pub use crate::class::MidiClass;
pub use crate::packet::cable_number::CableNumber;
pub use crate::packet::reader::MidiPacketBufferReader;
pub use crate::packet::UsbMidiEventPacket;

#[cfg(feature = "message-types")]
pub use crate::message::Message;

#![doc = include_str!("../README.md")]
#![no_std]

pub mod class;
pub mod packet;

#[cfg(feature = "message-types")]
pub mod message;

pub use crate::class::{MidiClass, MidiReadError};
pub use crate::packet::cable_number::CableNumber;
pub use crate::packet::reader::UsbMidiPacketReader;
pub use crate::packet::{MidiPacketParsingError, UsbMidiEventPacket};

#[cfg(feature = "message-types")]
pub use crate::message::Message;

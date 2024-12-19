//! Representation of a USB MIDI event packet.

use core::convert::{TryFrom, TryInto};

use crate::data::u4::U4;
use crate::message::raw::{Payload, Raw};
use crate::message::Message;
use crate::packet::cable_number::CableNumber;
use crate::packet::code_index_number::CodeIndexNumber;

/// A packet that communicates with the host.
///
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UsbMidiEventPacket {
    /// Raw packet data.
    raw: [u8; 4],
}

impl From<UsbMidiEventPacket> for [u8; 4] {
    fn from(value: UsbMidiEventPacket) -> [u8; 4] {
        let cable_number = U4::from(value.cable_number());
        let message = value.message();
        let index_number = {
            let code_index = CodeIndexNumber::find_from_message(&message);
            U4::from(code_index)
        };
        let header = U4::combine(cable_number, index_number);

        let raw_midi = Raw::from(message);
        let status = raw_midi.status;

        match raw_midi.payload {
            Payload::Empty => [header, status, 0, 0],
            Payload::SingleByte(byte) => [header, status, byte.into(), 0],
            Payload::DoubleByte(byte1, byte2) => [header, status, byte1.into(), byte2.into()],
        }
    }
}

/// Error variants for parsing the packet.
#[derive(Debug)]
pub enum MidiPacketParsingError {
    /// Invalid packet.
    InvalidPacket,
    /// Invalid note.
    InvalidNote(u8),
    /// Invalid cable number.
    InvalidCableNumber(u8),
    /// Invalid event type.
    InvalidEventType(u8),
    /// Missing data packet.
    MissingDataPacket,
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = MidiPacketParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let Ok(raw) = value.try_into() else {
            return Err(MidiPacketParsingError::InvalidPacket);
        };

        Ok(UsbMidiEventPacket { raw })
    }
}

impl UsbMidiEventPacket {
    /// Returns the cable number.
    pub fn cable_number(&self) -> CableNumber {
        let raw_cable_number = self.raw.first().unwrap() >> 4;

        CableNumber::try_from(raw_cable_number).unwrap()
    }

    /// Returns the message.
    pub fn message(&self) -> Message {
        Message::try_from(&self.raw[1..]).unwrap()
    }

    /// Returns a slice to the message bytes. The length is dependent on the message type.
    pub fn as_message_bytes(&self) -> &[u8] {
        let r = Raw::from(self.message());
        let length = match r.payload {
            Payload::Empty => 1,
            Payload::SingleByte(_) => 2,
            Payload::DoubleByte(_, _) => 3,
        };

        &self.raw[1..1 + length]
    }

    /// Returns a reference to the raw bytes.
    pub fn as_raw_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Returns the raw bytes as owned array.
    pub fn to_raw_bytes(&self) -> [u8; 4] {
        self.raw.clone()
    }

    /// Creates a packet from a message and returns it.
    pub fn from_message(cable: CableNumber, message: Message) -> Self {
        message.into_packet(cable)
    }

    /// Creates a packet from a slice of message bytes.
    pub fn from_message_bytes(
        cable: CableNumber,
        bytes: &[u8],
    ) -> Result<Self, MidiPacketParsingError> {
        let message = Message::try_from(bytes)?;

        Ok(Self::from_message(cable, message))
    }
}

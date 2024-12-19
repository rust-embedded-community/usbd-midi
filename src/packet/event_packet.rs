//! Representation of a USB MIDI event packet.

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
        value.raw
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
    /// Invalid code index number.
    InvalidCodeIndexNumber(u8),
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
        let raw_cable_number = self.raw[0] >> 4;

        CableNumber::try_from(raw_cable_number).unwrap()
    }

    /// Returns a slice to the message bytes. The length is dependent on the message type.
    pub fn as_message_bytes(&self) -> &[u8] {
        let cin = CodeIndexNumber::try_from(self.raw[0] & 0x0F).unwrap();
        let size = cin.event_size();

        &self.raw[1..1 + size]
    }

    /// Returns a reference to the raw bytes.
    pub fn as_raw_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Returns the raw bytes as owned array.
    pub fn to_raw_bytes(&self) -> [u8; 4] {
        self.raw
    }

    /// Creates a packet from a slice of message bytes.
    pub fn try_from_message_bytes(
        cable: CableNumber,
        bytes: &[u8],
    ) -> Result<Self, MidiPacketParsingError> {
        let message = Message::try_from(bytes)?;

        Ok(message.into_packet(cable))
    }
}

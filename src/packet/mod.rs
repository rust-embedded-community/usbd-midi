//! USB MIDI event packet parser and related types.

pub mod cable_number;
pub mod code_index_number;
pub mod reader;

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
    /// Empty event.
    EmptyEvent,
    /// Invalid event status.
    InvalidEventStatus,
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = MidiPacketParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let Ok(raw) = value.try_into() else {
            return Err(MidiPacketParsingError::InvalidPacket);
        };

        Ok(Self { raw })
    }
}

impl UsbMidiEventPacket {
    /// Returns the cable number.
    pub fn cable_number(&self) -> CableNumber {
        let raw_cable_number = self.raw[0] >> 4;

        // Unwrap can't fail because of limited `raw_cable_number` value range.
        CableNumber::try_from(raw_cable_number).unwrap_or_default()
    }

    /// Returns a slice to the message bytes. The length is dependent on the message type.
    pub fn as_message_bytes(&self) -> &[u8] {
        let raw_cin = self.raw[0] & 0x0F;

        match CodeIndexNumber::try_from(raw_cin) {
            Ok(cin) => {
                let size = cin.event_size();
                &self.raw[1..1 + size]
            }

            // Can't happen because of limited `raw_cin` value range.
            Err(_) => &[],
        }
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
        let cin = CodeIndexNumber::try_from_event(bytes)?;
        let event_size = cin.event_size();

        let mut raw = [0; 4];
        raw[0] = (cable as u8) << 4 | cin as u8;
        raw[1..1 + event_size].copy_from_slice(&bytes[..event_size]);

        Ok(Self { raw })
    }
}

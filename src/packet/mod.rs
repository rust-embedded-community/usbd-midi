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
#[derive(Debug, Clone, Eq, PartialEq)]
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
    /// Empty payload.
    EmptyPayload,
    /// Invalid payload status.
    InvalidPayloadStatus,
    /// Invalid payload size.
    InvalidPayloadSize,
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

    /// Returns the header byte.
    pub fn header(&self) -> u8 {
        self.raw[0]
    }

    /// Returns a slice to the event payload bytes. The length is dependent on the payload type.
    pub fn payload_bytes(&self) -> &[u8] {
        let raw_cin = self.raw[0] & 0x0F;

        match CodeIndexNumber::try_from(raw_cin) {
            Ok(cin) => {
                let size = cin.payload_size();
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

    /// Creates a packet from a slice of event payload bytes.
    pub fn try_from_payload_bytes(
        cable: CableNumber,
        bytes: &[u8],
    ) -> Result<Self, MidiPacketParsingError> {
        let cin = CodeIndexNumber::try_from_payload(bytes)?;
        let payload_size = cin.payload_size();

        if bytes.len() < payload_size {
            return Err(MidiPacketParsingError::InvalidPayloadSize);
        }

        let mut raw = [0; 4];
        raw[0] = (cable as u8) << 4 | cin as u8;
        raw[1..1 + payload_size].copy_from_slice(&bytes[..payload_size]);

        Ok(Self { raw })
    }

    /// Returns if the packet payload is part of a SysEx message.
    pub fn is_sysex(&self) -> bool {
        let Ok(cin) = CodeIndexNumber::try_from(self.raw[0] & 0x0F) else {
            return false;
        };

        match cin {
            CodeIndexNumber::SysexStartsOrContinues
            | CodeIndexNumber::SysexEnds2Bytes
            | CodeIndexNumber::SysexEnds3Bytes => true,
            CodeIndexNumber::SystemCommon1Byte => self.raw[1] == 0xF7,
            CodeIndexNumber::SingleByte => self.raw[1] < 0x80,
            _ => false,
        }
    }

    /// Returns if the packet payload contains the start of a SysEx message.
    pub fn is_sysex_start(&self) -> bool {
        self.is_sysex() && self.raw[1] == 0xF0
    }

    /// Returns if the packet payload contains the end of a SysEx message.
    pub fn is_sysex_end(&self) -> bool {
        self.is_sysex() && (self.raw[1] == 0xF7 || self.raw[2] == 0xF7 || self.raw[3] == 0xF7)
    }
}

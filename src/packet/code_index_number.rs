//! Enum representing the code index number of a packet.

use crate::data::u4::U4;
use crate::packet::event_packet::MidiPacketParsingError;

/// The Code Index Number(CIN) indicates the classification
/// of the bytes in the MIDI_x fields.
/// Code Index Number classifications.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum CodeIndexNumber {
    /// Miscellaneous function codes. Reserved for future extensions.
    MiscFunction = 0x00,
    /// Cable events. Reserved for future expansion.
    CableEvents = 0x1,
    /// Two-byte System Common messages like MTC, SongSelect, etc.
    SystemCommon2Bytes = 0x2,
    /// Three-byte System Common messages like SPP, etc.
    SystemCommon3Bytes = 0x3,
    /// SysEx starts or continues.
    SysexStartsOrContinues = 0x4,
    /// Single-byte System Common Message or SysEx ends with following single byte.
    SystemCommon1Byte = 0x5,
    /// SysEx ends with following two bytes.
    SysexEnds2Bytes = 0x6,
    /// SysEx ends with following three bytes.
    SysexEnds3Bytes = 0x7,
    /// Note-off.
    NoteOff = 0x8,
    /// Note-on.
    NoteOn = 0x9,
    /// Poly-KeyPress.
    PolyKeyPress = 0xA,
    /// Control Change.
    ControlChange = 0xB,
    /// Program Change.
    ProgramChange = 0xC,
    /// Channel Pressure.
    ChannelPressure = 0xD,
    /// PitchBend Change.
    PitchBendChange = 0xE,
    /// Single Byte.
    SingleByte = 0xF,
}

impl TryFrom<u8> for CodeIndexNumber {
    type Error = MidiPacketParsingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == CodeIndexNumber::MiscFunction as u8 => Ok(CodeIndexNumber::MiscFunction),
            x if x == CodeIndexNumber::CableEvents as u8 => Ok(CodeIndexNumber::CableEvents),
            x if x == CodeIndexNumber::SystemCommon2Bytes as u8 => {
                Ok(CodeIndexNumber::SystemCommon2Bytes)
            }
            x if x == CodeIndexNumber::SystemCommon3Bytes as u8 => {
                Ok(CodeIndexNumber::SystemCommon3Bytes)
            }
            x if x == CodeIndexNumber::SysexStartsOrContinues as u8 => {
                Ok(CodeIndexNumber::SysexStartsOrContinues)
            }
            x if x == CodeIndexNumber::SystemCommon1Byte as u8 => {
                Ok(CodeIndexNumber::SystemCommon1Byte)
            }
            x if x == CodeIndexNumber::SysexEnds2Bytes as u8 => {
                Ok(CodeIndexNumber::SysexEnds2Bytes)
            }
            x if x == CodeIndexNumber::SysexEnds3Bytes as u8 => {
                Ok(CodeIndexNumber::SysexEnds3Bytes)
            }
            x if x == CodeIndexNumber::NoteOff as u8 => Ok(CodeIndexNumber::NoteOff),
            x if x == CodeIndexNumber::NoteOn as u8 => Ok(CodeIndexNumber::NoteOn),
            x if x == CodeIndexNumber::PolyKeyPress as u8 => Ok(CodeIndexNumber::PolyKeyPress),
            x if x == CodeIndexNumber::ControlChange as u8 => Ok(CodeIndexNumber::ControlChange),
            x if x == CodeIndexNumber::ProgramChange as u8 => Ok(CodeIndexNumber::ProgramChange),
            x if x == CodeIndexNumber::ChannelPressure as u8 => {
                Ok(CodeIndexNumber::ChannelPressure)
            }
            x if x == CodeIndexNumber::PitchBendChange as u8 => {
                Ok(CodeIndexNumber::PitchBendChange)
            }
            x if x == CodeIndexNumber::SingleByte as u8 => Ok(CodeIndexNumber::SingleByte),
            _ => Err(MidiPacketParsingError::InvalidCodeIndexNumber(value)),
        }
    }
}

impl From<CodeIndexNumber> for U4 {
    fn from(value: CodeIndexNumber) -> U4 {
        U4::from_overflowing_u8(value as u8)
    }
}

impl CodeIndexNumber {
    /// Returns the size of the MIDI_x event in bytes.
    pub fn event_size(&self) -> usize {
        match self {
            Self::SystemCommon1Byte | Self::SingleByte => 1,
            Self::SystemCommon2Bytes
            | Self::SysexEnds2Bytes
            | Self::ProgramChange
            | Self::ChannelPressure => 2,
            Self::SystemCommon3Bytes
            | Self::SysexEnds3Bytes
            | Self::SysexStartsOrContinues
            | Self::NoteOff
            | Self::NoteOn
            | Self::PolyKeyPress
            | Self::ControlChange
            | Self::PitchBendChange => 3,

            // These variants are reserved for future use.
            // We assume the maximum length of 3 bytes so that no data can get lost.
            Self::MiscFunction | Self::CableEvents => 3,
        }
    }
}

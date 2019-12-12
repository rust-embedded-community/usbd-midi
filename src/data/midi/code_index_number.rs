use crate::util::try_from::{TryFrom};

/// The Code Index Number(CIN) indicates the classification 
/// of the bytes in the MIDI_x fields
pub struct CodeIndexNumber(u8);
 
impl TryFrom<u8> for CodeIndexNumber {

    fn try_from(value:u8) -> Option<Self> {
        if value > 0xF {
            None
        } else {
            Some(CodeIndexNumber(value))
        }
    }
}

impl Into<u8> for CodeIndexNumber {
    fn into(self) -> u8 {
        self.0
    }
}

impl CodeIndexNumber {

    /// Miscellaneous function codes. Reserved for future extensions
    pub const MISC_FUNCTION : CodeIndexNumber = CodeIndexNumber(0x00);
    /// Cable events. Reserved for future expansion.
    pub const CABLE_EVENTS : CodeIndexNumber = CodeIndexNumber(0x1);
    /// Two-byte System Common messages like MTC, SongSelect, etc.
    pub const SYSTEM_COMMON_LEN2 :CodeIndexNumber = CodeIndexNumber(0x2);
    /// Three-byte System Common messages like SPP, etc.
    pub const SYSTEM_COMMON_LEN3 :CodeIndexNumber = CodeIndexNumber(0x3);
    /// SysEx starts or continues
    pub const SYSEX_STARTS : CodeIndexNumber = CodeIndexNumber(0x4);
    pub const SYSEX_CONTINUES : CodeIndexNumber = CodeIndexNumber::SYSEX_STARTS;
    /// Single-byte System Common Message or SysEx ends with following single byte.
    pub const SYSTEM_COMMON_LEN1 : CodeIndexNumber= CodeIndexNumber(0x5);
    /// SysEx ends with the following byte
    pub const SYSEX_ENDS_NEXT1 :CodeIndexNumber = CodeIndexNumber::SYSTEM_COMMON_LEN1;
    /// SysEx ends with following two bytes
    pub const SYSEX_ENDS_NEXT2 : CodeIndexNumber = CodeIndexNumber(0x6);
    /// SysEx ends with following three bytes
    pub const SYSEX_ENDS_NEXT3 : CodeIndexNumber = CodeIndexNumber(0x7);
    /// Note - Off
    pub const NOTE_OFF : CodeIndexNumber = CodeIndexNumber(0x8);
    /// Note - On
    pub const NOTE_ON : CodeIndexNumber = CodeIndexNumber(0x9);
    /// Poly-KeyPress
    pub const POLY_KEYPRESS : CodeIndexNumber = CodeIndexNumber(0xA);
    /// Control Change
    pub const CONTROL_CHANGE : CodeIndexNumber = CodeIndexNumber(0xB);
    /// Program Change
    pub const PROGRAM_CHANGE : CodeIndexNumber = CodeIndexNumber(0xC);
    /// Channel Pressure
    pub const CHANNEL_PRESSURE : CodeIndexNumber = CodeIndexNumber(0xD);
    /// Pitch Bend Change
    pub const PITCHBEND_CHANGE : CodeIndexNumber = CodeIndexNumber(0xE);
    /// Single Byte
    pub const SINGLE_BYTE : CodeIndexNumber= CodeIndexNumber(0xF);
 
}

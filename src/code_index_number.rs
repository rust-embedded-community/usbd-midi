//! The Code Index Number(CIN) indicates the classification
//! of the bytes in the MIDI_x fields.
//!
//! # Note
//! These numbers are all pre-shifted and ready to be ORed with the cable number nibble.
use crate::midi_types::MidiMessage;

/// Miscellaneous function codes. Reserved for future extensions
pub const MISC_FUNCTION: u8 = 0x00;
/// Cable events. Reserved for future expansion.
pub const CABLE_EVENTS: u8 = 0x10;
/// Two-byte System Common messages like MTC, SongSelect, etc.
pub const SYSTEM_COMMON_LEN2: u8 = 0x20;
/// Three-byte System Common messages like SPP, etc.
pub const SYSTEM_COMMON_LEN3: u8 = 0x30;
/// SysEx starts or continues
pub const SYSEX_STARTS: u8 = 0x40;
pub const SYSEX_CONTINUES: u8 = SYSEX_STARTS;
/// Single-byte System Common Message or SysEx ends with following single byte.
pub const SYSTEM_COMMON_LEN1: u8 = 0x50;
/// SysEx ends with the following byte
pub const SYSEX_ENDS_NEXT1: u8 = SYSTEM_COMMON_LEN1;
/// SysEx ends with following two bytes
pub const SYSEX_ENDS_NEXT2: u8 = 0x60;
/// SysEx ends with following three bytes
pub const SYSEX_ENDS_NEXT3: u8 = 0x70;
/// Note - Off
pub const NOTE_OFF: u8 = 0x80;
/// Note - On
pub const NOTE_ON: u8 = 0x90;
/// Poly-KeyPress
pub const POLY_KEYPRESS: u8 = 0xA0;
/// Control Change
pub const CONTROL_CHANGE: u8 = 0xB0;
/// Program Change
pub const PROGRAM_CHANGE: u8 = 0xC0;
/// Channel Pressure
pub const CHANNEL_PRESSURE: u8 = 0xD0;
/// Pitch Bend Change
pub const PITCHBEND_CHANGE: u8 = 0xE0;
/// Single Byte
pub const SINGLE_BYTE: u8 = 0xF0;

/// Find the appropriate Code Index Number from a `MidiMessage`
pub fn find_from_message(value: &MidiMessage) -> u8 {
    match value {
        MidiMessage::NoteOn(_, _, _) => NOTE_ON,
        MidiMessage::NoteOff(_, _, _) => NOTE_OFF,
        MidiMessage::ChannelPressure(_, _) => CHANNEL_PRESSURE,
        MidiMessage::PitchBendChange(_, _) => PITCHBEND_CHANGE,
        MidiMessage::KeyPressure(_, _, _) => POLY_KEYPRESS,
        MidiMessage::ProgramChange(_, _) => PROGRAM_CHANGE,
        MidiMessage::ControlChange(_, _, _) => CONTROL_CHANGE,

        MidiMessage::QuarterFrame(_) | MidiMessage::SongSelect(_) => SYSTEM_COMMON_LEN2,

        MidiMessage::SongPositionPointer(_) => SYSTEM_COMMON_LEN3,

        MidiMessage::TuneRequest
        | MidiMessage::TimingClock
        | MidiMessage::Start
        | MidiMessage::Continue
        | MidiMessage::Stop
        | MidiMessage::ActiveSensing
        | MidiMessage::Reset => SYSTEM_COMMON_LEN1,
    }
}

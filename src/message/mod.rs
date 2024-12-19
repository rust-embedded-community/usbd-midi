//! Enum representing the MIDI message.

pub mod channel;
pub mod control_function;
pub mod notes;
pub mod raw;

use core::convert::TryFrom;

use crate::data::from_traits::FromClamped;
use crate::data::u4::U4;
use crate::data::u7::U7;
use crate::message::channel::Channel;
use crate::message::control_function::ControlFunction;
use crate::message::notes::Note;
use crate::message::raw::{Payload, Raw};
use crate::packet::cable_number::CableNumber;
use crate::packet::code_index_number::CodeIndexNumber;
use crate::packet::event_packet::{MidiPacketParsingError, UsbMidiEventPacket};

type Velocity = U7;

/// Represents midi messages.
///
/// Note: not current exhaustive and SysEx messages end up
/// being a confusing case. So are currently note implemented
/// they are sort-of unbounded
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Message {
    /// Note On message.
    NoteOff(Channel, Note, Velocity),
    /// Note Off message.
    NoteOn(Channel, Note, Velocity),
    /// Polyphonic aftertouch (poly-pressure) message.
    PolyphonicAftertouch(Channel, Note, U7),
    /// Program change message.
    ProgramChange(Channel, U7),
    /// Channel aftertouch message.
    ChannelAftertouch(Channel, U7),
    /// Pitchwheel message.
    PitchWheelChange(Channel, U7, U7),
    /// Control Change (CC) message.
    ControlChange(Channel, ControlFunction, U7),
}

const NOTE_OFF_MASK: u8 = 0b1000_0000;
const NOTE_ON_MASK: u8 = 0b1001_0000;
const POLYPHONIC_MASK: u8 = 0b1010_0000;
const PROGRAM_MASK: u8 = 0b1100_0000;
const CHANNEL_AFTERTOUCH_MASK: u8 = 0b1101_0000;
const PITCH_BEND_MASK: u8 = 0b1110_0000;
const CONTROL_CHANGE_MASK: u8 = 0b1011_0000;

impl From<Message> for Raw {
    fn from(value: Message) -> Raw {
        match value {
            Message::NoteOn(chan, note, vel) => {
                let payload = Payload::DoubleByte(note.into(), vel);
                let status = NOTE_ON_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::NoteOff(chan, note, vel) => {
                let payload = Payload::DoubleByte(note.into(), vel);
                let status = NOTE_OFF_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::PolyphonicAftertouch(chan, note, pressure) => {
                let payload = Payload::DoubleByte(note.into(), pressure);
                let status = POLYPHONIC_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::ProgramChange(chan, program) => {
                let payload = Payload::SingleByte(program);
                let status = PROGRAM_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::ChannelAftertouch(chan, pressure) => {
                let payload = Payload::SingleByte(pressure);
                let status = CHANNEL_AFTERTOUCH_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::PitchWheelChange(chan, lsb, msb) => {
                let payload = Payload::DoubleByte(lsb, msb);
                let status = PITCH_BEND_MASK | u8::from(chan);
                Raw { status, payload }
            }
            Message::ControlChange(chan, control_function, value) => {
                let payload = Payload::DoubleByte(control_function.0, value);
                let status = CONTROL_CHANGE_MASK | u8::from(chan);
                Raw { status, payload }
            }
        }
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = MidiPacketParsingError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let status_byte = match data.first() {
            Some(byte) => byte,
            None => return Err(MidiPacketParsingError::MissingDataPacket),
        };

        let event_type = status_byte & 0b1111_0000;
        let channel_bytes = (status_byte) & 0b0000_1111;

        let channel = Channel::try_from(channel_bytes).ok().unwrap();

        match event_type {
            NOTE_ON_MASK => Ok(Message::NoteOn(
                channel,
                get_note(data)?,
                get_u7_at(data, 2)?,
            )),
            NOTE_OFF_MASK => Ok(Message::NoteOff(
                channel,
                get_note(data)?,
                get_u7_at(data, 2)?,
            )),
            POLYPHONIC_MASK => Ok(Message::PolyphonicAftertouch(
                channel,
                get_note(data)?,
                get_u7_at(data, 2)?,
            )),
            PROGRAM_MASK => Ok(Message::ProgramChange(channel, get_u7_at(data, 1)?)),
            CHANNEL_AFTERTOUCH_MASK => Ok(Message::ChannelAftertouch(channel, get_u7_at(data, 1)?)),
            PITCH_BEND_MASK => Ok(Message::PitchWheelChange(
                channel,
                get_u7_at(data, 1)?,
                get_u7_at(data, 2)?,
            )),
            CONTROL_CHANGE_MASK => Ok(Message::ControlChange(
                channel,
                ControlFunction(get_u7_at(data, 1)?),
                get_u7_at(data, 2)?,
            )),
            _ => Err(MidiPacketParsingError::InvalidEventType(event_type)),
        }
    }
}

impl TryFrom<&UsbMidiEventPacket> for Message {
    type Error = MidiPacketParsingError;

    fn try_from(value: &UsbMidiEventPacket) -> Result<Self, Self::Error> {
        Self::try_from(&value.as_raw_bytes()[1..])
    }
}

impl Message {
    /// Create a packet from the message.
    pub fn into_packet(self, cable: CableNumber) -> UsbMidiEventPacket {
        let cin = u8::from(U4::from(CodeIndexNumber::find_from_message(&self)));

        let mut raw = [0; 4];
        raw[0] = (cable as u8) << 4 | cin;
        let r = Raw::from(self);
        raw[1] = r.status;

        match r.payload {
            Payload::Empty => {
                raw[2] = 0;
                raw[3] = 0;
            }
            Payload::SingleByte(byte) => {
                raw[2] = byte.0;
                raw[3] = 0;
            }
            Payload::DoubleByte(byte1, byte2) => {
                raw[2] = byte1.0;
                raw[3] = byte2.0;
            }
        };

        UsbMidiEventPacket::try_from(raw.as_slice()).unwrap()
    }
}

fn get_note(data: &[u8]) -> Result<Note, MidiPacketParsingError> {
    let note_byte = get_byte_at_position(data, 1)?;
    match Note::try_from(note_byte) {
        Ok(note) => Ok(note),
        Err(_) => Err(MidiPacketParsingError::InvalidNote(note_byte)),
    }
}

fn get_u7_at(data: &[u8], index: usize) -> Result<U7, MidiPacketParsingError> {
    let data_byte = get_byte_at_position(data, index)?;
    Ok(U7::from_clamped(data_byte))
}

fn get_byte_at_position(data: &[u8], index: usize) -> Result<u8, MidiPacketParsingError> {
    match data.get(index) {
        Some(byte) => Ok(*byte),
        None => Err(MidiPacketParsingError::MissingDataPacket),
    }
}
//! Enum representing the MIDI message.

pub mod channel;
pub mod control_function;
pub mod data;
pub mod notes;
pub mod raw;

use crate::message::channel::Channel;
use crate::message::control_function::ControlFunction;
use crate::message::data::u7::U7;
use crate::message::data::FromClamped;
use crate::message::notes::Note;
use crate::message::raw::{Payload, Raw};
use crate::packet::cable_number::CableNumber;
use crate::packet::code_index_number::CodeIndexNumber;
use crate::packet::{UsbMidiEventPacket, UsbMidiEventPacketError};

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
    /// MTC Quarter Frame message.
    MtcQuarterFrame(U7),
    /// Song Position Pointer message.
    SongPositionPointer(U7, U7),
    /// Song Select message.
    SongSelect(U7),
    /// Tune Request message.
    TuneRequest,
    /// Timing Clock message.
    TimingClock,
    /// Tick message.
    Tick,
    /// Start message.
    Start,
    /// Continue message.
    Continue,
    /// Stop message.
    Stop,
    /// Active Sensing message.
    ActiveSensing,
    /// Reset message.
    Reset,
}

const NOTE_OFF_MASK: u8 = 0b1000_0000;
const NOTE_ON_MASK: u8 = 0b1001_0000;
const POLYPHONIC_MASK: u8 = 0b1010_0000;
const PROGRAM_MASK: u8 = 0b1100_0000;
const CHANNEL_AFTERTOUCH_MASK: u8 = 0b1101_0000;
const PITCH_BEND_MASK: u8 = 0b1110_0000;
const CONTROL_CHANGE_MASK: u8 = 0b1011_0000;

const MTC_QUARTER_FRAME: u8 = 0xF1;
const SONG_POSITION_POINTER: u8 = 0xF2;
const SONG_SELECT: u8 = 0xF3;
const TUNE_REQUEST: u8 = 0xF6;
const TIMING_CLOCK: u8 = 0xF8;
const TICK: u8 = 0xF9;
const START: u8 = 0xFA;
const CONTINUE: u8 = 0xFB;
const STOP: u8 = 0xFC;
const ACTIVE_SENSING: u8 = 0xFE;
const RESET: u8 = 0xFF;

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
            Message::MtcQuarterFrame(frame) => {
                let payload = Payload::SingleByte(frame);
                let status = MTC_QUARTER_FRAME;
                Raw { status, payload }
            }
            Message::SongPositionPointer(lsb, msb) => {
                let payload = Payload::DoubleByte(lsb, msb);
                let status = SONG_POSITION_POINTER;
                Raw { status, payload }
            }
            Message::SongSelect(song) => {
                let payload = Payload::SingleByte(song);
                let status = SONG_SELECT;
                Raw { status, payload }
            }
            Message::TuneRequest => {
                let payload = Payload::Empty;
                let status = TUNE_REQUEST;
                Raw { status, payload }
            }
            Message::TimingClock => {
                let payload = Payload::Empty;
                let status = TIMING_CLOCK;
                Raw { status, payload }
            }
            Message::Tick => {
                let payload = Payload::Empty;
                let status = TICK;
                Raw { status, payload }
            }
            Message::Start => {
                let payload = Payload::Empty;
                let status = START;
                Raw { status, payload }
            }
            Message::Continue => {
                let payload = Payload::Empty;
                let status = CONTINUE;
                Raw { status, payload }
            }
            Message::Stop => {
                let payload = Payload::Empty;
                let status = STOP;
                Raw { status, payload }
            }
            Message::ActiveSensing => {
                let payload = Payload::Empty;
                let status = ACTIVE_SENSING;
                Raw { status, payload }
            }
            Message::Reset => {
                let payload = Payload::Empty;
                let status = RESET;
                Raw { status, payload }
            }
        }
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = UsbMidiEventPacketError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let status_byte = match data.first() {
            Some(byte) => byte,
            None => return Err(UsbMidiEventPacketError::MissingDataPacket),
        };

        match *status_byte {
            MTC_QUARTER_FRAME => {
                return Ok(Message::MtcQuarterFrame(get_u7_at(data, 1)?));
            }
            SONG_POSITION_POINTER => {
                return Ok(Message::SongPositionPointer(
                    get_u7_at(data, 1)?,
                    get_u7_at(data, 2)?,
                ));
            }
            SONG_SELECT => {
                return Ok(Message::SongSelect(get_u7_at(data, 1)?));
            }
            TUNE_REQUEST => {
                return Ok(Message::TuneRequest);
            }
            TIMING_CLOCK => {
                return Ok(Message::TimingClock);
            }
            TICK => {
                return Ok(Message::Tick);
            }
            START => {
                return Ok(Message::Start);
            }
            CONTINUE => {
                return Ok(Message::Continue);
            }
            STOP => {
                return Ok(Message::Stop);
            }
            ACTIVE_SENSING => {
                return Ok(Message::ActiveSensing);
            }
            RESET => {
                return Ok(Message::Reset);
            }
            _ => {}
        }

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
            _ => Err(UsbMidiEventPacketError::InvalidEventType(event_type)),
        }
    }
}

impl TryFrom<&UsbMidiEventPacket> for Message {
    type Error = UsbMidiEventPacketError;

    fn try_from(value: &UsbMidiEventPacket) -> Result<Self, Self::Error> {
        Self::try_from(&value.as_raw_bytes()[1..])
    }
}

impl Message {
    /// Create a packet from the message.
    pub fn into_packet(self, cable: CableNumber) -> UsbMidiEventPacket {
        let cin = self.code_index_number() as u8;

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

    /// Returns the code index number for a message.
    pub fn code_index_number(&self) -> CodeIndexNumber {
        match self {
            Self::NoteOn(_, _, _) => CodeIndexNumber::NoteOn,
            Self::NoteOff(_, _, _) => CodeIndexNumber::NoteOff,
            Self::ChannelAftertouch(_, _) => CodeIndexNumber::ChannelPressure,
            Self::PitchWheelChange(_, _, _) => CodeIndexNumber::PitchBendChange,
            Self::PolyphonicAftertouch(_, _, _) => CodeIndexNumber::PolyKeyPress,
            Self::ProgramChange(_, _) => CodeIndexNumber::ProgramChange,
            Self::ControlChange(_, _, _) => CodeIndexNumber::ControlChange,
            Self::MtcQuarterFrame(_) => CodeIndexNumber::SystemCommon2Bytes,
            Self::SongPositionPointer(_, _) => CodeIndexNumber::SystemCommon3Bytes,
            Self::SongSelect(_) => CodeIndexNumber::SystemCommon2Bytes,
            Self::TuneRequest => CodeIndexNumber::SystemCommon1Byte,
            Self::TimingClock => CodeIndexNumber::SingleByte,
            Self::Tick => CodeIndexNumber::SingleByte,
            Self::Start => CodeIndexNumber::SingleByte,
            Self::Continue => CodeIndexNumber::SingleByte,
            Self::Stop => CodeIndexNumber::SingleByte,
            Self::ActiveSensing => CodeIndexNumber::SingleByte,
            Self::Reset => CodeIndexNumber::SingleByte,
        }
    }
}

fn get_note(data: &[u8]) -> Result<Note, UsbMidiEventPacketError> {
    let note_byte = get_byte_at_position(data, 1)?;
    match Note::try_from(note_byte) {
        Ok(note) => Ok(note),
        Err(_) => Err(UsbMidiEventPacketError::InvalidNote(note_byte)),
    }
}

fn get_u7_at(data: &[u8], index: usize) -> Result<U7, UsbMidiEventPacketError> {
    let data_byte = get_byte_at_position(data, index)?;
    Ok(U7::from_clamped(data_byte))
}

fn get_byte_at_position(data: &[u8], index: usize) -> Result<u8, UsbMidiEventPacketError> {
    match data.get(index) {
        Some(byte) => Ok(*byte),
        None => Err(UsbMidiEventPacketError::MissingDataPacket),
    }
}

#[cfg(test)]
mod tests {
    use crate::message::channel::Channel::{Channel1, Channel2};
    use crate::message::control_function::ControlFunction;
    use crate::message::data::u7::U7;
    use crate::message::notes::Note;
    use crate::message::Message;
    use crate::packet::cable_number::CableNumber::{Cable0, Cable1};
    use crate::packet::UsbMidiEventPacket;

    macro_rules! decode_message_test {
        ($($id:ident:$value:expr,)*) => {
            $(
                #[test]
                fn $id() {
                    let (usb_midi_data_packet, expected) = $value;
                    let message = UsbMidiEventPacket::try_from(&usb_midi_data_packet[..]).unwrap();
                    assert_eq!(expected, message);
                }
            )*
        }
    }

    decode_message_test! {
        note_on: ([9, 144, 36, 127], Message::NoteOn(Channel1, Note::C2, U7(127)).into_packet(Cable0)),
        note_off: ([8, 128, 36, 0], Message::NoteOff(Channel1, Note::C2, U7(0)).into_packet(Cable0)),
        polyphonic_aftertouch: ([10, 160, 36, 64], Message::PolyphonicAftertouch(Channel1, Note::C2, U7(64)).into_packet(Cable0)),
        program_change: ([28, 192, 127, 0], Message::ProgramChange(Channel1, U7(127)).into_packet(Cable1)),
        channel_aftertouch: ([13, 208, 127, 0], Message::ChannelAftertouch(Channel1, U7(127)).into_packet(Cable0)),
        pitch_wheel: ([14, 224, 64, 32], Message::PitchWheelChange(Channel1, U7(64), U7(32)).into_packet(Cable0)),
        control_change: ([11, 177, 1, 32], Message::ControlChange(Channel2, ControlFunction::MOD_WHEEL_1, U7(32)).into_packet(Cable0)),
        mtc_quarter_frame: ([0x02, 0xF1, 12, 0], Message::MtcQuarterFrame(U7(12)).into_packet(Cable0)),
        song_position_pointer: ([0x03, 0xF2, 38, 75], Message::SongPositionPointer(U7(38), U7(75)).into_packet(Cable0)),
        song_select: ([0x02, 0xF3, 4, 0], Message::SongSelect(U7(4)).into_packet(Cable0)),
        tune_request: ([0x05, 0xF6, 0, 0], Message::TuneRequest.into_packet(Cable0)),
        timing_clock: ([0x0F, 0xF8, 0, 0], Message::TimingClock.into_packet(Cable0)),
        tick: ([0x0F, 0xF9, 0, 0], Message::Tick.into_packet(Cable0)),
        start: ([0x0F, 0xFA, 0, 0], Message::Start.into_packet(Cable0)),
        continue_: ([0x0F, 0xFB, 0, 0], Message::Continue.into_packet(Cable0)),
        stop: ([0x0F, 0xFC, 0, 0], Message::Stop.into_packet(Cable0)),
        active_sensing: ([0x0F, 0xFE, 0, 0], Message::ActiveSensing.into_packet(Cable0)),
        reset: ([0x0F, 0xFF, 0, 0], Message::Reset.into_packet(Cable0)),
    }
}

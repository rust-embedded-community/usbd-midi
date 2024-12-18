//! Representation of a USB MIDI event packet.

use core::convert::{TryFrom, TryInto};

use crate::data::byte::u4::U4;
use crate::data::midi::message::raw::{Payload, Raw};
use crate::data::midi::message::Message;
use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::usb_midi::code_index_number::CodeIndexNumber;

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
        let cin = u8::from(U4::from(CodeIndexNumber::find_from_message(&message)));

        let mut raw = [0; 4];
        raw[0] = (cable as u8) << 4 | cin;
        let r = Raw::from(message.clone());
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

        Self { raw }
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

#[cfg(test)]
mod tests {
    use crate::data::byte::u7::U7;
    use crate::data::midi::channel::Channel::{Channel1, Channel2};
    use crate::data::midi::message::control_function::ControlFunction;
    use crate::data::midi::message::Message;
    use crate::data::midi::notes::Note;
    use crate::data::usb_midi::cable_number::CableNumber::{Cable0, Cable1};
    use crate::data::usb_midi::usb_midi_event_packet::UsbMidiEventPacket;
    use core::convert::TryFrom;

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
        note_on: ([9, 144, 36, 127], UsbMidiEventPacket::from_message(Cable0, Message::NoteOn(Channel1, Note::C2, U7(127)))),
        note_off: ([8, 128, 36, 0], UsbMidiEventPacket::from_message(Cable0, Message::NoteOff(Channel1, Note::C2, U7(0)))),
        polyphonic_aftertouch: ([10, 160, 36, 64], UsbMidiEventPacket::from_message(Cable0, Message::PolyphonicAftertouch(Channel1, Note::C2, U7(64)))),
        program_change: ([28, 192, 127, 0], UsbMidiEventPacket::from_message(Cable1, Message::ProgramChange(Channel1, U7(127)))),
        channel_aftertouch: ([13, 208, 127, 0], UsbMidiEventPacket::from_message(Cable0, Message::ChannelAftertouch(Channel1, U7(127)))),
        pitch_wheel: ([14, 224, 64, 32], UsbMidiEventPacket::from_message(Cable0, Message::PitchWheelChange(Channel1, U7(64), U7(32)))),
        control_change: ([11, 177, 1, 32], UsbMidiEventPacket::from_message(Cable0, Message::ControlChange(Channel2, ControlFunction::MOD_WHEEL_1, U7(32)))),
    }
}

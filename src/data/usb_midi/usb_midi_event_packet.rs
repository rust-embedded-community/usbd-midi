use crate::data::byte::u4::U4;
use crate::data::midi::message::raw::{Payload, Raw};
use crate::data::midi::message::Message;
use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::usb_midi::code_index_number::CodeIndexNumber;
use core::convert::TryFrom;

/// A packet that communicates with the host
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug, Eq, PartialEq)]
pub struct UsbMidiEventPacket {
    pub cable_number: CableNumber,
    pub message: Message,
}

impl From<UsbMidiEventPacket> for [u8; 4] {
    fn from(value: UsbMidiEventPacket) -> [u8; 4] {
        let message = value.message;
        let cable_number = U4::from(value.cable_number);
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

#[derive(Debug)]
pub enum MidiPacketParsingError {
    InvalidNote(u8),
    InvalidCableNumber(u8),
    InvalidEventType(u8),
    MissingDataPacket,
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = MidiPacketParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let raw_cable_number = match value.first() {
            Some(byte) => *byte >> 4,
            None => return Err(MidiPacketParsingError::MissingDataPacket),
        };

        let cable_number = match CableNumber::try_from(raw_cable_number) {
            Ok(val) => val,
            _ => return Err(MidiPacketParsingError::InvalidCableNumber(raw_cable_number)),
        };

        let message_body = match value.get(1..) {
            Some(bytes) => bytes,
            None => return Err(MidiPacketParsingError::MissingDataPacket),
        };

        let message = Message::try_from(message_body)?;

        Ok(UsbMidiEventPacket {
            cable_number,
            message,
        })
    }
}

impl UsbMidiEventPacket {
    pub fn from_midi(cable: CableNumber, midi: Message) -> UsbMidiEventPacket {
        UsbMidiEventPacket {
            cable_number: cable,
            message: midi,
        }
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
                    let (usb_midi_data_packet,expected) = $value;
                    let message = UsbMidiEventPacket::try_from(&usb_midi_data_packet[..]).unwrap();
                    assert_eq!(expected, message);
                }
            )*
        }
    }

    decode_message_test! {
        note_on: ([9, 144, 36, 127], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::NoteOn(Channel1, Note::C2, U7(127))
        }),
        note_off: ([8, 128, 36, 0], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::NoteOff(Channel1, Note::C2, U7(0))
        }),
        polyphonic_aftertouch: ([10, 160, 36, 64], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::PolyphonicAftertouch(Channel1, Note::C2, U7(64))
        }),
        program_change: ([28, 192, 127, 0], UsbMidiEventPacket {
            cable_number: Cable1,
            message: Message::ProgramChange(Channel1, U7(127))
        }),
        channel_aftertouch: ([13, 208, 127, 0], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::ChannelAftertouch(Channel1, U7(127))
        }),
        pitch_wheel: ([14, 224, 64, 32], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::PitchWheelChange(Channel1, U7(64), U7(32))
        }),
        control_change: ([11, 177, 1, 32], UsbMidiEventPacket {
            cable_number: Cable0,
            message: Message::ControlChange(Channel2, ControlFunction::MOD_WHEEL_1, U7(32))
        }),
    }
}

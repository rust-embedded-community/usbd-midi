use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::usb_midi::code_index_number::CodeIndexNumber;
use crate::data::byte::u4::U4;
use core::convert::TryFrom;
use midi_convert::{
    MidiRenderSlice,
    MidiTryParseSlice,
    midi_types::MidiMessage,
};


/// A packet that communicates with the host
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug, PartialEq)]
pub struct UsbMidiEventPacket {
    pub cable_number : CableNumber,
    pub message: MidiMessage
}

impl From<UsbMidiEventPacket> for [u8;4] {
    fn from(value:UsbMidiEventPacket) -> [u8;4] {
        let message= value.message;
        let cable_number = U4::from(value.cable_number);
        let index_number = {
                let code_index = 
                        CodeIndexNumber::find_from_message(&message);
                U4::from(code_index)
        };
        let header = U4::combine(cable_number,index_number);
        let mut data: [u8; 4] = [header, 0, 0, 0];
        message.render_slice(&mut data[1..]);

        data
    }
}

#[derive(Debug)]
pub enum MidiPacketParsingError {
    InvalidNote(u8),
    InvalidCableNumber(u8),
    InvalidEventType(u8),
    MissingDataPacket
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = MidiPacketParsingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let raw_cable_number= match value.get(0) {
            Some(byte) => *byte >> 4,
            None => return Err(MidiPacketParsingError::MissingDataPacket)
        };

        let cable_number = match CableNumber::try_from(u8::from(raw_cable_number)) {
            Ok(val) => val,
            _ => return Err(MidiPacketParsingError::InvalidCableNumber(raw_cable_number))
        };

        let message_body = match value.get(1..) {
            Some(bytes) => bytes,
            None => return Err(MidiPacketParsingError::MissingDataPacket)
        };

        let message = MidiMessage::try_parse_slice(message_body).map_err(|_| MidiPacketParsingError::MissingDataPacket)?;

        Ok(UsbMidiEventPacket {
            cable_number,
            message
        })
    }
}

impl UsbMidiEventPacket{

    pub fn from_midi(cable:CableNumber, midi:MidiMessage)
        -> UsbMidiEventPacket{
        UsbMidiEventPacket{
            cable_number : cable,
            message : midi
        }
    }
}

#[cfg(test)]
mod tests {
    use core::convert::TryFrom;
    use crate::data::usb_midi::usb_midi_event_packet::UsbMidiEventPacket;
    use crate::midi_types::{Channel, MidiMessage, Value7, Value14, Note, Program, Control};
    use crate::data::usb_midi::cable_number::CableNumber::{Cable0,Cable1};

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

    const CHANNEL1: Channel = Channel::new(0);
    const CHANNEL2: Channel = Channel::new(1);

    decode_message_test! {
        note_on: ([9, 144, 36, 127], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::NoteOn(CHANNEL1, Note::C1, Value7::new(127))
        }),
        note_off: ([8, 128, 36, 0], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::NoteOff(CHANNEL1, Note::C1, Value7::new(0))
        }),
        polyphonic_aftertouch: ([10, 160, 36, 64], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::KeyPressure(CHANNEL1, Note::C1, Value7::new(64))
        }),
        program_change: ([28, 192, 127, 0], UsbMidiEventPacket {
            cable_number: Cable1,
            message: MidiMessage::ProgramChange(CHANNEL1, Program::new(127))
        }),
        channel_aftertouch: ([13, 208, 127, 0], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::ChannelPressure(CHANNEL1, Value7::new(127))
        }),
        pitch_wheel: ([14, 224, 64, 32], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::PitchBendChange(CHANNEL1, Value14::from((64u8, 32u8)))
        }),
        control_change: ([11, 177, 1, 32], UsbMidiEventPacket {
            cable_number: Cable0,
            message: MidiMessage::ControlChange(CHANNEL2, Control::new(1), Value7::new(32))
        }),
    }
}

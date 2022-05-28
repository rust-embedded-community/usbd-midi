use crate::code_index_number;
use core::convert::TryFrom;

use midi_types::MidiMessage;

/// A packet that communicates with the host
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug, PartialEq)]
pub struct UsbMidiEventPacket {
    cable_number: u8,
    message: MidiMessage,
}

impl From<UsbMidiEventPacket> for [u8; 4] {
    fn from(value: UsbMidiEventPacket) -> [u8; 4] {
        let message = value.message;
        let cable_number = value.cable_number;
        let index_number = code_index_number::find_from_message(&message);
        let header: u8 = cable_number | index_number;

        //TODO Sysex
        let mut data: [u8; 4] = [header, 0, 0, 0];
        assert!(message.render(&mut data[1..]).is_ok());
        data
    }
}

#[derive(Debug)]
pub enum MidiPacketParsingError {
    InvalidData,
    MissingDataPacket,
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = MidiPacketParsingError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        let cable_number = match buf.get(0) {
            Some(byte) => *byte >> 4,
            None => return Err(MidiPacketParsingError::MissingDataPacket),
        };

        let message =
            MidiMessage::try_from(&buf[1..]).map_err(|_| MidiPacketParsingError::InvalidData)?;

        Ok(UsbMidiEventPacket {
            cable_number,
            message,
        })
    }
}

impl UsbMidiEventPacket {
    pub fn new(cable: u8, midi: MidiMessage) -> UsbMidiEventPacket {
        assert!(cable < 16);
        UsbMidiEventPacket {
            cable_number: cable,
            message: midi,
        }
    }

    pub fn message(&self) -> &MidiMessage {
        &self.message
    }

    pub fn cable_number(&self) -> u8 {
        self.cable_number
    }
}

#[cfg(test)]
mod tests {
    use crate::event_packet::UsbMidiEventPacket;
    use core::convert::TryFrom;
    use midi_types::{Channel, Control, MidiMessage, Note, Program, Value14, Value7};

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
            cable_number: 0,
            message: MidiMessage::NoteOn(Channel::from(0), Note::from(36), Value7::from(127))
        }),
        note_off: ([8, 128, 36, 0], UsbMidiEventPacket {
            cable_number: 0,
            message: MidiMessage::NoteOff(Channel::from(0), Note::from(36), Value7::from(0))
        }),
        polyphonic_aftertouch: ([10, 160, 36, 64], UsbMidiEventPacket {
            cable_number: 0,
            message: MidiMessage::KeyPressure(Channel::from(0), Note::from(36), Value7::from(64))
        }),
        program_change: ([28, 192, 127, 0], UsbMidiEventPacket {
            cable_number: 1,
            message: MidiMessage::ProgramChange(Channel::from(0), Program::from(127))
        }),
        channel_aftertouch: ([13, 208, 127, 0], UsbMidiEventPacket {
            cable_number: 0,
            message: MidiMessage::ChannelPressure(Channel::from(0), Value7::from(127))
        }),
        pitch_wheel: ([14, 224, 64, 32], UsbMidiEventPacket {
            cable_number: 0,
            message: MidiMessage::PitchBendChange(Channel::from(0), Value14::from((64, 32)))
        }),
        control_change: ([11, 177, 1, 32], UsbMidiEventPacket {
            cable_number: 0,
            message: MidiMessage::ControlChange(Channel::from(1), Control::from(1), Value7::from(32))
        }),
    }
}

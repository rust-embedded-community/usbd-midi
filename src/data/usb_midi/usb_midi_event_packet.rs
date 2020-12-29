use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::usb_midi::code_index_number::CodeIndexNumber;
use crate::data::midi::message::Message;
use crate::data::byte::u4::U4;
use crate::data::midi::message::raw::{Payload,Raw};
use core::convert::TryFrom;


/// A packet that communicates with the host
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug)]
pub struct UsbMidiEventPacket {
    pub cable_number : CableNumber,
    pub message: Message
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

        let raw_midi = Raw::from(message);
        let status = raw_midi.status;

        match raw_midi.payload {
            Payload::Empty => [header,status,0,0],
            Payload::SingleByte(byte) => 
                                [header,status,byte.into(),0],
            Payload::DoubleByte(byte1,byte2) => 
                                    [header,status,byte1.into(),byte2.into()]           
        }
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

        let message = Message::try_from(message_body)?;

        Ok(UsbMidiEventPacket {
            cable_number,
            message
        })
    }
}

impl UsbMidiEventPacket{

    pub fn from_midi(cable:CableNumber, midi:Message)
        -> UsbMidiEventPacket{
        UsbMidiEventPacket{
            cable_number : cable,
            message : midi
        }
    }
}
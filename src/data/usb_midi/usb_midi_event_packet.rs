use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::midi::code_index_number::CodeIndexNumber;
use crate::data::midi::notes::Note;
use crate::data::midi::channel::Channel;
use crate::data::midi::message::MidiMessage;
use crate::data::midi::velocity::Velocity;
use crate::util::nibble::{combine_nibble};

/// A packet that communicates with the host
/// Note that the payload seems fairly 'open'
/// It's contents can depend on the cable/code index number
/// but may not!?
pub struct UsbMidiEventPacket {
    cable_number : CableNumber,
    code_index_number: CodeIndexNumber,
    message: MidiMessage
}

/// Constructs a note-on midi message given the cable, note and velocity
pub fn note_on( cable:CableNumber,
                channel: Channel,
                note:Note, 
                velocity: Velocity) -> UsbMidiEventPacket {
    let message = MidiMessage::note_on(channel,note,velocity);              

    UsbMidiEventPacket{
        cable_number : cable,
        code_index_number : CodeIndexNumber::NOTE_ON,
        message : message
    }
}
impl Into<[u8;4]> for UsbMidiEventPacket {
    /// Converts the midi packet into a byte array
    /// suitable for transfer via usb
    fn into(self) -> [u8;4] {
        let cable_number : u8 = self.cable_number.into();
        let index_number : u8 = self.code_index_number.into();
        let header = combine_nibble(cable_number,index_number);
        let payload : [u8;3]= self.message.into();
        [   header,
            payload[0],
            payload[1],
            payload[2]
        ]
    }
}


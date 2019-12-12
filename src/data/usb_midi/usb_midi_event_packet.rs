use crate::data::usb_midi::cable_number::CableNumber;
use crate::data::midi::code_index_number::CodeIndexNumber;
use crate::data::midi::notes::Note;
use crate::data::midi::midi_channel::MidiChannel;

/// A packet that communicates with the host
/// Note that the payload seems fairly 'open'
/// It's contents can depend on the cable/code index number
/// but may not!?
pub struct UsbMidiEventPacket {
    cable_number : CableNumber,
    code_index_number: CodeIndexNumber,
    payload: [u8;3]
}

/// Combines two nibbles.
/// Note that the upper will overflow if greater than 0xF
/// The lower will be clamped to the range 0-0xF
fn combine_nibble(upper:u8,lower:u8) -> u8 {
    let upper = upper.overflowing_shl(8).0;
    let lower = lower & 0xF;
    upper | lower
}

/// Constructs a note-on midi message given the cable, note and velocity
pub fn note_on( cable:CableNumber,
                channel: MidiChannel,
                note:Note, 
                velocity: u8) -> UsbMidiEventPacket {
         
    let code :u8  = CodeIndexNumber::NOTE_ON.into();                    
    let channel : u8 = channel.into();   
    
    UsbMidiEventPacket{
        cable_number : cable,
        code_index_number : CodeIndexNumber::NOTE_ON,
        payload : [ code & channel,
                    note as u8,
                    velocity        
                  ] 
    }
}
impl Into<[u8;4]> for UsbMidiEventPacket {
    /// Converts the midi packet into a byte array
    /// suitable for transfer via usb
    fn into(self) -> [u8;4] {
        let cable_number : u8 = self.cable_number.into();
        let index_number : u8 = self.code_index_number.into();
        let header = combine_nibble(cable_number,index_number);
        [   header,
            self.payload[0],
            self.payload[1],
            self.payload[2]
        ]
    }
}


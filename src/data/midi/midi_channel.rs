use crate::util::try_from::{TryFrom};

/// The MidiChannel is a value ranging from 0x0 to 0xF 
/// This is a standard midi concept
pub struct MidiChannel(u8);


impl TryFrom<u8> for MidiChannel {

    fn try_from(value:u8) -> Option<MidiChannel> {
        if value > 0xF {
            None
        } else {
            Some(MidiChannel(value))
        }
    }
}

impl Into<u8> for MidiChannel {
    fn into(self) -> u8 {
        self.0
    }
}



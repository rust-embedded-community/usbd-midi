use crate::util::try_from::{TryFrom};

/// The Cable Number (CN) is a value ranging from 0x0 to 0xF 
/// indicating the number assignment of the Embedded MIDI Jack associated 
/// with the endpoint that is transferring the data
pub struct CableNumber(u8);

impl TryFrom<u8> for CableNumber {

    fn try_from(value:u8) -> Option<Self> {
        if value > 0xF {
            None
        } else {
            Some(CableNumber(value))
        }
    }
}

impl Into<u8> for CableNumber {
    fn into(self) -> u8 {
        self.0
    }
}

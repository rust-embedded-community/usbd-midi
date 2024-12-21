//! Enum representing the cable number of a packet.

use crate::data::u4::U4;
use crate::packet::MidiPacketParsingError;

/// The Cable Number (CN) is a value ranging from 0x0 to 0xF
/// indicating the number assignment of the Embedded MIDI Jack associated
/// with the endpoint that is transferring the data
#[allow(missing_docs)]
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum CableNumber {
    #[default]
    Cable0 = 0x0,
    Cable1 = 0x1,
    Cable2 = 0x2,
    Cable3 = 0x3,
    Cable4 = 0x4,
    Cable5 = 0x5,
    Cable6 = 0x6,
    Cable7 = 0x7,
    Cable8 = 0x8,
    Cable9 = 0x9,
    Cable10 = 0xA,
    Cable11 = 0xB,
    Cable12 = 0xC,
    Cable13 = 0xD,
    Cable14 = 0xE,
    Cable15 = 0xF,
}

impl TryFrom<u8> for CableNumber {
    type Error = MidiPacketParsingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == CableNumber::Cable0 as u8 => Ok(CableNumber::Cable0),
            x if x == CableNumber::Cable1 as u8 => Ok(CableNumber::Cable1),
            x if x == CableNumber::Cable2 as u8 => Ok(CableNumber::Cable2),
            x if x == CableNumber::Cable3 as u8 => Ok(CableNumber::Cable3),
            x if x == CableNumber::Cable4 as u8 => Ok(CableNumber::Cable4),
            x if x == CableNumber::Cable5 as u8 => Ok(CableNumber::Cable5),
            x if x == CableNumber::Cable6 as u8 => Ok(CableNumber::Cable6),
            x if x == CableNumber::Cable7 as u8 => Ok(CableNumber::Cable7),
            x if x == CableNumber::Cable8 as u8 => Ok(CableNumber::Cable8),
            x if x == CableNumber::Cable9 as u8 => Ok(CableNumber::Cable9),
            x if x == CableNumber::Cable10 as u8 => Ok(CableNumber::Cable10),
            x if x == CableNumber::Cable11 as u8 => Ok(CableNumber::Cable11),
            x if x == CableNumber::Cable12 as u8 => Ok(CableNumber::Cable12),
            x if x == CableNumber::Cable13 as u8 => Ok(CableNumber::Cable13),
            x if x == CableNumber::Cable14 as u8 => Ok(CableNumber::Cable14),
            x if x == CableNumber::Cable15 as u8 => Ok(CableNumber::Cable15),
            _ => Err(MidiPacketParsingError::InvalidCableNumber(value)),
        }
    }
}

impl From<CableNumber> for u8 {
    fn from(value: CableNumber) -> u8 {
        value as u8
    }
}

impl From<CableNumber> for U4 {
    fn from(value: CableNumber) -> U4 {
        U4::from_overflowing_u8(u8::from(value))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    macro_rules! cable_test {
        ($($id:ident:$value:expr,)*) => {
            $(
                #[test]
                fn $id() {
                    let (input,expected) = $value;
                    assert_eq!(u8::from(input), expected);
                }
            )*
        }
    }

    cable_test! {
            cable_0:  (CableNumber::Cable0,0),
            cable_1:  (CableNumber::Cable1,1),
            cable_2:  (CableNumber::Cable2,2),
            cable_3:  (CableNumber::Cable3,3),
            cable_4:  (CableNumber::Cable4,4),
            cable_5:  (CableNumber::Cable5,5),
            cable_6:  (CableNumber::Cable6,6),
            cable_7:  (CableNumber::Cable7,7),
            cable_8:  (CableNumber::Cable8,8),
            cable_9:  (CableNumber::Cable9,9),
            cable_10:  (CableNumber::Cable10,10),
            cable_11:  (CableNumber::Cable11,11),
            cable_12:  (CableNumber::Cable12,12),
            cable_13:  (CableNumber::Cable13,13),
            cable_14:  (CableNumber::Cable14,14),
            cable_15:  (CableNumber::Cable15,15),
    }
}

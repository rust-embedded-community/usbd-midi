use crate::util::try_from::{TryFrom};

/// The MidiChannel is a value ranging from 0x0 to 0xF 
/// This is a standard midi concept
#[derive(Debug)]
#[repr(u8)]
pub enum MidiChannel {
    Channel0  = 0x0, Channel1  = 0x1, Channel2  = 0x2, Channel3  = 0x3,
    Channel4  = 0x4, Channel5  = 0x5, Channel6  = 0x6, Channel7  = 0x7,
    Channel8  = 0x8, Channel9  = 0x9, Channel10 = 0xA, Channel11 = 0xB,
    Channel12 = 0xC, Channel13 = 0xD, Channel14 = 0xE, Channel15 = 0xF
}

impl TryFrom<u8> for MidiChannel {

    fn try_from(value:u8) -> Option<Self> {
        match value {
            x if x == MidiChannel::Channel0  as u8 => Some(MidiChannel::Channel0),
            x if x == MidiChannel::Channel1  as u8 => Some(MidiChannel::Channel1),
            x if x == MidiChannel::Channel2  as u8 => Some(MidiChannel::Channel2),
            x if x == MidiChannel::Channel3  as u8 => Some(MidiChannel::Channel3),
            x if x == MidiChannel::Channel4  as u8 => Some(MidiChannel::Channel4),
            x if x == MidiChannel::Channel5  as u8 => Some(MidiChannel::Channel5),
            x if x == MidiChannel::Channel6  as u8 => Some(MidiChannel::Channel6),
            x if x == MidiChannel::Channel7  as u8 => Some(MidiChannel::Channel7),
            x if x == MidiChannel::Channel8  as u8 => Some(MidiChannel::Channel8),
            x if x == MidiChannel::Channel9  as u8 => Some(MidiChannel::Channel9),
            x if x == MidiChannel::Channel10 as u8 => Some(MidiChannel::Channel10),
            x if x == MidiChannel::Channel11 as u8 => Some(MidiChannel::Channel11),
            x if x == MidiChannel::Channel12 as u8 => Some(MidiChannel::Channel12),
            x if x == MidiChannel::Channel13 as u8 => Some(MidiChannel::Channel13),
            x if x == MidiChannel::Channel14 as u8 => Some(MidiChannel::Channel14),
            x if x == MidiChannel::Channel15 as u8 => Some(MidiChannel::Channel15),
            _ => None
        }
    }

}

impl Into<u8> for MidiChannel {
    fn into(self) -> u8 {
        self as u8
    }
}

#[cfg(test)]
mod tests {
    
    use super::*;
    macro_rules! channel_test {
        ($($id:ident:$value:expr,)*) => {
            $(
                #[test]
                fn $id() {
                    let (input,expected) = $value;
                    assert_eq!(input as u8, expected);
                }
            )*
        }
    }

    channel_test! {
            cable_0:  (MidiChannel::Channel0,0),
            cable_1:  (MidiChannel::Channel1,1),
            cable_2:  (MidiChannel::Channel2,2),
            cable_3:  (MidiChannel::Channel3,3),
            cable_4:  (MidiChannel::Channel4,4),
            cable_5:  (MidiChannel::Channel5,5),
            cable_6:  (MidiChannel::Channel6,6),
            cable_7:  (MidiChannel::Channel7,7),
            cable_8:  (MidiChannel::Channel8,8),
            cable_9:  (MidiChannel::Channel9,9),
            cable_10:  (MidiChannel::Channel10,10),
            cable_11:  (MidiChannel::Channel11,11),
            cable_12:  (MidiChannel::Channel12,12),
            cable_13:  (MidiChannel::Channel13,13),
            cable_14:  (MidiChannel::Channel14,14),
            cable_15:  (MidiChannel::Channel15,15),
    }
}
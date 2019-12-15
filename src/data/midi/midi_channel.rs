use core::convert::TryFrom;

/// The MidiChannel is a value ranging from 0x0 to 0xF 
/// This is a standard midi concept
/// Note Channel1 = 0 on the wire
#[derive(Debug)]
#[repr(u8)]
pub enum MidiChannel {
    Channel1  = 0x0, Channel2  = 0x1, Channel3  = 0x2, Channel4  = 0x3,
    Channel5  = 0x4, Channel6  = 0x5, Channel7  = 0x6, Channel8  = 0x7,
    Channel9  = 0x8, Channel10 = 0x9, Channel11 = 0xA, Channel12 = 0xB,
    Channel13 = 0xC, Channel14 = 0xD, Channel15 = 0xE, Channel16 = 0xF
}

pub struct InvalidMidiChannel(u8);

impl TryFrom<u8> for MidiChannel {
    type Error = InvalidMidiChannel;
    fn try_from(value:u8) -> Result<Self,Self::Error> {
        match value {
            x if x == MidiChannel::Channel1  as u8 => Ok(MidiChannel::Channel1),
            x if x == MidiChannel::Channel2  as u8 => Ok(MidiChannel::Channel2),
            x if x == MidiChannel::Channel3  as u8 => Ok(MidiChannel::Channel3),
            x if x == MidiChannel::Channel4  as u8 => Ok(MidiChannel::Channel4),
            x if x == MidiChannel::Channel5  as u8 => Ok(MidiChannel::Channel5),
            x if x == MidiChannel::Channel6  as u8 => Ok(MidiChannel::Channel6),
            x if x == MidiChannel::Channel7  as u8 => Ok(MidiChannel::Channel7),
            x if x == MidiChannel::Channel8  as u8 => Ok(MidiChannel::Channel8),
            x if x == MidiChannel::Channel9  as u8 => Ok(MidiChannel::Channel9),
            x if x == MidiChannel::Channel10 as u8 => Ok(MidiChannel::Channel10),
            x if x == MidiChannel::Channel11 as u8 => Ok(MidiChannel::Channel11),
            x if x == MidiChannel::Channel12 as u8 => Ok(MidiChannel::Channel12),
            x if x == MidiChannel::Channel13 as u8 => Ok(MidiChannel::Channel13),
            x if x == MidiChannel::Channel14 as u8 => Ok(MidiChannel::Channel14),
            x if x == MidiChannel::Channel15 as u8 => Ok(MidiChannel::Channel15),
            x if x == MidiChannel::Channel16 as u8 => Ok(MidiChannel::Channel16),
            _ => Err(InvalidMidiChannel(value))
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
            channel_1:   (MidiChannel::Channel1,0),
            channel_2:   (MidiChannel::Channel2,1),
            channel_3:   (MidiChannel::Channel3,2),
            channel_4:   (MidiChannel::Channel4,3),
            channel_5:   (MidiChannel::Channel5,4),
            channel_6:   (MidiChannel::Channel6,5),
            channel_7:   (MidiChannel::Channel7,6),
            channel_8:   (MidiChannel::Channel8,7),
            channel_9:   (MidiChannel::Channel9,8),
            channel_10:  (MidiChannel::Channel10,9),
            channel_11:  (MidiChannel::Channel11,10),
            channel_12:  (MidiChannel::Channel12,11),
            channel_13:  (MidiChannel::Channel13,12),
            channel_14:  (MidiChannel::Channel14,13),
            channel_15:  (MidiChannel::Channel15,14),
            channel_16:  (MidiChannel::Channel16,15),
    }
}
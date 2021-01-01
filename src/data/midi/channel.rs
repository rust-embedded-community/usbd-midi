use core::convert::TryFrom;

/// The Channel is a value ranging from 0x0 to 0xF 
/// This is a standard midi concept
/// Note Channel1 = 0 on the wire
#[derive(Debug,Copy,Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Channel {
    Channel1  = 0x0, Channel2  = 0x1, Channel3  = 0x2, Channel4  = 0x3,
    Channel5  = 0x4, Channel6  = 0x5, Channel7  = 0x6, Channel8  = 0x7,
    Channel9  = 0x8, Channel10 = 0x9, Channel11 = 0xA, Channel12 = 0xB,
    Channel13 = 0xC, Channel14 = 0xD, Channel15 = 0xE, Channel16 = 0xF
}

pub struct InvalidChannel(u8);

impl TryFrom<u8> for Channel {
    type Error = InvalidChannel;
    fn try_from(value:u8) -> Result<Self,Self::Error> {
        match value {
            x if x == Channel::Channel1  as u8 => Ok(Channel::Channel1),
            x if x == Channel::Channel2  as u8 => Ok(Channel::Channel2),
            x if x == Channel::Channel3  as u8 => Ok(Channel::Channel3),
            x if x == Channel::Channel4  as u8 => Ok(Channel::Channel4),
            x if x == Channel::Channel5  as u8 => Ok(Channel::Channel5),
            x if x == Channel::Channel6  as u8 => Ok(Channel::Channel6),
            x if x == Channel::Channel7  as u8 => Ok(Channel::Channel7),
            x if x == Channel::Channel8  as u8 => Ok(Channel::Channel8),
            x if x == Channel::Channel9  as u8 => Ok(Channel::Channel9),
            x if x == Channel::Channel10 as u8 => Ok(Channel::Channel10),
            x if x == Channel::Channel11 as u8 => Ok(Channel::Channel11),
            x if x == Channel::Channel12 as u8 => Ok(Channel::Channel12),
            x if x == Channel::Channel13 as u8 => Ok(Channel::Channel13),
            x if x == Channel::Channel14 as u8 => Ok(Channel::Channel14),
            x if x == Channel::Channel15 as u8 => Ok(Channel::Channel15),
            x if x == Channel::Channel16 as u8 => Ok(Channel::Channel16),
            _ => Err(InvalidChannel(value))
        }
    }

}

impl From<Channel> for u8 {
    fn from(src:Channel) -> u8 {
        src as u8
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
            channel_1:   (Channel::Channel1,0),
            channel_2:   (Channel::Channel2,1),
            channel_3:   (Channel::Channel3,2),
            channel_4:   (Channel::Channel4,3),
            channel_5:   (Channel::Channel5,4),
            channel_6:   (Channel::Channel6,5),
            channel_7:   (Channel::Channel7,6),
            channel_8:   (Channel::Channel8,7),
            channel_9:   (Channel::Channel9,8),
            channel_10:  (Channel::Channel10,9),
            channel_11:  (Channel::Channel11,10),
            channel_12:  (Channel::Channel12,11),
            channel_13:  (Channel::Channel13,12),
            channel_14:  (Channel::Channel14,13),
            channel_15:  (Channel::Channel15,14),
            channel_16:  (Channel::Channel16,15),
    }
}
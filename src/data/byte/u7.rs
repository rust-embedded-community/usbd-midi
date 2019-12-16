use core::convert::TryFrom;

/// A primitive value that can be from 0-0x7F
pub struct U7(u8);

/// Error representing that this value is not a valid u7
pub struct InvalidU7(u8);

impl TryFrom<u8> for U7{
    type Error = InvalidU7;

    fn try_from(value:u8) -> Result<Self,Self::Error> {
        if value > 0x7F {
            Err(InvalidU7(value))
        } else {
            Ok(U7(value))
        }
    }
}

impl From<U7> for u8 {
    fn from(value:U7) -> u8 {
        value.0
    }
}

impl U7 {
    pub const MAX: U7= U7(0x7F);
    pub const MIN: U7 = U7(0);
}
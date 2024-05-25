use crate::data::byte::from_traits::{FromClamped, FromOverFlow};
use core::convert::TryFrom;

/// A primitive value that can be from 0-0x7F
#[derive(Debug, Eq, PartialEq)]
pub struct U7(pub(crate) u8);

/// Error representing that this value is not a valid u7
pub struct InvalidU7(u8);

impl TryFrom<u8> for U7 {
    type Error = InvalidU7;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 0x7F {
            Err(InvalidU7(value))
        } else {
            Ok(U7(value))
        }
    }
}

impl From<U7> for u8 {
    fn from(value: U7) -> u8 {
        value.0
    }
}

impl FromOverFlow<u8> for U7 {
    fn from_overflow(value: u8) -> U7 {
        const MASK: u8 = 0b0111_1111;
        let value = MASK & value;
        U7(value)
    }
}

impl FromClamped<u8> for U7 {
    fn from_clamped(value: u8) -> U7 {
        match U7::try_from(value) {
            Ok(x) => x,
            _ => U7::MAX,
        }
    }
}

impl U7 {
    pub const MAX: U7 = U7(0x7F);
    pub const MIN: U7 = U7(0);
}

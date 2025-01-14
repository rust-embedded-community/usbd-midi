//! A primitive value with 14-bit length.

use crate::message::data::{FromClamped, FromOverFlow};

/// A primitive value that can be from 0-0x4000
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct U14(pub(crate) u16);

/// Error representing that this value is not a valid u14
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct InvalidU14(pub u16);

impl TryFrom<u16> for U14 {
    type Error = InvalidU14;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 0x3FFF {
            Err(InvalidU14(value))
        } else {
            Ok(U14(value))
        }
    }
}

impl From<U14> for u16 {
    fn from(value: U14) -> u16 {
        value.0
    }
}

impl FromOverFlow<u16> for U14 {
    fn from_overflow(value: u16) -> U14 {
        const MASK: u16 = 0b0011_1111_1111_1111;
        let value = MASK & value;
        U14(value)
    }
}

impl FromClamped<u16> for U14 {
    fn from_clamped(value: u16) -> U14 {
        match U14::try_from(value) {
            Ok(x) => x,
            _ => U14::MAX,
        }
    }
}

impl U14 {
    /// Maximum value for the type.
    pub const MAX: U14 = U14(0x3FFF);
    /// Minimum value for the type.
    pub const MIN: U14 = U14(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_valid() {
        assert_eq!(U14::try_from(0x1234), Ok(U14(0x1234)));
    }

    #[test]
    fn try_from_invalid() {
        assert_eq!(U14::try_from(0x4000), Err(InvalidU14(0x4000)));
    }

    #[test]
    fn from_overflow() {
        assert_eq!(U14::from_overflow(0x400F), U14(0x0F));
    }

    #[test]
    fn from_clamped() {
        assert_eq!(U14::from_clamped(0x400F), U14(0x3FFF));
    }
}

use core::convert::TryFrom;

/// A primitive value that can be from 0-0x7F
pub struct U4(u8);

/// Error representing that this value is not a valid u4
pub struct InvalidU4(u8);

impl TryFrom<u8> for U4 {
    type Error = InvalidU4;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > U4::MAX.0 {
            Err(InvalidU4(value))
        } else {
            Ok(U4(value))
        }
    }
}

impl From<U4> for u8 {
    fn from(value: U4) -> u8 {
        value.0
    }
}

impl U4 {
    pub const MAX: U4 = U4(0x0F);
    pub const MIN: U4 = U4(0);

    /// Combines two nibbles (u4) eg half byte
    /// result will be a full byte
    pub fn combine(upper: U4, lower: U4) -> u8 {
        let upper = upper.0.overflowing_shl(8).0;
        let lower = lower.0 & U4::MAX.0;
        upper | lower
    }

    /// Constructs a U4 from a u8.
    /// Note this clamps off the upper portions
    pub fn from_overflowing_u8(value: u8) -> U4 {
        const MASK: u8 = 0b0000_1111;
        let number = MASK & value;
        U4(number)
    }
}

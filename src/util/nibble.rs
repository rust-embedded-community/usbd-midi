

/// Combines two nibbles.
/// Note that the upper will overflow if greater than 0xF
/// The lower will be clamped to the range 0-0xF
pub fn combine_nibble(upper:u8,lower:u8) -> u8 {
    let upper = upper.overflowing_shl(8).0;
    let lower = lower & 0xF;
    upper | lower
}
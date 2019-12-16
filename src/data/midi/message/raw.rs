/// Represents the payloads that the midi message may contain
pub enum Payload {
    Empty,
    SingleByte(u8),
    DoubleByte(u8,u8)
}

/// A struct that captures the valid states
/// a midi message may be in, but without domain logic
/// mainly useful for serializing.
/// This represents the possible 'shapes', doesn't verify if
/// the data makes sense though!
pub struct Raw {
    pub status: u8,
    pub payload: Payload
}

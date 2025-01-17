//! Type for the raw MIDI message.

use crate::message::data::u7::U7;

/// Represents the payloads that the midi message may contain.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Payload {
    /// No payload.
    Empty,
    /// One-byte payload.
    SingleByte(U7),
    /// Two-byte payload.
    DoubleByte(U7, U7),
}

/// A struct that captures the valid states.
///
/// A midi message may be in, but without domain logic mainly useful for serializing.
/// This represents the possible 'shapes', doesn't verify if the data makes sense though!
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Raw {
    /// Status byte.
    pub status: u8,
    /// Payload of maximum 2 bytes.
    pub payload: Payload,
}

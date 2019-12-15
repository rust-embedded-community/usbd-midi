use core::convert::TryFrom;

pub struct MidiVelocity(u8);

pub struct InvalidMidiVelocity(u8);

impl TryFrom<u8> for MidiVelocity{
    type Error = InvalidMidiVelocity;

    fn try_from(value:u8) -> Result<Self,Self::Error> {
        if value > 0x7F {
            Err(InvalidMidiVelocity(value))
        } else {
            Ok(MidiVelocity(value))
        }
    }
}

impl Into<u8> for MidiVelocity {
    fn into(self) -> u8 {
        self.0
    }
}

impl MidiVelocity {
    pub const MAX: MidiVelocity= MidiVelocity(0x7F);
    pub const MIN: MidiVelocity = MidiVelocity(0);
}
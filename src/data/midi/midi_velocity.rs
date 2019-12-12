use crate::util::try_from::TryFrom;

pub struct MidiVelocity(u8);

impl From<u8> for MidiVelocity{
    fn from(value:u8) -> MidiVelocity{
        let masked = value & 0x7F;
        MidiVelocity(masked)
    }
}

impl Into<u8> for MidiVelocity{
    fn into(self) -> u8 {
        self.0
    }
}

impl TryFrom<u8> for MidiVelocity{
    fn try_from(value:u8) -> Option<MidiVelocity> {
        if value > 0x7F {
            None
        } else {
            Some(MidiVelocity(value))
        }
    }
}

impl MidiVelocity {
    pub const MAX: MidiVelocity= MidiVelocity(0x7F);
    pub const MIN: MidiVelocity = MidiVelocity(0);
}
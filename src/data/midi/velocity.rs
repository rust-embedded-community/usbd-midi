use core::convert::TryFrom;

pub struct Velocity(u8);

pub struct InvalidVelocity(u8);

impl TryFrom<u8> for Velocity{
    type Error = InvalidVelocity;

    fn try_from(value:u8) -> Result<Self,Self::Error> {
        if value > 0x7F {
            Err(InvalidVelocity(value))
        } else {
            Ok(Velocity(value))
        }
    }
}

impl Into<u8> for Velocity {
    fn into(self) -> u8 {
        self.0
    }
}

impl Velocity {
    pub const MAX: Velocity= Velocity(0x7F);
    pub const MIN: Velocity = Velocity(0);
}
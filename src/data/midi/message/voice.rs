
pub use crate::data::midi::channel::Channel;
pub use crate::data::midi::notes::Note;
pub use crate::data::midi::velocity::Velocity;

pub enum Voice { 
    NoteOff(Channel,Note,Velocity),
    NoteOn(Channel,Note,Velocity),
    PolyPressure(Channel)
}


impl Voice {
    
}
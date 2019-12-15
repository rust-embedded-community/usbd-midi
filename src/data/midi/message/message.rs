
use crate::data::midi::channel::Channel;
use crate::data::midi::velocity::Velocity;
use crate::data::midi::notes::Note;

pub enum Mode  {Mode}

pub enum RealTime {}
pub enum Common {}
pub enum SysEx {}

pub enum System {
    RealTime(RealTime),
    Common(Common),
    SysEx(SysEx)
}

pub enum Message {
    Channel(Channel),
    System(System)
}

pub struct MidiMessage {
    payload: [u8;3]
}



impl MidiMessage {
    pub fn note_on(channel:Channel, note:Note, velocity:Velocity) 
                                                            -> MidiMessage{
        let channel : u8 = channel.into();
        let note : u8 = note.into();   
        let velocity : u8 = velocity.into();                                                             
        MidiMessage {
            payload: [channel,note,velocity]
        }
    }
}

impl Into<[u8;3]> for MidiMessage {
    /// Converts the midi packet into a byte array
    /// suitable for transfer via usb
    fn into(self) -> [u8;3] {
        self.payload
    }
}

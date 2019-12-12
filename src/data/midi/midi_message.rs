
use crate::data::midi::midi_channel::MidiChannel;
use crate::data::midi::notes::Note;
use crate::data::midi::midi_velocity::MidiVelocity;

pub struct MidiMessage {
    payload: [u8;3]
}

impl MidiMessage {
    pub fn note_on(channel:MidiChannel, note:Note, velocity:MidiVelocity) 
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

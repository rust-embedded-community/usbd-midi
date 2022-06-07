use crate::{
    event_packet::{MidiPacketParsingError, UsbMidiEventPacket},
    midi_device::{MAX_PACKET_SIZE, MIDI_PACKET_SIZE},
};
use core::convert::TryFrom;

pub struct MidiPacketBufferReader<'a> {
    inner: core::slice::Chunks<'a, u8>,
}

impl<'a> MidiPacketBufferReader<'a> {
    pub fn new(buffer: &'a [u8; MAX_PACKET_SIZE], raw_bytes_received: usize) -> Self {
        let inner = buffer[..raw_bytes_received].chunks(MIDI_PACKET_SIZE);
        MidiPacketBufferReader { inner }
    }
}

impl<'a> Iterator for MidiPacketBufferReader<'a> {
    type Item = Result<UsbMidiEventPacket, MidiPacketParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|packet| UsbMidiEventPacket::try_from(packet))
    }
}

impl<'a> ExactSizeIterator for MidiPacketBufferReader<'a> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::midi_types::{Channel, Control, MidiMessage, Note, Program, Value14, Value7};
    const VALID_BUF: [u8; 64] = [
        9, 144, 36, 127, //note on
        8, 128, 36, 0, //note off
        10, 160, 36, 64, //poly after touch (key pressure)
        28, 192, 127, 0, //prog change
        13, 208, 127, 0, //channel pressure
        14, 224, 64, 32, //pitch bend
        11, 177, 1, 32, //control change
        11, 177, 1, 123, //control change
        //repeat
        9, 144, 36, 127, //note on
        8, 128, 36, 0, //note off
        10, 160, 36, 64, //poly after touch (key pressure)
        28, 192, 127, 0, //prog change
        13, 208, 127, 0, //channel pressure
        14, 224, 64, 32, //pitch bend
        11, 177, 1, 32, //control change
        11, 177, 1, 123, //control change
    ];

    #[test]
    fn read() {
        let reader = MidiPacketBufferReader::new(&VALID_BUF, VALID_BUF.len());
        let mut iter = reader.into_iter();

        assert_eq!(iter.len(), 16);

        //repeats 2x
        for _ in 0..2 {
            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::NoteOn(Channel::from(0), Note::from(36), Value7::from(127))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::NoteOff(Channel::from(0), Note::from(36), Value7::from(0))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::KeyPressure(Channel::from(0), Note::from(36), Value7::from(64))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::ProgramChange(Channel::from(0), Program::from(127))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::ChannelPressure(Channel::from(0), Value7::from(127))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::PitchBendChange(Channel::from(0), Value14::from((64, 32)))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::ControlChange(Channel::from(1), Control::from(1), Value7::from(32))
            );

            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::ControlChange(Channel::from(1), Control::from(1), Value7::from(123))
            );
        }
        assert_eq!(None, iter.next());
    }

    #[test]
    fn read_partial() {
        let reader = MidiPacketBufferReader::new(&VALID_BUF, 4);
        let mut iter = reader.into_iter();

        assert_eq!(iter.len(), 1);
        let v = iter.next().unwrap().unwrap();
        assert_eq!(
            v.message(),
            &MidiMessage::NoteOn(Channel::from(0), Note::from(36), Value7::from(127))
        );
        assert_eq!(None, iter.next());

        for len in 5..8 {
            let reader = MidiPacketBufferReader::new(&VALID_BUF, len);
            let mut iter = reader.into_iter();

            assert_eq!(iter.len(), 2);
            let v = iter.next().unwrap().unwrap();
            assert_eq!(
                v.message(),
                &MidiMessage::NoteOn(Channel::from(0), Note::from(36), Value7::from(127))
            );
            assert_eq!(Some(Err(MidiPacketParsingError::InvalidData)), iter.next());
            assert_eq!(None, iter.next());
        }
    }
}

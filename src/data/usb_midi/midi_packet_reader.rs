use crate::data::usb_midi::usb_midi_event_packet::{MidiPacketParsingError, UsbMidiEventPacket};
use crate::midi_device::{MAX_PACKET_SIZE, MIDI_PACKET_SIZE};
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

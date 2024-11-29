//! Reader for received packets.

use crate::data::usb_midi::usb_midi_event_packet::{MidiPacketParsingError, UsbMidiEventPacket};
use crate::midi_device::{MAX_PACKET_SIZE, MIDI_PACKET_SIZE};
use core::convert::TryFrom;

/// Packet reader with internal buffer for received message.
pub struct MidiPacketBufferReader<'a> {
    buffer: &'a [u8; MAX_PACKET_SIZE],
    position: usize,
    raw_bytes_received: usize,
}

impl<'a> MidiPacketBufferReader<'a> {
    /// Creates a new reader.
    pub fn new(buffer: &'a [u8; MAX_PACKET_SIZE], raw_bytes_received: usize) -> Self {
        MidiPacketBufferReader {
            buffer,
            position: 0,
            raw_bytes_received,
        }
    }
}

impl Iterator for MidiPacketBufferReader<'_> {
    type Item = Result<UsbMidiEventPacket, MidiPacketParsingError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position <= MAX_PACKET_SIZE && self.position < self.raw_bytes_received {
            let packet = self
                .buffer
                .get(self.position..(self.position + MIDI_PACKET_SIZE))
                .map(UsbMidiEventPacket::try_from);

            self.position += MIDI_PACKET_SIZE;
            return packet;
        }
        None
    }
}

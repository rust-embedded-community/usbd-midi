//! Contains the class implementation.

use usb_device::class_prelude::*;
use usb_device::Result;

use crate::packet::{UsbMidiEventPacket, UsbMidiEventPacketError};

// Constants for use in descriptors.
const USB_AUDIO_CLASS: u8 = 0x01;
const USB_AUDIOCONTROL_SUBCLASS: u8 = 0x01;
const USB_MIDISTREAMING_SUBCLASS: u8 = 0x03;
const MIDI_IN_JACK_SUBTYPE: u8 = 0x02;
const MIDI_OUT_JACK_SUBTYPE: u8 = 0x03;
const EMBEDDED: u8 = 0x01;
const EXTERNAL: u8 = 0x02;
const CS_INTERFACE: u8 = 0x24;
const CS_ENDPOINT: u8 = 0x25;
const HEADER_SUBTYPE: u8 = 0x01;
const MS_HEADER_SUBTYPE: u8 = 0x01;
const MS_GENERAL: u8 = 0x01;

const MIDI_IN_SIZE: u8 = 0x06;
const MIDI_OUT_SIZE: u8 = 0x09;

/// Size of a single packet in bytes.
pub const MIDI_PACKET_SIZE: usize = 4;

/// Maximum transfer size of an endpoint.
pub const MAX_PACKET_SIZE: usize = 64;

/// Packet-level implementation of a USB MIDI device.
pub struct MidiClass<'a, B: UsbBus> {
    standard_ac: InterfaceNumber,
    standard_mc: InterfaceNumber,
    standard_bulkout: EndpointOut<'a, B>,
    standard_bulkin: EndpointIn<'a, B>,
    n_in_jacks: u8,
    n_out_jacks: u8,
}

/// Error variants for read operations.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UsbMidiReadError {
    /// Parsing of the packet failed.
    ParsingFailed(UsbMidiEventPacketError),
    /// USB stack error returned from `usb-device`.
    UsbError(UsbError),
}

/// Error returned when passing invalid arguments to `MidiClass::new`.
#[derive(Debug)]
pub struct InvalidArguments;

impl<B: UsbBus> MidiClass<'_, B> {
    /// Creates a new MidiClass with the provided UsbBus and `n_in/out_jacks` embedded input/output jacks
    /// (or "cables", depending on the terminology).
    /// Note that a maximum of 16 in and 16 out jacks is supported.
    pub fn new(
        alloc: &UsbBusAllocator<B>,
        n_in_jacks: u8,
        n_out_jacks: u8,
    ) -> core::result::Result<MidiClass<'_, B>, InvalidArguments> {
        if n_in_jacks > 16 || n_out_jacks > 16 {
            return Err(InvalidArguments);
        }
        Ok(MidiClass {
            standard_ac: alloc.interface(),
            standard_mc: alloc.interface(),
            standard_bulkout: alloc.bulk(MAX_PACKET_SIZE as u16),
            standard_bulkin: alloc.bulk(MAX_PACKET_SIZE as u16),
            n_in_jacks,
            n_out_jacks,
        })
    }

    /// Sends bytes from a 4-byte buffer and returns either the transferred size or an error.
    pub fn send_bytes(&mut self, buffer: [u8; 4]) -> Result<usize> {
        self.standard_bulkin.write(&buffer)
    }

    /// Sends a `UsbMidiEventPacket` and returns either the transferred size or an error.
    pub fn send_packet(&mut self, usb_midi: UsbMidiEventPacket) -> Result<usize> {
        let bytes: [u8; MIDI_PACKET_SIZE] = usb_midi.into();
        self.standard_bulkin.write(&bytes)
    }

    /// Reads received bytes into a buffer and returns either the transferred size or an error.
    pub fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.standard_bulkout.read(buffer)
    }

    /// Calculates the index'th external midi in jack id.
    fn in_jack_id_ext(&self, index: u8) -> u8 {
        debug_assert!(index < self.n_in_jacks);
        2 * index + 1
    }

    /// Calculates the index'th embedded midi out jack id.
    fn out_jack_id_emb(&self, index: u8) -> u8 {
        debug_assert!(index < self.n_in_jacks);
        2 * index + 2
    }

    /// Calculates the index'th external midi out jack id.
    fn out_jack_id_ext(&self, index: u8) -> u8 {
        debug_assert!(index < self.n_out_jacks);
        2 * self.n_in_jacks + 2 * index + 1
    }

    /// Calculates the index'th embedded midi in jack id.
    fn in_jack_id_emb(&self, index: u8) -> u8 {
        debug_assert!(index < self.n_out_jacks);
        2 * self.n_in_jacks + 2 * index + 2
    }
}

impl<B: UsbBus> UsbClass<B> for MidiClass<'_, B> {
    fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        // AUDIO CONTROL STANDARD
        writer.interface(
            self.standard_ac,
            USB_AUDIO_CLASS,
            USB_AUDIOCONTROL_SUBCLASS,
            0, // no protocol,
        )?;

        // AUDIO CONTROL EXTRA INFO
        writer.write(
            CS_INTERFACE,
            &[
                HEADER_SUBTYPE,
                0x00,
                0x01, // REVISION
                0x09,
                0x00, // SIZE of class specific descriptions
                0x01, // Number of streaming interfaces
                0x01, // MIDIStreaming interface 1 belongs to this AC interface
            ],
        )?;

        // Streaming Standard
        writer.interface(
            self.standard_mc,
            USB_AUDIO_CLASS,
            USB_MIDISTREAMING_SUBCLASS,
            0, // no protocol
        )?; // Num endpoints?

        let midi_streaming_start_byte = writer.position();
        let midi_streaming_total_length = 7
            + (self.n_in_jacks + self.n_out_jacks) as usize
                * (MIDI_IN_SIZE + MIDI_OUT_SIZE) as usize
            + 9
            + (4 + self.n_out_jacks as usize)
            + 9
            + (4 + self.n_in_jacks as usize);

        // Streaming extra info
        writer.write(
            // len = 7
            CS_INTERFACE,
            &[
                MS_HEADER_SUBTYPE,
                0x00,
                0x01, //REVISION
                (midi_streaming_total_length & 0xFF) as u8,
                ((midi_streaming_total_length >> 8) & 0xFF) as u8,
            ],
        )?;

        // JACKS
        for i in 0..self.n_in_jacks {
            writer.write(
                // len = 6 = MIDI_IN_SIZE
                CS_INTERFACE,
                &[
                    MIDI_IN_JACK_SUBTYPE,
                    EXTERNAL,
                    self.in_jack_id_ext(i), // id
                    0x00,
                ],
            )?;
        }

        for i in 0..self.n_out_jacks {
            writer.write(
                // len = 6 = MIDI_IN_SIZE
                CS_INTERFACE,
                &[
                    MIDI_IN_JACK_SUBTYPE,
                    EMBEDDED,
                    self.in_jack_id_emb(i), // id
                    0x00,
                ],
            )?;
        }

        for i in 0..self.n_out_jacks {
            writer.write(
                // len = 9 = MIDI_OUT_SIZE
                CS_INTERFACE,
                &[
                    MIDI_OUT_JACK_SUBTYPE,
                    EXTERNAL,
                    self.out_jack_id_ext(i), // id
                    0x01,                    // 1 pin
                    self.in_jack_id_emb(i),  // pin is connected to this entity...
                    0x01,                    // ...to the first pin
                    0x00,
                ],
            )?;
        }

        for i in 0..self.n_in_jacks {
            writer.write(
                // len = 9 = MIDI_OUT_SIZE
                CS_INTERFACE,
                &[
                    MIDI_OUT_JACK_SUBTYPE,
                    EMBEDDED,
                    self.out_jack_id_emb(i), // id
                    0x01,                    // 1 pin
                    self.in_jack_id_ext(i),  // pin is connected to this entity...
                    0x01,                    // ...to the first pin
                    0x00,
                ],
            )?;
        }

        let mut endpoint_data = [
            MS_GENERAL, 0, // number of jacks. must be filled in!
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, // jack mappings. must be filled in and cropped.
        ];

        writer.endpoint_ex(&self.standard_bulkout, |data| {
            data[0] = 0; // bRefresh
            data[1] = 0; // bSynchAddress
            Ok(2)
        })?; // len = 9

        endpoint_data[1] = self.n_out_jacks;
        for i in 0..self.n_out_jacks {
            endpoint_data[2 + i as usize] = self.in_jack_id_emb(i);
        }
        writer.write(
            // len = 4 + self.n_out_jacks
            CS_ENDPOINT,
            &endpoint_data[0..2 + self.n_out_jacks as usize],
        )?;

        writer.endpoint_ex(&self.standard_bulkin, |data| {
            data[0] = 0; // bRefresh
            data[1] = 0; // bSynchAddress
            Ok(2)
        })?; // len = 9

        endpoint_data[1] = self.n_in_jacks;
        for i in 0..self.n_in_jacks {
            endpoint_data[2 + i as usize] = self.out_jack_id_emb(i);
        }
        writer.write(
            // len = 4 + self.n_in_jacks
            CS_ENDPOINT,
            &endpoint_data[0..2 + self.n_in_jacks as usize],
        )?;

        let midi_streaming_end_byte = writer.position();
        assert!(midi_streaming_end_byte - midi_streaming_start_byte == midi_streaming_total_length);

        Ok(())
    }
}

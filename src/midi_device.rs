use usb_device::class_prelude::*;
use usb_device::Result;
use crate::data::midi::notes::Note;
use crate::data::usb::constants::*;
use crate::data::usb_midi::usb_midi_event_packet::UsbMidiEventPacket;

//const MIDI_IN_SIZE: u8 = 0x06;
const MIDI_OUT_SIZE: u8 = 0x09;

///Note we are using MidiIn here to refer to the fact that
///The Host sees it as a midi in device
///This class allows you to send data in
pub struct MidiClass<'a,B: UsbBus> {
    standard_ac: InterfaceNumber,
    standard_mc: InterfaceNumber,
    //standard_bulkout: EndpointOut<'a, B>,
    standard_bulkin: EndpointIn<'a,B>
}


impl<B: UsbBus> MidiClass<'_, B> {
    /// Creates a new MidiClass with the provided UsbBus
    pub fn new(alloc: &UsbBusAllocator<B>) -> MidiClass<'_, B> {
        MidiClass {
            standard_ac: alloc.interface(),
            standard_mc: alloc.interface(),
            //standard_bulkout : alloc.bulk(64),
            standard_bulkin: alloc.bulk(64)
        }
    }

    pub fn send_message(&mut self, usb_midi:UsbMidiEventPacket) -> Result<usize> {
        let bytes : [u8;4] = usb_midi.into();
        self.standard_bulkin.write(&bytes)
    }

    pub fn note_on(&mut self, chan: u8, note: Note, vel : u8) -> Result<usize> {
        let note = note as u8;
        self.standard_bulkin.write(
            &[
                0x09,//Note-on message (usb-midi)
                0x90 | (chan & 0x0f),// (note-on, normal midi)
                note & 0x7f, //note
                vel & 0x7f //vel
            ])
    }

      pub fn note_off(&mut self, chan: u8, note: Note, vel : u8) -> Result<usize> {
        let note = note as u8;
        self.standard_bulkin.write(
            &[
                0x08,//Note-on message (usb-midi)
                0x80 | (chan & 0x0f),// (note-on, normal midi)
                note & 0x7f, //note
                vel & 0x7f //vel
            ])
    }

}

impl<B: UsbBus> UsbClass<B> for MidiClass<'_, B> {

     fn get_configuration_descriptors(&self, writer: &mut DescriptorWriter) -> Result<()> {
        
        //AUDIO CONTROL STANDARD

        writer.interface(
            self.standard_ac,
            USB_AUDIO_CLASS,
            USB_AUDIOCONTROL_SUBCLASS,
            0 //no protocol,
        )?;

        // AUDIO CONTROL EXTRA INFO
        writer.write(
            CS_INTERFACE,
            &[
                HEADER_SUBTYPE,
                0x00,0x01, // REVISION
                0x09,0x00, //SIZE of class specific descriptions
                0x01, //Number of streaming interfaces
                0x01 // MIDIStreaming interface 1 belongs to this AC interface
            ]
        )?;

        //Streaming Standard

        writer.interface(
            self.standard_mc,
            USB_AUDIO_CLASS,
            USB_MIDISTREAMING_SUBCLASS,
            0, //no protocol
        )?; //Num endpoints?

        //Streaming extra info

        writer.write(
            CS_INTERFACE,
            &[
                MS_HEADER_SUBTYPE,
                0x00,0x01, //REVISION
                (0x07 + MIDI_OUT_SIZE),0x00 //Total size of class specific descriptors? (little endian?)
            ]
        )?;
    
        //JACKS

/*         writer.write(
            CS_INTERFACE,
            &[
                MIDI_IN_JACK_SUBTYPE,
                EMBEDDED,
                0x01, // id
                0x00
            ]
        )?; */

        writer.write (
            CS_INTERFACE,
            &[
                MIDI_OUT_JACK_SUBTYPE,
                EMBEDDED,
                0x01,//id
                0x01, // 1 pin
                0x01, // pin 1
                0x01, //sorta vague source pin?
                0x00
            ]
        )?;

/*         writer.endpoint(&self.standard_bulkout)?;

        writer.write(
            CS_ENDPOINT,
            &[
                MS_GENERAL,
                0x01,
                0x01
            ]
        )?; */

        writer.endpoint(&self.standard_bulkin)?;

        writer.write(
            CS_ENDPOINT,
            &[
                MS_GENERAL,
                0x01,
                0x01
            ]
        )?;
        Ok(())
    }

}
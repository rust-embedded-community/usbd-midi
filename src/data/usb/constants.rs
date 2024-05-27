//! Constants for use in USB descriptors.

#![allow(missing_docs)]

// TODO: this should be better organized in future.

pub const USB_CLASS_NONE: u8 = 0x00;
pub const USB_AUDIO_CLASS: u8 = 0x01;
pub const USB_AUDIOCONTROL_SUBCLASS: u8 = 0x01;
pub const USB_MIDISTREAMING_SUBCLASS: u8 = 0x03;
pub const MIDI_IN_JACK_SUBTYPE: u8 = 0x02;
pub const MIDI_OUT_JACK_SUBTYPE: u8 = 0x03;
pub const EMBEDDED: u8 = 0x01;
pub const EXTERNAL: u8 = 0x02;
pub const CS_INTERFACE: u8 = 0x24;
pub const CS_ENDPOINT: u8 = 0x25;
pub const HEADER_SUBTYPE: u8 = 0x01;
pub const MS_HEADER_SUBTYPE: u8 = 0x01;
pub const MS_GENERAL: u8 = 0x01;

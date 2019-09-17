#![no_std]
mod midi_device;
mod notes;

pub use usb_device::{Result,UsbError};
pub use crate::midi_device::*;
pub use crate::notes::Note;
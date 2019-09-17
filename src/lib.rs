#![no_std]
mod usb_constants;
mod midi_device;
mod notes;

pub use usb_device::{Result,UsbError};
pub use crate::usb_constants::USB_CLASS_NONE;
pub use crate::midi_device::*;
pub use crate::notes::Note;
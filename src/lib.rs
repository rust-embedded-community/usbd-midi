#![no_std]
mod midi_device;

pub use usb_device::{Result,UsbError};
pub use crate::midi_device::*;
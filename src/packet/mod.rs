//! USB MIDI event packet parser and related types.

pub mod cable_number;
pub mod code_index_number;
pub mod reader;

use crate::packet::cable_number::CableNumber;
use crate::packet::code_index_number::CodeIndexNumber;

/// A packet that communicates with the host.
///
/// Currently supported is sending the specified normal midi
/// message over the supplied cable number
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct UsbMidiEventPacket {
    /// Raw packet data.
    raw: [u8; 4],
}

impl From<UsbMidiEventPacket> for [u8; 4] {
    fn from(value: UsbMidiEventPacket) -> [u8; 4] {
        value.raw
    }
}

/// Error variants for parsing the packet.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum UsbMidiEventPacketError {
    /// Invalid packet.
    InvalidPacket,
    /// Invalid note.
    InvalidNote(u8),
    /// Invalid cable number.
    InvalidCableNumber(u8),
    /// Invalid code index number.
    InvalidCodeIndexNumber(u8),
    /// Invalid event type.
    InvalidEventType(u8),
    /// Missing data packet.
    MissingDataPacket,
    /// Empty payload.
    EmptyPayload,
    /// Invalid payload status.
    InvalidPayloadStatus,
    /// Invalid payload size.
    InvalidPayloadSize,
}

impl TryFrom<&[u8]> for UsbMidiEventPacket {
    type Error = UsbMidiEventPacketError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let Ok(raw) = value.try_into() else {
            return Err(UsbMidiEventPacketError::InvalidPacket);
        };

        Ok(Self { raw })
    }
}

impl UsbMidiEventPacket {
    /// Returns the cable number.
    pub fn cable_number(&self) -> CableNumber {
        let raw_cable_number = self.raw[0] >> 4;

        // Unwrap can't fail because of limited `raw_cable_number` value range.
        CableNumber::try_from(raw_cable_number).unwrap_or_default()
    }

    /// Returns the header byte.
    pub fn header(&self) -> u8 {
        self.raw[0]
    }

    /// Returns a slice to the event payload bytes. The length is dependent on the payload type.
    pub fn payload_bytes(&self) -> &[u8] {
        let raw_cin = self.raw[0] & 0x0F;

        match CodeIndexNumber::try_from(raw_cin) {
            Ok(cin) => {
                let size = cin.payload_size();
                &self.raw[1..1 + size]
            }

            // Can't happen because of limited `raw_cin` value range.
            Err(_) => &[],
        }
    }

    /// Returns a reference to the raw bytes.
    pub fn as_raw_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Returns the raw bytes as owned array.
    pub fn to_raw_bytes(&self) -> [u8; 4] {
        self.raw
    }

    /// Creates a packet from a slice of event payload bytes.
    pub fn try_from_payload_bytes(
        cable: CableNumber,
        bytes: &[u8],
    ) -> Result<Self, UsbMidiEventPacketError> {
        let cin = CodeIndexNumber::try_from_payload(bytes)?;
        let payload_size = cin.payload_size();

        if bytes.len() < payload_size {
            return Err(UsbMidiEventPacketError::InvalidPayloadSize);
        }

        let mut raw = [0; 4];
        raw[0] = (cable as u8) << 4 | cin as u8;
        raw[1..1 + payload_size].copy_from_slice(&bytes[..payload_size]);

        Ok(Self { raw })
    }

    /// Returns if the packet payload is part of a SysEx message.
    pub fn is_sysex(&self) -> bool {
        let Ok(cin) = CodeIndexNumber::try_from(self.raw[0] & 0x0F) else {
            return false;
        };

        match cin {
            CodeIndexNumber::SysexStartsOrContinues
            | CodeIndexNumber::SysexEnds2Bytes
            | CodeIndexNumber::SysexEnds3Bytes => true,
            CodeIndexNumber::SystemCommon1Byte => self.raw[1] == 0xF7,
            CodeIndexNumber::SingleByte => self.raw[1] < 0x80,
            _ => false,
        }
    }

    /// Returns if the packet payload contains the start of a SysEx message.
    pub fn is_sysex_start(&self) -> bool {
        self.is_sysex() && self.raw[1] == 0xF0
    }

    /// Returns if the packet payload contains the end of a SysEx message.
    pub fn is_sysex_end(&self) -> bool {
        self.is_sysex() && (self.raw[1] == 0xF7 || self.raw[2] == 0xF7 || self.raw[3] == 0xF7)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod decode {
        use super::*;

        macro_rules! decode_packet_test {
            ($($id:ident: $value:expr,)*) => {
                $(
                    #[test]
                    fn $id() {
                        let (raw, expected) = $value;
                        let expected = (
                            expected.0,
                            expected.1.as_slice(),
                            expected.2, expected.3,
                            expected.4
                        );
                        let packet = UsbMidiEventPacket::try_from(raw.as_slice()).unwrap();
                        let decoded = (
                            packet.cable_number(),
                            packet.payload_bytes(),
                            packet.is_sysex(),
                            packet.is_sysex_start(),
                            packet.is_sysex_end()
                        );
                        assert_eq!(decoded, expected);
                    }
                )*
            }
        }

        decode_packet_test! {
            note_off: ([0x18, 0x80, 60, 64], (CableNumber::Cable1, [0x80, 60, 64], false, false, false)),
            note_on: ([0x49, 0x92, 48, 20], (CableNumber::Cable4, [0x92, 48, 20], false, false, false)),
            poly_key_press: ([0x0A, 0xA0, 15, 74], (CableNumber::Cable0, [0xA0, 15, 74], false, false, false)),
            control_change: ([0x0B, 0xB0, 64, 127], (CableNumber::Cable0, [0xB0, 64, 127], false, false, false)),
            program_change: ([0x0C, 0xC0, 5, 0], (CableNumber::Cable0, [0xC0, 5], false, false, false)),
            channel_pressure: ([0x0D, 0xD0, 85, 0], (CableNumber::Cable0, [0xD0, 85], false, false, false)),
            pitch_bend: ([0x0E, 0xE0, 40, 120], (CableNumber::Cable0, [0xE0, 40, 120], false, false, false)),
            mtc_quarter_frame: ([0x02, 0xF1, 27, 0], (CableNumber::Cable0, [0xF1, 27], false, false, false)),
            song_position_pointer: ([0x03, 0xF2, 38, 17], (CableNumber::Cable0, [0xF2, 38, 17], false, false, false)),
            song_select: ([0x02, 0xF3, 2, 0], (CableNumber::Cable0, [0xF3, 2], false, false, false)),
            tune_request: ([0x05, 0xF6, 0, 0], (CableNumber::Cable0, [0xF6], false, false, false)),
            timing_clock: ([0x0F, 0xF8, 0, 0], (CableNumber::Cable0, [0xF8], false, false, false)),
            tick: ([0x0F, 0xF9, 0, 0], (CableNumber::Cable0, [0xF9], false, false, false)),
            start: ([0x0F, 0xFA, 0, 0], (CableNumber::Cable0, [0xFA], false, false, false)),
            continue_: ([0x0F, 0xFB, 0, 0], (CableNumber::Cable0, [0xFB], false, false, false)),
            stop: ([0x0F, 0xFC, 0, 0], (CableNumber::Cable0, [0xFC], false, false, false)),
            active_sensing: ([0x0F, 0xFE, 0, 0], (CableNumber::Cable0, [0xFE], false, false, false)),
            system_reset: ([0x0F, 0xFF, 0, 0], (CableNumber::Cable0, [0xFF], false, false, false)),
            sysex_starts: ([0x04, 0xF0, 1, 2], (CableNumber::Cable0, [0xF0, 1, 2], true, true, false)),
            sysex_continues_1byte: ([0x0F, 1, 0, 0], (CableNumber::Cable0, [1], true, false, false)),
            sysex_continues_3bytes: ([0x04, 1, 2, 3], (CableNumber::Cable0, [1, 2, 3], true, false, false)),
            sysex_ends_1byte: ([0x05, 0xF7, 0, 0], (CableNumber::Cable0, [0xF7], true, false, true)),
            sysex_ends_2bytes: ([0x06, 1, 0xF7, 0], (CableNumber::Cable0, [1, 0xF7], true, false, true)),
            sysex_ends_3bytes: ([0x07, 1, 2, 0xF7], (CableNumber::Cable0, [1, 2, 0xF7], true, false, true)),
            sysex_2bytes: ([0x06, 0xF0, 0xF7, 0], (CableNumber::Cable0, [0xF0, 0xF7], true, true, true)),
            sysex_3bytes: ([0x07, 0xF0, 1, 0xF7], (CableNumber::Cable0, [0xF0, 1, 0xF7], true, true, true)),
            undefined_f4: ([0x02, 0xF4, 1, 0], (CableNumber::Cable0, [0xF4, 1], false, false, false)),
            undefined_f5: ([0x03, 0xF5, 1, 2], (CableNumber::Cable0, [0xF5, 1, 2], false, false, false)),
            empty: ([0x00, 0, 0, 0], (CableNumber::Cable0, [0, 0, 0], false, false, false)),
        }
    }

    mod encode {
        use super::*;

        macro_rules! encode_packet_test {
            ($($id:ident: $value:expr,)*) => {
                $(
                    #[test]
                    fn $id() {
                        let ((cable, payload), expected) = $value;
                        let payload = payload.as_slice();
                        let encoded = UsbMidiEventPacket::try_from_payload_bytes(cable, payload);
                        let expected: Result<[u8; 4], UsbMidiEventPacketError> = expected;
                        assert_eq!(encoded, expected.map(
                            |v| UsbMidiEventPacket::try_from(v.as_slice()).unwrap())
                        );
                    }
                )*
            }
        }

        encode_packet_test! {
            note_off: ((CableNumber::Cable3, [0x80, 33, 75]), Ok([0x38, 0x80, 33, 75])),
            note_on: ((CableNumber::Cable2, [0x96, 67, 14]), Ok([0x29, 0x96, 67, 14])),
            poly_key_press: ((CableNumber::Cable0, [0xA0, 48, 72]), Ok([0x0A, 0xA0, 48, 72])),
            control_change: ((CableNumber::Cable0, [0xB0, 10, 127]), Ok([0x0B, 0xB0, 10, 127])),
            program_change: ((CableNumber::Cable0, [0xC0, 36]), Ok([0x0C, 0xC0, 36, 0])),
            program_change_extra: ((CableNumber::Cable0, [0xC0, 36, 54]), Ok([0x0C, 0xC0, 36, 0])),
            channel_pressure: ((CableNumber::Cable0, [0xD0, 115]), Ok([0x0D, 0xD0, 115, 0])),
            channel_pressure_extra: ((CableNumber::Cable0, [0xD0, 115, 27]), Ok([0x0D, 0xD0, 115, 0])),
            pitch_bend: ((CableNumber::Cable0, [0xE0, 93, 46]), Ok([0x0E, 0xE0, 93, 46])),
            mtc_quarter_frame: ((CableNumber::Cable0, [0xF1, 102, 46]), Ok([0x02, 0xF1, 102, 0])),
            mtc_quarter_frame_extra: ((CableNumber::Cable0, [0xF1, 102, 46, 7]), Ok([0x02, 0xF1, 102, 0])),
            song_position_pointer: ((CableNumber::Cable0, [0xF2, 42, 74]), Ok([0x03, 0xF2, 42, 74])),
            song_select: ((CableNumber::Cable0, [0xF3, 24]), Ok([0x02, 0xF3, 24, 0])),
            song_select_extra: ((CableNumber::Cable0, [0xF3, 24, 96]), Ok([0x02, 0xF3, 24, 0])),
            tune_request: ((CableNumber::Cable0, [0xF6]), Ok([0x05, 0xF6, 0, 0])),
            tune_request_extra: ((CableNumber::Cable0, [0xF6, 67, 72]), Ok([0x05, 0xF6, 0, 0])),
            timing_clock: ((CableNumber::Cable0, [0xF8]), Ok([0x0F, 0xF8, 0, 0])),
            timing_clock_extra: ((CableNumber::Cable0, [0xF8, 38, 126]), Ok([0x0F, 0xF8, 0, 0])),
            tick: ((CableNumber::Cable0, [0xF9]), Ok([0x0F, 0xF9, 0, 0])),
            start: ((CableNumber::Cable0, [0xFA]), Ok([0x0F, 0xFA, 0, 0])),
            continue_: ((CableNumber::Cable0, [0xFB]), Ok([0x0F, 0xFB, 0, 0])),
            stop: ((CableNumber::Cable0, [0xFC]), Ok([0x0F, 0xFC, 0, 0])),
            active_sensing: ((CableNumber::Cable0, [0xFE]), Ok([0x0F, 0xFE, 0, 0])),
            system_reset: ((CableNumber::Cable0, [0xFF]), Ok([0x0F, 0xFF, 0, 0])),
            sysex_starts: ((CableNumber::Cable0, [0xF0, 1, 2]), Ok([0x04, 0xF0, 1, 2])),
            sysex_starts_1byte: ((CableNumber::Cable0, [0xF0]), Err(UsbMidiEventPacketError::InvalidPayloadSize)),
            sysex_starts_2bytes: ((CableNumber::Cable0, [0xF0, 1]), Err(UsbMidiEventPacketError::InvalidPayloadSize)),
            sysex_continues_1byte: ((CableNumber::Cable0, [1]), Ok([0x0F, 1, 0, 0])),
            sysex_continues_2bytes: ((CableNumber::Cable0, [1, 2]), Err(UsbMidiEventPacketError::InvalidPayloadSize)),
            sysex_continues_3bytes: ((CableNumber::Cable0, [1, 2, 3]), Ok([0x04, 1, 2, 3])),
            sysex_ends_1byte: ((CableNumber::Cable0, [0xF7]), Ok([0x05, 0xF7, 0, 0])),
            sysex_ends_2bytes: ((CableNumber::Cable0, [1, 0xF7]), Ok([0x06, 1, 0xF7, 0])),
            sysex_ends_3bytes: ((CableNumber::Cable0, [1, 2, 0xF7]), Ok([0x07, 1, 2, 0xF7])),
            sysex_2bytes: ((CableNumber::Cable0, [0xF0, 0xF7]), Ok([0x06, 0xF0, 0xF7, 0])),
            sysex_3bytes: ((CableNumber::Cable0, [0xF0, 1, 0xF7]), Ok([0x07, 0xF0, 1, 0xF7])),
            undefined_f4: ((CableNumber::Cable0, [0xF4]), Err(UsbMidiEventPacketError::InvalidPayloadStatus)),
            undefined_f5: ((CableNumber::Cable0, [0xF5]), Err(UsbMidiEventPacketError::InvalidPayloadStatus)),
            note_off_missing_1byte: ((CableNumber::Cable3, [0x80, 26]), Err(UsbMidiEventPacketError::InvalidPayloadSize)),
            note_off_missing_2bytes: ((CableNumber::Cable3, [0x80]), Err(UsbMidiEventPacketError::InvalidPayloadSize)),
            empty: ((CableNumber::Cable0, []), Err(UsbMidiEventPacketError::EmptyPayload)),
        }
    }
}

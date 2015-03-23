// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use std::num::from_u8;

use super::types::{Channel, U7, U14};

/// 7 bit mask
#[inline(always)]
pub fn mask7(input: u8) -> U7 {
    input & 0b01111111
}

/// 14 bit mask
#[inline(always)]
pub fn mask14(input: u16) -> U14 {
    input & 0b0011111111111111
}

/// Extract the MSB and LSB from a `U14`
#[inline]
pub fn u14_to_msb_lsb(input: U14) -> (U7, U7) {
    let msb = mask7((input >> 7) as U7);
    let lsb = mask7(input as u8);
    (msb, lsb)
}

/// Convert an MSB and LSB to a `U14`
#[inline]
pub fn msb_lsb_to_u14(msb: U7, lsb: U7) -> U14 {
    ((mask7(msb) as U14) << 7) + mask7(lsb) as U14
}

/// Calculate the status byte for a given channel no.
#[inline(always)]
pub fn status_byte(status: u8, channel: Channel) -> u8 {
    (status & 0b00001111) * 16 + (channel as u8)
}

/// Seperate the status from the channel no.
#[inline]
pub fn from_status_byte(sb: u8) -> (u8, Channel) {
    let status = (sb & 0b11110000) >> 4;
    let channel = from_u8(sb & 0b00001111).unwrap();
    (status, channel)
}


#[cfg(test)]
mod tests {
    use super::*;
    use constants::*;
    use types::Channel;

    #[test]
    fn test_mask7() {
        assert_eq!(127, mask7(255));
        assert_eq!(126, mask7(254));
        assert_eq!(127, mask7(127));
        assert_eq!(126, mask7(126));
    }

    #[test]
    fn test_all_mask7() {
        for i in 0..255 { // should be 0..256
            if i > 127 {
                assert_eq!(mask7(i), i - 128);
            }
            else {
                assert_eq!(mask7(i), i);
            }
        }
    }

    #[test]
    fn test_mask14() {
        assert_eq!(16383, mask14(65535));
        assert_eq!(16382, mask14(65534));
        assert_eq!(16383, mask14(16383));
        assert_eq!(16382, mask14(16382));
    }

    #[test]
    fn test_all_mask14() {
        for i in 0..65535 { // should be 0..65536
            if i > 16383 {
                assert!(mask14(i) != i);
            }
            else {
                assert_eq!(mask14(i), i);
            }
        }
    }

    #[test]
    fn test_msb_lsb() {
        // data from: http://mididesigner.com/help/midi-byte-calculator/
        //
        assert_eq!(0, msb_lsb_to_u14(0, 0));
        assert_eq!((0, 0), u14_to_msb_lsb(0));

        assert_eq!(16383, msb_lsb_to_u14(127, 127));
        assert_eq!((127, 127), u14_to_msb_lsb(16383));

        assert_eq!(14442, msb_lsb_to_u14(112, 106));
        assert_eq!((112, 106), u14_to_msb_lsb(14442));

        assert_eq!(24, msb_lsb_to_u14(0, 24));
        assert_eq!((0, 24), u14_to_msb_lsb(24));
    }

    #[test]
    fn test_all_msb_lsb() {
        for i in 0..65535 { // should be 0..65536
            let (msb, lsb) = u14_to_msb_lsb(i);
            assert_eq!(msb_lsb_to_u14(msb, lsb), mask14(i))
        }
    }

    #[test]
    fn test_status_byte() {
        // data from: http://www.midi.org/techspecs/midimessages.php

        assert_eq!(128, status_byte(NOTE_OFF, Channel::Ch1));
        assert_eq!((NOTE_OFF, Channel::Ch1), from_status_byte(128));

        assert_eq!(155, status_byte(NOTE_ON, Channel::Ch12));
        assert_eq!((NOTE_ON, Channel::Ch12), from_status_byte(155));
    }

    #[test]
    fn test_all_status_byte() {
        use std::num::from_u8;
        for status in 0..16 {
            for ch in 0..16 {
                let channel = from_u8(ch).unwrap();
                let converted = from_status_byte(status_byte(status, channel));
                assert_eq!((status, channel), converted);
            }
        }
    }
}


// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use num::FromPrimitive;

pub type U7 = u8;
pub type U14 = u16;

/// Represents a Midi channel
///
/// Note than `Ch1 = 0`, `Ch2 = 1`, etc, as the actual protocol is 0-indexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Channel {
    Ch1  = 0 , Ch2  = 1 ,  Ch3 = 2 ,  Ch4 = 3 ,
    Ch5  = 4 , Ch6  = 5 ,  Ch7 = 6 ,  Ch8 = 7 ,
    Ch9  = 8 , Ch10 = 9 , Ch11 = 10, Ch12 = 11,
    Ch13 = 12, Ch14 = 13, Ch15 = 14, Ch16 = 15
}

impl FromPrimitive for Channel {
    fn from_u64(n: u64) -> Option<Self> {
        if n < 16 {
            FromPrimitive::from_i64(n as i64)
        }
        else {
            None
        }
    }

    fn from_i64(n: i64) -> Option<Self> {
        match n {
            0 => Some(Channel::Ch1),
            1 => Some(Channel::Ch2),
            2 => Some(Channel::Ch3),
            3 => Some(Channel::Ch4),
            4 => Some(Channel::Ch5),
            5 => Some(Channel::Ch6),
            6 => Some(Channel::Ch7),
            7 => Some(Channel::Ch8),
            8 => Some(Channel::Ch9),
            9 => Some(Channel::Ch10),
            10 => Some(Channel::Ch11),
            11 => Some(Channel::Ch12),
            12 => Some(Channel::Ch13),
            13 => Some(Channel::Ch14),
            14 => Some(Channel::Ch15),
            15 => Some(Channel::Ch16),
            _ => None
        }
    }
}


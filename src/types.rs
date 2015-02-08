// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

#[cfg(test)] use quickcheck::{Arbitrary, Gen};

pub type U7 = u8;
pub type U14 = u16;

/// Represents a Midi channel
///
/// Note than `Ch1 = 0`, `Ch2 = 1`, etc, as the actual protocol is 0-indexed.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, FromPrimitive)]
#[derive_Rand]
pub enum Channel {
    Ch1  = 0 , Ch2  = 1 ,  Ch3 = 2 ,  Ch4 = 3 ,
    Ch5  = 4 , Ch6  = 5 ,  Ch7 = 6 ,  Ch8 = 7 ,
    Ch9  = 8 , Ch10 = 9 , Ch11 = 10, Ch12 = 11,
    Ch13 = 12, Ch14 = 13, Ch15 = 14, Ch16 = 15
}

#[cfg(test)]
impl Arbitrary for Channel {
    fn arbitrary<G: Gen>(g: &mut G) -> Channel {
        g.gen()
    }
}

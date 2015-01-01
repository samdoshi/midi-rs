// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use types::U7;

#[deriving(Show, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum RawMessage {
    Status(u8),
    StatusData(u8, U7),
    StatusDataData(u8, U7, U7),
    /// Raw 8-bit data, useful for SysEx
    Raw(u8)
}


// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

//! Midi types and traits for Rust

#![feature(core, test)]

#[cfg(test)] extern crate test;

pub use types::{Channel, U7, U14};
pub use Channel::{Ch1,  Ch2,  Ch3,  Ch4,  Ch5,  Ch6,  Ch7,  Ch8,
                  Ch9,  Ch10, Ch11, Ch12, Ch13, Ch14, Ch15, Ch16};
pub use raw_message::RawMessage;
pub use RawMessage::{Status, StatusData, StatusDataData, Raw};
pub use message::Message;
pub use Message::{Start, TimingClock, Continue, Stop, ActiveSensing, SystemReset,
                  AllSoundOff, ResetAllControllers, LocalControlOff, LocalControlOn,
                  AllNotesOff, NoteOff,
                  ProgramChange, ControlChange, RPN7, RPN14, NRPN7, NRPN14,
                  SysEx, NoteOn, PitchBend, PolyphonicPressure, ChannelPressure};
pub use manufacturer::Manufacturer;
pub use to_raw_messages::ToRawMessages;

pub mod constants;
pub mod utils;

mod types;
mod raw_message;
mod message;
mod manufacturer;
mod to_raw_messages;


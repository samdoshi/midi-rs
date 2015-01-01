// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use constants::*;
use types::{U7, Channel};
use raw_message::{RawMessage};
use RawMessage::*;
use message::{Message};
use Message::*;
use utils::{mask7, status_byte, u14_to_msb_lsb};

/// Convert `self` to `Vec<RawMessage>`
///
/// A `Vec<RawMessage>` represents ordered Midi data that must be sent as a contigious
/// block, this is useful for representing `Message::SysEx` and `Message::NRPN14`,
/// note that midi clock messages are allowed to interrupt sysex messages as part of the spec.
pub trait ToRawMessages {
    fn to_raw_messages(&self) -> Vec<RawMessage>;
}

impl ToRawMessages for RawMessage {
    fn to_raw_messages(&self) -> Vec<RawMessage> {
        vec!(*self)
    }
}

impl ToRawMessages for Message {
    fn to_raw_messages(&self) -> Vec<RawMessage> {
        match self {
            // System realtime
            &Start => vec!(Status(START)),
            &TimingClock => vec!(Status(TIMING_CLOCK)),
            &Continue => vec!(Status(CONTINUE)),
            &Stop => vec!(Status(STOP)),
            &ActiveSensing => vec!(Status(ACTIVE_SENSING)),
            &SystemReset => vec!(Status(SYSTEM_RESET)),

            // Channel mode
            &AllSoundOff(ch) => ControlChange(ch, 120, 0).to_raw_messages(),
            &ResetAllControllers(ch) => ControlChange(ch, 121, 0).to_raw_messages(),
            &LocalControlOff(ch) => ControlChange(ch, 122, 0).to_raw_messages(),
            &LocalControlOn(ch) => ControlChange(ch, 122, 127).to_raw_messages(),
            &AllNotesOff(ch) => ControlChange(ch, 123, 0).to_raw_messages(),

            // Channel voice
            &ProgramChange(ch, no) => {
                let sb = status_byte(PROGRAM_CHANGE, ch);
                vec!(StatusData(sb, mask7(no)))
            },
            &ControlChange(ch, no, val) => {
                vec!(cc(ch, mask7(no), mask7(val)))
            },
            &RPN7(ch, rpn, val) => {
                let (rpn_msb, rpn_lsb) = u14_to_msb_lsb(rpn);
                vec!(
                    cc(ch, CC_RPN_MSB, rpn_msb),
                    cc(ch, CC_RPN_LSB, rpn_lsb),
                    cc(ch, CC_DATA_ENTRY_MSB, mask7(val))
                )
            },
            &RPN14(ch, rpn, val) => {
                let (rpn_msb, rpn_lsb) = u14_to_msb_lsb(rpn);
                let (val_msb, val_lsb) = u14_to_msb_lsb(val);
                vec!(
                    cc(ch, CC_RPN_MSB, rpn_msb),
                    cc(ch, CC_RPN_LSB, rpn_lsb),
                    cc(ch, CC_DATA_ENTRY_MSB, val_msb),
                    cc(ch, CC_DATA_ENTRY_LSB, val_lsb)
                )
            },
            &NRPN7(ch, nrpn, val) => {
                let (nrpn_msb, nrpn_lsb) = u14_to_msb_lsb(nrpn);
                vec!(
                    cc(ch, CC_NRPN_MSB, nrpn_msb),
                    cc(ch, CC_NRPN_LSB, nrpn_lsb),
                    cc(ch, CC_DATA_ENTRY_MSB, mask7(val))
                )
            },
            &NRPN14(ch, nrpn, val) => {
                let (nrpn_msb, nrpn_lsb) = u14_to_msb_lsb(nrpn);
                let (val_msb, val_lsb) = u14_to_msb_lsb(val);
                vec!(
                    cc(ch, CC_NRPN_MSB, nrpn_msb),
                    cc(ch, CC_NRPN_LSB, nrpn_lsb),
                    cc(ch, CC_DATA_ENTRY_MSB, val_msb),
                    cc(ch, CC_DATA_ENTRY_LSB, val_lsb)
                )
            },
            &SysEx(manufacturer, ref data) => {
                let mut output = Vec::new();
                output.push(SYSEX);
                output.push_all(manufacturer.to_u7s().as_slice());
                output.extend(data.iter().map(|d| mask7(*d)));
                output.push(SYSEX_EOX);
                output.into_iter().map(|d| Raw(d)).collect()
            },
            &NoteOff(ch, no, vel) => {
                let sb = status_byte(NOTE_OFF, ch);
                vec!(StatusDataData(sb, mask7(no), mask7(vel)))
            },
            &NoteOn(ch, no, vel) => {
                let sb = status_byte(NOTE_ON, ch);
                vec!(StatusDataData(sb, mask7(no), mask7(vel)))
            },
            &PitchBend(ch, bend) => {
                let sb = status_byte(PITCH_BEND, ch);
                let (msb, lsb) = u14_to_msb_lsb(bend);
                vec!(StatusDataData(sb, lsb, msb))
            }
            &PolyphonicPressure(ch, no, vel) => {
                let sb = status_byte(POLYPHONIC_PRESSURE, ch);
                vec!(StatusDataData(sb, mask7(no), mask7(vel)))
            },
            &ChannelPressure(ch, vel) => {
                let sb = status_byte(CHANNEL_PRESSURE, ch);
                vec!(StatusData(sb, mask7(vel)))
            }
        }
    }
}

// we need to generate a lot of CC messages...
fn cc(ch: Channel, cc_no: U7, val: U7) -> RawMessage {
    let sb = status_byte(CONTROL_CHANGE, ch);
    StatusDataData(sb, cc_no, val)
}

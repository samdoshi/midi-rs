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
                output.extend(manufacturer.to_u7s());
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

#[cfg(test)]
mod test {
    use super::ToRawMessages;
    use message::Message::*;
    use raw_message::RawMessage::*;
    use manufacturer::Manufacturer::*;
    use types::Channel::*;

    #[test]
    fn test_message_to_raw_messages() {
        // Where possible these numbers have been pasted in from
        // http://www.midi.org/techspecs/midimessages.php

        // Start
        assert_eq!(Start.to_raw_messages(), vec![Status(0b11111010)]);

        // TimingClock
        assert_eq!(TimingClock.to_raw_messages(), vec![Status(0b11111000)]);

        // Continue
        assert_eq!(Continue.to_raw_messages(), vec![Status(0b11111011)]);

        // Stop
        assert_eq!(Stop.to_raw_messages(), vec![Status(0b11111100)]);

        // ActiveSensing
        assert_eq!(ActiveSensing.to_raw_messages(), vec![Status(0b11111110)]);

        // SystemReset
        assert_eq!(SystemReset.to_raw_messages(), vec![Status(0b11111111)]);

        // AllSoundOff
        assert_eq!(AllSoundOff(Ch1).to_raw_messages(), vec![StatusDataData(176, 120, 0)]);

        // ResetAllControllers
        assert_eq!(ResetAllControllers(Ch1).to_raw_messages(), vec![StatusDataData(176, 121, 0)]);

        // LocalControlOff
        assert_eq!(LocalControlOff(Ch1).to_raw_messages(), vec![StatusDataData(176, 122, 0)]);

        // LocalControlOn
        assert_eq!(LocalControlOn(Ch1).to_raw_messages(), vec![StatusDataData(176, 122, 127)]);

        // AllNotesOff
        assert_eq!(AllNotesOff(Ch1).to_raw_messages(), vec![StatusDataData(176, 123, 0)]);

        // ProgramChange
        assert_eq!(ProgramChange(Ch1, 0).to_raw_messages(), vec![StatusData(192, 0)]);
        assert_eq!(ProgramChange(Ch1, 127).to_raw_messages(), vec![StatusData(192, 127)]);
        assert_eq!(ProgramChange(Ch1, 128).to_raw_messages(), vec![StatusData(192, 0)]);

        // ControlChange
        assert_eq!(ControlChange(Ch1, 0, 0).to_raw_messages(), vec![StatusDataData(176, 0, 0)]);
        assert_eq!(ControlChange(Ch1, 0, 127).to_raw_messages(), vec![StatusDataData(176, 0, 127)]);
        assert_eq!(ControlChange(Ch1, 0, 128).to_raw_messages(), vec![StatusDataData(176, 0, 0)]);
        assert_eq!(ControlChange(Ch1, 127, 0).to_raw_messages(), vec![StatusDataData(176, 127, 0)]);
        assert_eq!(ControlChange(Ch1, 128, 0).to_raw_messages(), vec![StatusDataData(176, 0, 0)]);

        // RPN7
        assert_eq!(RPN7(Ch1, 1000, 0).to_raw_messages(), vec![StatusDataData(176, 101, 7),
                                                              StatusDataData(176, 100, 104),
                                                              StatusDataData(176, 6, 0)]);

        // RPN14
        assert_eq!(RPN14(Ch1, 1000, 1001).to_raw_messages(), vec![StatusDataData(176, 101, 7),
                                                                  StatusDataData(176, 100, 104),
                                                                  StatusDataData(176, 6, 7),
                                                                  StatusDataData(176, 38, 105)]);

        // NRPN7
        assert_eq!(NRPN7(Ch1, 1000, 0).to_raw_messages(), vec![StatusDataData(176, 99, 7),
                                                               StatusDataData(176, 98, 104),
                                                               StatusDataData(176, 6, 0)]);

        // NRPN14
        assert_eq!(NRPN14(Ch1, 1000, 1001).to_raw_messages(), vec![StatusDataData(176, 99, 7),
                                                                   StatusDataData(176, 98, 104),
                                                                   StatusDataData(176, 6, 7),
                                                                   StatusDataData(176, 38, 105)]);

        // SysEx
        assert_eq!(SysEx(OneByte(100), vec![1, 2, 3, 4]).to_raw_messages(),
                   vec![Raw(0b11110000),
                        Raw(100),
                        Raw(1), Raw(2), Raw(3), Raw(4),
                        Raw(0b11110111)]);

        assert_eq!(SysEx(OneByte(128), vec![1, 2, 3, 4, 128]).to_raw_messages(),
                   vec![Raw(0b11110000),
                        Raw(0),
                        Raw(1), Raw(2), Raw(3), Raw(4), Raw(0),
                        Raw(0b11110111)]);

        assert_eq!(SysEx(ThreeByte(100, 101, 128), vec![1, 2, 3, 4]).to_raw_messages(),
                   vec![Raw(0b11110000),
                        Raw(100), Raw(101), Raw(0),
                        Raw(1), Raw(2), Raw(3), Raw(4),
                        Raw(0b11110111)]);

        // NoteOff
        assert_eq!(NoteOff(Ch1, 0, 0).to_raw_messages(), vec![StatusDataData(128, 0, 0)]);
        assert_eq!(NoteOff(Ch2, 127, 127).to_raw_messages(), vec![StatusDataData(129, 127, 127)]);
        assert_eq!(NoteOff(Ch3, 128, 128).to_raw_messages(), vec![StatusDataData(130, 0, 0)]);

        // NoteOn
        assert_eq!(NoteOn(Ch4, 0, 0).to_raw_messages(), vec![StatusDataData(147, 0, 0)]);
        assert_eq!(NoteOn(Ch5, 127, 127).to_raw_messages(), vec![StatusDataData(148, 127, 127)]);
        assert_eq!(NoteOn(Ch6, 128, 128).to_raw_messages(), vec![StatusDataData(149, 0, 0)]);

        // PitchBend
        assert_eq!(PitchBend(Ch7, 0).to_raw_messages(), vec![StatusDataData(230, 0, 0)]);
        assert_eq!(PitchBend(Ch8, 1000).to_raw_messages(), vec![StatusDataData(231, 104, 7)]);
        assert_eq!(PitchBend(Ch9, 45000).to_raw_messages(), vec![StatusDataData(232, 72, 95)]);
        assert_eq!(PitchBend(Ch10, 12232).to_raw_messages(), vec![StatusDataData(233, 72, 95)]);

        // PolyphonicPressure
        assert_eq!(PolyphonicPressure(Ch11, 0, 0).to_raw_messages(),
                   vec![StatusDataData(170, 0, 0)]);
        assert_eq!(PolyphonicPressure(Ch12, 127, 127).to_raw_messages(),
                   vec![StatusDataData(171, 127, 127)]);
        assert_eq!(PolyphonicPressure(Ch13, 128, 128).to_raw_messages(),
                   vec![StatusDataData(172, 0, 0)]);

        // ChannelPressure
        assert_eq!(ChannelPressure(Ch14, 0).to_raw_messages(), vec![StatusData(221, 0)]);
        assert_eq!(ChannelPressure(Ch15, 127).to_raw_messages(), vec![StatusData(222, 127)]);
        assert_eq!(ChannelPressure(Ch16, 128).to_raw_messages(), vec![StatusData(223, 0)]);
    }
}


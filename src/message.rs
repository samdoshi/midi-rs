// Copyright 2015 Sam Doshi (sam@metal-fish.co.uk)
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use types::{U7, U14, Channel};
use manufacturer::Manufacturer;

/// Defines the various Midi messages that can be sent
///
/// The variants are ordered such that they may be sorted and sent in a sensible order when they
/// occur at the same time, thus `NoteOff` before `NoteOn`, `Start` before `TimingClock`,
/// `ControlChange` and `ProgramChange` before `NoteOn`, etc, etc
#[deriving(Show, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Message {
    // System realtime
    // ---------------

    /// Start. Start the current sequence playing.
    /// (This message will be followed with Timing Clocks).
    Start,

    /// Timing Clock. Sent 24 times per quarter note when synchronization
    /// is required.
    TimingClock,

    /// Continue. Continue at the point the sequence was Stopped
    Continue,

    /// Stop. Stop the current sequence.
    Stop,

    /// Active Sensing. This message is intended to be sent repeatedly to tell the receiver that a
    /// connection is alive. Use of this message is optional. When initially received, the receiver
    /// will expect to receive another Active Sensing message each 300ms (max), and if it does not
    /// then it will assume that the connection has been terminated. At termination, the receiver
    /// will turn off all voices and return to normal (non- active sensing) operation.
    ActiveSensing,

    /// Reset. Reset all receivers in the system to power-up status. This should be used sparingly,
    /// preferably under manual control. In particular, it should not be sent on power-up.
    SystemReset,

    // Channel mode
    // ------------

    /// All Sound Off. When All Sound Off is received all oscillators will turn off, and their
    /// volume envelopes are set to zero as soon as possible.
    AllSoundOff(Channel),

    /// Reset All Controllers. When Reset All Controllers is received, all controller values are
    /// reset to their default values. (See specific Recommended Practices for defaults).
    ResetAllControllers(Channel),

    /// When Local Control is Off, all devices on a given channel will respond only to data
    /// received over MIDI. Played data, etc. will be ignored.
    LocalControlOff(Channel),

    /// Local Control On restores the functions of the normal controllers.
    LocalControlOn(Channel),

    /// All Notes Off. When an All Notes Off is received, all oscillators will turn off.
    ///
    /// (if you need to use one of the more obsure all notes off modes, send the direct
    /// `ControlChange` message
    AllNotesOff(Channel),

    // Channel voice
    // -------------

    /// Note Off event. This message is sent when a note is released (ended).
    /// The second argument is the key (note) number.
    /// The third argument is the velocity.
    NoteOff(Channel, U7, U7),

    /// Program Change. This message sent when the patch number changes.
    /// The second argument is the new program number.
    ProgramChange(Channel, U7),

    /// Control Change.  This message is sent when a controller value changes.
    /// The second argument is the controller number (0-119, though 0-127 is allowed).
    /// The third argument is the controller value (0-127).
    ControlChange(Channel, U7, U7),

    /// 7-bit RPN. This message is sent when a 7-bit RPN changes.
    /// The second argument is the RPN.
    /// The third argument is the value.
    RPN7(Channel, U14, U7),

    /// 14-bit RPN. This message is sent when a 14-bit RPN changes.
    /// The second argument is the RPN.
    /// The third argument is the value.
    RPN14(Channel, U14, U14),

    /// 7-bit NRPN. This message is sent when a 7-bit NRPN changes.
    /// The second argument is the NRPN.
    /// The third argument is the value.
    NRPN7(Channel, U14, U7),

    /// 14-bit NRPN. This message is sent when a 14-bit NRPN changes.
    /// The second argument is the NRPN.
    /// The third argument is the value.
    NRPN14(Channel, U14, U14),

    /// System Exclusive. This message type allows manufacturers to create their own messages (such
    /// as bulk dumps, patch parameters, and other non-spec data) and provides a mechanism for
    /// creating additional MIDI Specification messages.
    /// The first argument indicates the manufacturer.
    /// The second argument contains the data (without the `F0` header, or `F7` terminator).
    SysEx(Manufacturer, Vec<U7>),

    /// Note On event. This message is sent when a note is depressed (start).
    /// The second argument is the key (note) number.
    /// The third is the velocity.
    NoteOn(Channel, U7, U7),

    /// Pitch Bend Change. This message is sent to indicate a change in the pitch bender
    /// (wheel or lever, typically). The pitch bender is measured by a fourteen bit value. Center
    /// (no pitch change) is 2000H.
    PitchBend(Channel, U14),

    /// Polyphonic Key Pressure (Aftertouch). This message is most often sent by pressing down
    /// on the key after it "bottoms out".
    /// The second argument is the key (note) number.
    /// The third argument is the pressure value.
    PolyphonicPressure(Channel, U7, U7),

    /// Channel Pressure (Aftertouch). This message is most often sent by pressing down on the key
    /// after it "bottoms out". This message is different from polyphonic after-touch. Use this
    /// message to send the single greatest pressure value (of all the current depressed keys).
    /// The second argument is the pressure value.
    ChannelPressure(Channel, U7)
}


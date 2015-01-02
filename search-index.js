var searchIndex = {};
searchIndex['midi'] = {"items":[[0,"","midi","Midi types and traits for Rust"],[2,"Channel","","Represents a Midi channel"],[12,"Ch1","","",0],[12,"Ch2","","",0],[12,"Ch3","","",0],[12,"Ch4","","",0],[12,"Ch5","","",0],[12,"Ch6","","",0],[12,"Ch7","","",0],[12,"Ch8","","",0],[12,"Ch9","","",0],[12,"Ch10","","",0],[12,"Ch11","","",0],[12,"Ch12","","",0],[12,"Ch13","","",0],[12,"Ch14","","",0],[12,"Ch15","","",0],[12,"Ch16","","",0],[2,"RawMessage","",""],[12,"Status","","",1],[12,"StatusData","","",1],[12,"StatusDataData","","",1],[12,"Raw","","Raw 8-bit data, useful for SysEx",1],[2,"Message","","Defines the various Midi messages that can be sent"],[12,"Start","","Start. Start the current sequence playing.\n(This message will be followed with Timing Clocks).",2],[12,"TimingClock","","Timing Clock. Sent 24 times per quarter note when synchronization\nis required.",2],[12,"Continue","","Continue. Continue at the point the sequence was Stopped",2],[12,"Stop","","Stop. Stop the current sequence.",2],[12,"ActiveSensing","","Active Sensing. This message is intended to be sent repeatedly to tell the receiver that a\nconnection is alive. Use of this message is optional. When initially received, the receiver\nwill expect to receive another Active Sensing message each 300ms (max), and if it does not\nthen it will assume that the connection has been terminated. At termination, the receiver\nwill turn off all voices and return to normal (non- active sensing) operation.",2],[12,"SystemReset","","Reset. Reset all receivers in the system to power-up status. This should be used sparingly,\npreferably under manual control. In particular, it should not be sent on power-up.",2],[12,"AllSoundOff","","All Sound Off. When All Sound Off is received all oscillators will turn off, and their\nvolume envelopes are set to zero as soon as possible.",2],[12,"ResetAllControllers","","Reset All Controllers. When Reset All Controllers is received, all controller values are\nreset to their default values. (See specific Recommended Practices for defaults).",2],[12,"LocalControlOff","","When Local Control is Off, all devices on a given channel will respond only to data\nreceived over MIDI. Played data, etc. will be ignored.",2],[12,"LocalControlOn","","Local Control On restores the functions of the normal controllers.",2],[12,"AllNotesOff","","All Notes Off. When an All Notes Off is received, all oscillators will turn off.",2],[12,"NoteOff","","Note Off event. This message is sent when a note is released (ended).\nThe second argument is the key (note) number.\nThe third argument is the velocity.",2],[12,"ProgramChange","","Program Change. This message sent when the patch number changes.\nThe second argument is the new program number.",2],[12,"ControlChange","","Control Change.  This message is sent when a controller value changes.\nThe second argument is the controller number (0-119, though 0-127 is allowed).\nThe third argument is the controller value (0-127).",2],[12,"RPN7","","7-bit RPN. This message is sent when a 7-bit RPN changes.\nThe second argument is the RPN.\nThe third argument is the value.",2],[12,"RPN14","","14-bit RPN. This message is sent when a 14-bit RPN changes.\nThe second argument is the RPN.\nThe third argument is the value.",2],[12,"NRPN7","","7-bit NRPN. This message is sent when a 7-bit NRPN changes.\nThe second argument is the NRPN.\nThe third argument is the value.",2],[12,"NRPN14","","14-bit NRPN. This message is sent when a 14-bit NRPN changes.\nThe second argument is the NRPN.\nThe third argument is the value.",2],[12,"SysEx","","System Exclusive. This message type allows manufacturers to create their own messages (such\nas bulk dumps, patch parameters, and other non-spec data) and provides a mechanism for\ncreating additional MIDI Specification messages.\nThe first argument indicates the manufacturer.\nThe second argument contains the data (without the `F0` header, or `F7` terminator).",2],[12,"NoteOn","","Note On event. This message is sent when a note is depressed (start).\nThe second argument is the key (note) number.\nThe third is the velocity.",2],[12,"PitchBend","","Pitch Bend Change. This message is sent to indicate a change in the pitch bender\n(wheel or lever, typically). The pitch bender is measured by a fourteen bit value. Center\n(no pitch change) is 2000H.",2],[12,"PolyphonicPressure","","Polyphonic Key Pressure (Aftertouch). This message is most often sent by pressing down\non the key after it \"bottoms out\".\nThe second argument is the key (note) number.\nThe third argument is the pressure value.",2],[12,"ChannelPressure","","Channel Pressure (Aftertouch). This message is most often sent by pressing down on the key\nafter it \"bottoms out\". This message is different from polyphonic after-touch. Use this\nmessage to send the single greatest pressure value (of all the current depressed keys).\nThe second argument is the pressure value.",2],[2,"Manufacturer","",""],[12,"OneByte","","",3],[12,"ThreeByte","","",3],[0,"constants","",""],[18,"NOTE_OFF","midi::constants",""],[18,"NOTE_ON","",""],[18,"POLYPHONIC_PRESSURE","",""],[18,"CONTROL_CHANGE","",""],[18,"PROGRAM_CHANGE","",""],[18,"CHANNEL_PRESSURE","",""],[18,"PITCH_BEND","",""],[18,"SYSEX","",""],[18,"MTC_QUARTER_FRAME","",""],[18,"SONG_POSITION_POINTER","",""],[18,"SONG_SELECT","",""],[18,"TUNE_REQUEST","",""],[18,"SYSEX_EOX","",""],[18,"TIMING_CLOCK","",""],[18,"START","",""],[18,"CONTINUE","",""],[18,"STOP","",""],[18,"ACTIVE_SENSING","",""],[18,"SYSTEM_RESET","",""],[18,"CC_RPN_MSB","",""],[18,"CC_RPN_LSB","",""],[18,"CC_NRPN_MSB","",""],[18,"CC_NRPN_LSB","",""],[18,"CC_DATA_ENTRY_MSB","",""],[18,"CC_DATA_ENTRY_LSB","",""],[0,"utils","midi",""],[3,"mask7","midi::utils","7 bit mask"],[3,"mask14","","14 bit mask"],[3,"u14_to_msb_lsb","","Extract the MSB and LSB from a `U14`"],[3,"msb_lsb_to_u14","","Convert an MSB and LSB to a `U14`"],[3,"status_byte","","Calculate the status byte for a given channel no."],[3,"from_status_byte","","Seperate the status from the channel no."],[10,"rand","midi","",0],[10,"from_i64","","",0],[10,"from_u64","","",0],[10,"partial_cmp","","",0],[10,"lt","","",0],[10,"le","","",0],[10,"gt","","",0],[10,"ge","","",0],[10,"cmp","","",0],[10,"eq","","",0],[10,"ne","","",0],[10,"clone","","",0],[10,"fmt","","",0],[10,"hash","","",1],[10,"partial_cmp","","",1],[10,"lt","","",1],[10,"le","","",1],[10,"gt","","",1],[10,"ge","","",1],[10,"cmp","","",1],[10,"eq","","",1],[10,"ne","","",1],[10,"clone","","",1],[10,"fmt","","",1],[10,"partial_cmp","","",2],[10,"lt","","",2],[10,"le","","",2],[10,"gt","","",2],[10,"ge","","",2],[10,"cmp","","",2],[10,"eq","","",2],[10,"ne","","",2],[10,"clone","","",2],[10,"fmt","","",2],[10,"hash","","",3],[10,"partial_cmp","","",3],[10,"lt","","",3],[10,"le","","",3],[10,"gt","","",3],[10,"ge","","",3],[10,"cmp","","",3],[10,"eq","","",3],[10,"ne","","",3],[10,"clone","","",3],[10,"fmt","","",3],[10,"to_u7s","","",3],[10,"to_raw_messages","","",1],[10,"to_raw_messages","","",2],[4,"U7","",""],[4,"U14","",""],[6,"ToRawMessages","","Convert `self` to `Vec<RawMessage>`"],[9,"to_raw_messages","","",4]],"paths":[[2,"Channel"],[2,"RawMessage"],[2,"Message"],[2,"Manufacturer"],[6,"ToRawMessages"]]};
initSearch(searchIndex);

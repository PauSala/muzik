//! MIDI chord module.
use chordparser::{chord::Chord, voicings::generate_voicing};

/// Struct representing a chord in MIDI format
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiChord {
    /// MIDI codes for the chord notes.
    pub midi_codes: Vec<u8>,
    /// Start time of the chord, relative to previous MIDI event.
    pub start: u16,
    /// Duration of the chord in MIDI ticks.
    pub duration: u16,
}

impl MidiChord {
    pub fn new(ch: Chord, start: u16, duration: u16, lead: u8) -> MidiChord {
        MidiChord {
            midi_codes: generate_voicing(&ch, Some(lead)),
            start,
            duration,
        }
    }
}

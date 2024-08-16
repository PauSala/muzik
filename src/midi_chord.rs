use chordparser::{chord::Chord, voicings::generate_voicing};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MidiChord {
    pub midi_codes: Vec<u8>,
    pub start: u16,
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

use chordparser::parsing::{parser_error::ParserErrors, Parser};
use midly::{
    num::{u4, u7},
    Format, Header, MetaMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};

use crate::{
    midi_chord::MidiChord,
    parser::{
        lexer::Lexer,
        tokens::{Duration, Rest, TokenType},
    },
};

pub struct ChordCompingGenerator {
    /// Beats per minute
    bpm: u32,
    /// Ticks per beat
    tpb: u16,
    /// Higest note to build the voicing around
    lead_note: u8,
}

impl Default for ChordCompingGenerator {
    fn default() -> Self {
        Self::new(120, 74)
    }
}

impl ChordCompingGenerator {
    pub fn new(bpm: u32, lead_note: u8) -> ChordCompingGenerator {
        ChordCompingGenerator {
            bpm,
            lead_note,
            tpb: 500,
        }
    }

    // TODO: add whole, half, sixteenth
    // fn whole(&self) -> u16 {
    //     4 * self.tpb
    // }

    fn half(&self) -> u16 {
        2 * self.tpb
    }

    fn quarter(&self) -> u16 {
        self.tpb
    }

    fn eigth(&self) -> u16 {
        self.tpb / 2
    }

    pub fn with_wildcards(
        &self,
        i: &str,
        chords: &mut Vec<&str>,
        omit_bass: bool,
    ) -> Result<Smf, ParserErrors> {
        let mut parsed = String::new();
        chords.reverse();
        for c in i.chars() {
            if c == '*' {
                if let Some(ch) = chords.pop() {
                    parsed.push_str(ch);
                } else {
                    return Err(ParserErrors::new(vec![
                        "There are more wildcards than chords".to_string(),
                    ]));
                }
            } else {
                parsed.push(c);
            }
        }
        self.from_string(&parsed, omit_bass)
    }

    pub fn from_string(&self, i: &str, omit_bass: bool) -> Result<Smf, ParserErrors> {
        // microseconds x beat
        let mc_x_beat = 60 * 1_000_000 / self.bpm;
        let mut events = vec![];
        let tempo = midly::MetaMessage::Tempo(mc_x_beat.into());
        events.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(tempo),
        });

        let chords = self.chord_events(i)?;
        for mut ch in chords {
            if omit_bass {
                ch.midi_codes.remove(0);
            }
            self.add_midi_chord(&mut events, ch.start, ch.duration, &ch.midi_codes);
        }
        events.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
        });

        let mut track = Track::new();
        for event in events {
            track.push(event);
        }
        let smf = Smf {
            header: Header {
                format: Format::SingleTrack,
                timing: Timing::Metrical(midly::num::u15::new(self.tpb)),
            },
            tracks: vec![track],
        };
        Ok(smf)
    }

    fn chord_events(&self, i: &str) -> Result<Vec<MidiChord>, ParserErrors> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let tokens = lexer.scan_tokens(i);
        let mut chords: Vec<MidiChord> = Vec::new();
        let mut context: Option<MidiChord> = None;

        let mut start: u16 = 0;
        for t in tokens {
            match t.token_type {
                TokenType::Chord(ch) => {
                    let ch = parser.parse(&ch);
                    match ch {
                        Ok(ch) => context = Some(MidiChord::new(ch, 0, 0, self.lead_note)),
                        Err(e) => return Err(e),
                    }
                }
                TokenType::Rest(r) => match r {
                    Rest::Quarter => start += self.quarter(),
                    Rest::Eight => start += self.eigth(),
                    Rest::Half => start += self.half(),
                },
                TokenType::Duration(d) => {
                    let duration = match d {
                        Duration::Quarter => self.quarter(),
                        Duration::Eight => self.eigth(),
                        Duration::Half => self.half(),
                    };
                    if let Some(ref mut ctx) = context {
                        ctx.start = start;
                        ctx.duration = duration;
                        chords.push(ctx.clone());
                    }
                    start = 0;
                }
                TokenType::Eof => (),
            }
        }
        Ok(chords)
    }

    fn add_midi_chord(
        &self,
        events: &mut Vec<TrackEvent>,
        start: u16,
        duration: u16,
        midi_codes: &[u8],
    ) {
        let velocity = u7::new(64);
        // Start chord
        for (i, &note) in midi_codes.iter().enumerate() {
            events.push(TrackEvent {
                delta: if i == 0 {
                    (start as u32).into()
                } else {
                    0.into()
                },
                kind: TrackEventKind::Midi {
                    channel: u4::new(0),
                    message: midly::MidiMessage::NoteOn {
                        key: u7::new(note),
                        vel: velocity - (i as u8).into(),
                    },
                },
            });
        }
        // Stop chord after duration
        for (i, &note) in midi_codes.iter().enumerate() {
            events.push(TrackEvent {
                delta: if i == 0 {
                    (duration as u32).into()
                } else {
                    0.into()
                },
                kind: TrackEventKind::Midi {
                    channel: u4::new(0),
                    message: midly::MidiMessage::NoteOff {
                        key: u7::new(note),
                        vel: velocity - (i as u8).into(),
                    },
                },
            });
        }
    }
}

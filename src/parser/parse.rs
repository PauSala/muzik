use std::path::Path;

use chordparser::parsing::Parser;
use midly::{
    num::{u4, u7},
    Format, Header, MetaMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
};

use super::{lexer::Lexer, midi_chord::MidiChord};

pub struct ChordCompingGenerator {}

impl ChordCompingGenerator {
    pub fn new() -> ChordCompingGenerator {
        ChordCompingGenerator {}
    }
    fn quarter(&self, tpb: u16) -> u16 {
        tpb
    }

    fn eigth(&self, tpb: u16) -> u16 {
        tpb * 1 / 2
    }

    pub fn scan(&self, i: &str, bpm: u32) {
        // microseconds x beat
        let mc_x_beat = 60 * 1_000_000 / bpm;
        // Ticks per beat
        let tpb: u16 = 500;
        let mut events = vec![];
        let tempo = midly::MetaMessage::Tempo(mc_x_beat.into());
        events.push(TrackEvent {
            delta: 0.into(),
            kind: TrackEventKind::Meta(tempo),
        });

        let chords = self.chord_events(i, tpb);
        for ch in chords {
            self.add_midi_chord(&mut events, ch.start, ch.duration, &ch.ch);
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
                timing: Timing::Metrical(midly::num::u15::new(tpb)),
            },
            tracks: vec![track],
        };

        let path = Path::new("comping").with_extension("mid");
        let mut file = std::fs::File::create(path).unwrap();
        smf.write_std(&mut file).unwrap();
    }

    fn chord_events(&self, i: &str, tpb: u16) -> Vec<MidiChord> {
        let mut lexer = Lexer::new();
        let mut parser = Parser::new();
        let tokens = lexer.scan_tokens(i);
        let mut chords: Vec<MidiChord> = Vec::new();
        let mut context: Option<MidiChord> = None;

        let mut start: u16 = 0;
        for t in tokens {
            match t.token_type {
                super::tokens::TokenType::Chord(ch) => {
                    let ch = parser.parse(&ch).expect("This needs to be handled!");
                    context = Some(MidiChord::new(ch, 0, 0, 72));
                }
                super::tokens::TokenType::Rest(r) => match r {
                    super::tokens::Rest::QuarterRest => start += self.quarter(tpb),
                    super::tokens::Rest::EightRest => start += self.eigth(tpb),
                },
                super::tokens::TokenType::Duration(d) => match d {
                    super::tokens::Duration::Quarter => {
                        if let Some(ref mut ctx) = context {
                            ctx.start = start;
                            ctx.duration = self.quarter(tpb);
                            chords.push(ctx.clone());
                        }
                        start = 0;
                    }
                    super::tokens::Duration::Eight => {
                        if let Some(ref mut ctx) = context {
                            ctx.start = start;
                            ctx.duration = self.eigth(tpb);
                            chords.push(ctx.clone());
                        }
                        start = 0;
                    }
                },
                super::tokens::TokenType::Eof => (),
            }
        }
        chords
    }

    fn add_midi_chord(
        &self,
        events: &mut Vec<TrackEvent>,
        start: u16,
        duration: u16,
        chord_notes: &[u8],
    ) {
        let velocity = u7::new(64);
        // Start chord
        for (i, &note) in chord_notes.iter().enumerate() {
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
        for (i, &note) in chord_notes.iter().enumerate() {
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

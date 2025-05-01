use anyhow::Error;
use midly::{
    Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent, TrackEventKind,
    num::{u24, u28},
};
use std::{char::MAX, fs::File};

use crate::ast::*; // assuming this includes your parsed AST types
use midly::num::{u4, u7};
use std::collections::HashMap;

const MAX_NUMBER_OF_CHANNELS: u8 = 16;

pub struct MidiGen {
    songs: HashMap<String, Song>,
    sections: HashMap<String, Section>,
    instruments: HashMap<String, Instrument>,
    patterns: HashMap<String, Pattern>,
    time: u32,
}

impl MidiGen {
    pub fn new(ast: &[TopLevel]) -> Self {
        let mut songs: HashMap<String, Song> = HashMap::new();
        let mut sections: HashMap<String, Section> = HashMap::new();
        let mut instruments: HashMap<String, Instrument> = HashMap::new();
        let mut patterns: HashMap<String, Pattern> = HashMap::new();

        for node in ast {
            match node {
                TopLevel::Song(song) => {
                    songs.insert(song.name.clone(), song.clone());
                }
                TopLevel::Pattern(pattern) => {
                    patterns.insert(pattern.name.clone(), pattern.clone());
                }
                TopLevel::Section(section) => {
                    sections.insert(section.name.clone(), section.clone());
                }
                TopLevel::Instrument(instrument) => {
                    instruments.insert(instrument.name.clone(), instrument.clone());
                }
            }
        }
        MidiGen {
            songs: songs,
            sections: sections,
            instruments: instruments,
            patterns: patterns,
            time: 0u32,
        }
    }
    pub fn generate(&mut self) -> Vec<String> {
        let mut song_names = Vec::new();
        let songs = self.songs.clone();
        for song in songs.iter() {
            let (name, _) = song;
            self.time = 0u32;
            let file_name = self.generate_song(name).unwrap();
            song_names.push(file_name);
        }

        song_names
    }

    fn generate_song(&mut self, song_name: &String) -> Result<String, Error> {
        let mut tracks: Vec<Vec<TrackEvent>> = vec![Vec::new(); MAX_NUMBER_OF_CHANNELS.into()];
        for chan_i in 0..MAX_NUMBER_OF_CHANNELS {
            let mut track = Vec::new();

            // This means BPM 120
            let tempo: u24 = 500000.into();
            // Add tempo meta
            track.push(TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::Tempo(tempo)), // 120 BPM
            });
            tracks.push(track);
        }

        let section_refs: Vec<_> = self.songs.get(song_name).unwrap().entry_sections.clone();

        for section_name in section_refs {
            tracks = self.generate_section(&section_name, tracks)
        }

        for chan_i in 0..MAX_NUMBER_OF_CHANNELS {
            tracks[usize::try_from(chan_i).unwrap()].push(TrackEvent {
                delta: 0.into(),
                kind: TrackEventKind::Meta(MetaMessage::EndOfTrack),
            });
        }
        let smf = Smf {
            header: Header {
                format: Format::Parallel,
                timing: Timing::Metrical(480.into()),
            },
            tracks: tracks,
        };
        let file_name = format!("{}.mid", song_name);

        let mut out = File::create(file_name.clone())?;
        smf.write_std(&mut out)?;

        Ok(file_name)
    }

    fn generate_section<'a>(
        &mut self,
        section_name: &String,
        mut tracks: Vec<Vec<TrackEvent<'a>>>,
    ) -> Vec<Vec<TrackEvent<'a>>> {
        let section = self.sections.get(section_name).unwrap().clone();

        for i in 0..section.channels.len() {
            let channel = section.channels.get(i).unwrap();
            let mut time = 0u32;
            for pattern in channel.pattern_calls.iter() {
                let result = self.generate_pattern(pattern, i, tracks);
                time = self.time.max(result.0);
                tracks = result.1;
            }
            self.time = time;
        }

        tracks
    }
    fn generate_pattern<'a>(
        &mut self,
        pattern_name: &String,
        channel: usize,
        mut tracks: Vec<Vec<TrackEvent<'a>>>,
    ) -> (u32, Vec<Vec<TrackEvent<'a>>>) {
        let pattern = self.patterns.get(pattern_name).unwrap().clone();
        let mut time = self.time.clone();

        for event in pattern.events.iter() {
            match event {
                PatternEvent::Note { chord, duration } => {
                    let (start, end) = duration;
                    let duration = 120u32 * (u32::from(*end) - u32::from(*start));

                    let start_time = time + u32::from(*start);
                    let end_time = time + u32::from(*end);

                    let chord_events = chord_to_midi_events(
                        &chord,
                        start_time,
                        duration,
                        100,
                        channel.try_into().unwrap(),
                    );
                    tracks.get_mut(channel).unwrap().extend(chord_events);
                    time += end_time;
                }
                PatternEvent::Wait { duration } => {
                    let (start, end) = duration;
                    let duration = 120u32 * (u32::from(*end) - u32::from(*start));

                    let start_time = time + u32::from(*start);
                    let end_time = time + u32::from(*end);

                    let chord_events = chord_to_midi_events(
                        "Am",
                        start_time,
                        duration,
                        0,
                        channel.try_into().unwrap(),
                    );
                    tracks.get_mut(channel).unwrap().extend(chord_events);
                    time += end_time;
                }
            }
        }
        (time, tracks)
    }
}

pub fn chord_to_midi_events(
    chord: &str,
    start_time: u32,
    duration: u32,
    velocity: u8,
    channel: u8,
) -> Vec<TrackEvent<'static>> {
    let mut notes = match parse_chord(chord) {
        Some(n) => n,
        None => return vec![],
    };

    notes.sort();
    let mut events = Vec::new();

    // NoteOn events – all with delta = 0 except the first
    for (i, note) in notes.iter().enumerate() {
        events.push(TrackEvent {
            delta: if i == 0 { start_time.into() } else { 0.into() },
            kind: TrackEventKind::Midi {
                channel: u4::new(channel),
                message: MidiMessage::NoteOn {
                    key: u7::new(*note),
                    vel: u7::new(velocity),
                },
            },
        });
    }

    // NoteOff events – all with delta = 0 except the first
    for (i, note) in notes.iter().enumerate() {
        events.push(TrackEvent {
            delta: if i == 0 { duration.into() } else { 0.into() },
            kind: TrackEventKind::Midi {
                channel: u4::new(channel),
                message: MidiMessage::NoteOff {
                    key: u7::new(*note),
                    vel: u7::new(0),
                },
            },
        });
    }

    events
}

fn parse_chord(name: &str) -> Option<Vec<u8>> {
    let base_notes = HashMap::from([
        ("C", 60),
        ("C#", 61),
        ("D", 62),
        ("D#", 63),
        ("E", 64),
        ("F", 65),
        ("F#", 66),
        ("G", 67),
        ("G#", 68),
        ("A", 69),
        ("A#", 70),
        ("B", 71),
    ]);

    let (root, is_minor) = if name.ends_with('m') && name.len() > 1 {
        (&name[..name.len() - 1], true)
    } else {
        (name, false)
    };

    let root_midi = *base_notes.get(root)?;
    Some(if is_minor {
        vec![root_midi, root_midi + 3, root_midi + 7] // minor triad
    } else {
        vec![root_midi, root_midi + 4, root_midi + 7] // major triad
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_test_ast() -> Vec<TopLevel> {
        vec![
            TopLevel::Instrument(Instrument {
                midi_path: "midipath".to_string(),
                type_: "guitar".to_string(),
                name: "Piano".to_string(),
            }),
            TopLevel::Pattern(Pattern {
                name: "Pattern1".to_string(),
                events: vec![
                    PatternEvent::Note {
                        chord: "C".to_string(),
                        duration: (0, 1),
                    },
                    PatternEvent::Wait { duration: (1, 2) },
                ],
            }),
            TopLevel::Section(Section {
                name: "Section1".to_string(),
                channels: vec![Channel {
                    name: "x".to_string(),
                    pattern_calls: vec!["Pattern1".to_string()],
                }],
            }),
            TopLevel::Song(Song {
                name: "Song1".to_string(),
                entry_sections: vec!["Section1".to_string()],
            }),
        ]
    }

    #[test]
    fn test_chord_to_midi_events_major() {
        let events = chord_to_midi_events("C", 0, 480, 100, 0);
        assert_eq!(events.len(), 6); // 3 NoteOn + 3 NoteOff

        if let TrackEventKind::Midi { channel, message } = events[0].kind {
            match message {
                MidiMessage::NoteOn { key, vel } => {
                    assert_eq!(key.as_int(), 60); // C note
                    assert_eq!(vel.as_int(), 100);
                }
                _ => panic!("Expected NoteOn"),
            }
            assert_eq!(channel.as_int(), 0);
        }
    }

    #[test]
    fn test_chord_to_midi_events_minor() {
        let events = chord_to_midi_events("Am", 0, 480, 100, 0);
        assert_eq!(events.len(), 6); // 3 NoteOn + 3 NoteOff
        if let TrackEventKind::Midi { message, .. } = events[0].kind {
            match message {
                MidiMessage::NoteOn { key, .. } => {
                    assert_eq!(key.as_int(), 69); // A note
                }
                _ => panic!("Expected NoteOn"),
            }
        }
    }

    #[test]
    fn test_parse_chord_major() {
        let notes = parse_chord("C").unwrap();
        assert_eq!(notes, vec![60, 64, 67]);
    }

    #[test]
    fn test_parse_chord_minor() {
        let notes = parse_chord("Am").unwrap();
        assert_eq!(notes, vec![69, 72, 76]);
    }

    #[test]
    fn test_parse_chord_invalid() {
        let notes = parse_chord("H"); // H is not a valid note
        assert!(notes.is_none());
    }

    #[test]
    fn test_midigen_new() {
        let ast = create_test_ast();
        let midigen = MidiGen::new(&ast);

        assert_eq!(midigen.songs.len(), 1);
        assert_eq!(midigen.sections.len(), 1);
        assert_eq!(midigen.instruments.len(), 1);
        assert_eq!(midigen.patterns.len(), 1);
    }

    #[test]
    fn test_midigen_generate_song() {
        let ast = create_test_ast();
        let mut midigen = MidiGen::new(&ast);

        let result = midigen.generate_song(&"Song1".to_string());
        assert!(result.is_ok());

        let file_name = result.unwrap();
        assert!(file_name.ends_with(".mid"));

        let _ = std::fs::remove_file(file_name);
    }

    #[test]
    fn test_midigen_generate() {
        let ast = create_test_ast();
        let mut midigen = MidiGen::new(&ast);

        let songs = midigen.generate();
        assert_eq!(songs.len(), 1);

        let file_name = &songs[0];
        assert!(file_name.ends_with(".mid"));

        let _ = std::fs::remove_file(file_name);
    }
}

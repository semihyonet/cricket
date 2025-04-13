use core::panic;
use std::{
    collections::{HashMap, HashSet},
    fmt::Result,
};

use crate::ast::{Instrument, Pattern, Section, Song, TopLevel};

pub struct Semantic {
    patterns: HashMap<String, Pattern>,
    sections: HashMap<String, Section>,
    instruments: HashMap<String, Instrument>,
    songs: HashMap<String, Song>,
}

impl Semantic {
    pub fn new(results: Vec<TopLevel>) -> Self {
        let mut patterns = HashMap::new();
        let mut sections = HashMap::new();
        let mut instruments = HashMap::new();
        let mut songs = HashMap::new();

        for res in results {
            match res {
                TopLevel::Song(song) => {
                    if songs.contains_key(&song.name) {
                        panic!("Song with name {:?} defined more then once.", &song.name);
                    }
                    songs.insert(song.name.clone(), song.clone());
                }
                TopLevel::Pattern(pattern) => {
                    if patterns.contains_key(&pattern.name) {
                        panic!(
                            "Pattern with name {:?} defined more then once.",
                            &pattern.name
                        );
                    }
                    patterns.insert(pattern.name.clone(), pattern.clone());
                }
                TopLevel::Section(section) => {
                    if sections.contains_key(&section.name) {
                        panic!(
                            "Section with name {:?} defined more then once.",
                            &section.name
                        );
                    }
                    sections.insert(section.name.clone(), section.clone());
                }
                TopLevel::Instrument(instrument) => {
                    if instruments.contains_key(&instrument.name) {
                        panic!(
                            "Instrument with name {:?} defined more then once.",
                            &instrument.name
                        );
                    }
                    instruments.insert(instrument.name.clone(), instrument.clone());
                }
            }
        }

        Self {
            patterns,
            sections,
            instruments,
            songs,
        }
    }

    pub fn analyze(&mut self) -> Result {
        self.analyze_patterns().unwrap();
        self.analyze_sections().unwrap();
        self.analyze_instruments().unwrap();
        self.analyze_songs().unwrap();

        Result::Ok(())
    }

    fn analyze_patterns(&mut self) -> Result {
        Result::Ok(())
    }

    fn analyze_sections(&mut self) -> Result {
        for section in &self.sections {
            let name = section.0;
            let section = section.1;
            let mut channels: HashSet<String> = HashSet::new();
            for part in &section.channels {
                if channels.contains(&part.name) {
                    panic!(
                        "Song {:?} contains a section named {:?}, that's defined more then once.",
                        name, part.name
                    )
                }
                channels.insert(part.name.clone());
            }
        }
        Result::Ok(())
    }

    fn analyze_instruments(&mut self) -> Result {
        Result::Ok(())
    }

    fn analyze_songs(&mut self) -> Result {
        // Should a file always contain a song? It might be a problem in the future due to multi
        // file imports we can create
        // if self.songs.len() == 0 {
        //     panic!("No songs defined");
        // }
        for song in &self.songs {
            let name = song.0;
            let song = song.1;

            for part in &song.entry_sections {
                if !self.sections.contains_key(part) {
                    panic!(
                        "Song {:?} contains a section named {:?}, that was not defined",
                        name, part
                    )
                }
            }
        }

        Result::Ok(())
    }
}

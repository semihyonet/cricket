#[derive(Debug, Clone)]
pub struct Instrument {
    pub name: String,
    pub type_: String,
    pub midi_path: String,
}

#[derive(Debug, Clone)]
pub enum PatternEvent {
    Note { chord: String, duration: (u8, u8) },
    Wait { duration: (u8, u8) },
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: String,
    pub events: Vec<PatternEvent>,
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub pattern_calls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub channels: Vec<Channel>,
}

#[derive(Debug, Clone)]
pub struct Song {
    pub name: String,
    pub entry_sections: Vec<String>,
}

#[derive(Debug)]
pub enum TopLevel {
    Instrument(Instrument),
    Pattern(Pattern),
    Section(Section),
    Song(Song),
}

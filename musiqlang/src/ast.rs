pub enum Statement {
    InstrumentDef(Instrument),
    PatternDef(Pattern),
    SectionDef(Section),
    SongDef(Song),
}

pub struct Instrument {
    name: String,
    type_: String,
    midi_path: String,
}

pub struct Pattern {
    name: String,
    events: Vec<PatternEvent>,
}

pub enum PatternEvent {
    Note { note: String, length: (u8, u8) },
    Wait { length: (u8, u8) },
}

pub struct Section {
    name: String,
    channels: Vec<Channel>,
}

pub struct Channel {
    name: String,
    calls: Vec<String>,
}

pub struct Song {
    name: String,
    entry_section: String,
}

#[derive(Logos, Debug, PartialEq)]
pub enum Token {
    #[token("Instrument")]
    Instrument,
    #[token("Pattern")]
    Pattern,
    #[token("Section")]
    Section,
    #[token("Song")]
    Song,
    #[token("Channel")]
    Channel,
    #[token("type")]
    Type,
    #[token("midi_path")]
    MidiPath,
    #[token("return")]
    Return,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    #[regex(r"[0-9]+")]
    Number,
    #[regex(r"[A-G][#b]?[m]?")] // Note names like Am, G, A#
    Note,
    #[token(":")]
    Colon,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("+")]
    Plus,
    #[token("=")]
    Equals,
    #[token("\n")]
    Newline,
    #[error]
    #[regex(r"[ \t\r\f]+", logos::skip)]
    Error,
}

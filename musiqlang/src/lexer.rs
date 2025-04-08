use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
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
    //    #[regex(r"[A-G][#b]?[m]?")]
    //    Chord,
    #[regex(r"[0-9]+")]
    Number,
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
    #[token(".")]
    Dot,
    #[token(",")]
    Comma,
    #[token("#")]
    Hash,
    #[regex(r"[ \t\n\r\f]+", logos::skip)]
    #[error]
    Error,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
}

pub fn tokenize(source: &str) -> Vec<(Token, String)> {
    Token::lexer(source)
        .spanned()
        .filter_map(|(tok, span)| {
            if tok != Token::Error {
                Some((tok, source[span].to_string()))
            } else {
                None
            }
        })
        .collect()
}


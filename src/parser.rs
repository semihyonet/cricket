use crate::ast::*;
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<(Token, String)>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<(Token, String)>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos).map(|t| &t.0)
    }

    fn advance(&mut self) -> Option<(Token, String)> {
        let tok = self.tokens.get(self.pos).cloned();
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: Token, location: String) -> String {
        match self.advance() {
            Some((tok, val)) if tok == expected => val,
            Some((tok, _)) => panic!("Expected {:?}, found {:?} at {:?}", expected, tok, location),
            None => panic!("Expected {:?}, found end of input", expected),
        }
    }

    pub fn parse(&mut self) -> Vec<TopLevel> {
        let mut items = vec![];
        while let Some(token) = self.peek() {
            let item = match token {
                Token::Instrument => self.parse_instrument(),
                Token::Pattern => self.parse_pattern(),
                Token::Section => self.parse_section(),
                Token::Song => self.parse_song(),
                _ => panic!("Unexpected token: {:?}", token),
            };
            items.push(item);
        }
        items
    }

    pub fn is_identifier_chord(&mut self, identifier: &String) -> bool {
        if identifier.is_empty() {
            return false;
        }

        if !identifier
            .chars()
            .next()
            .map_or(false, |c| "ABCDEFG".contains(c))
        {
            return false;
        }

        let rest = &identifier[1..]; // Ignore the first character, which is the chord root (A-G)

        let is_valid_chord = rest
            .chars()
            .all(|c| c.is_alphanumeric() || c == '#' || c == 'b' || c == 'm' || c == '7');

        if is_valid_chord {
            return true;
        }

        false
    }

    fn parse_instrument(&mut self) -> TopLevel {
        self.expect(Token::Instrument, stringify!("instrument").to_string());
        let name = self.expect(Token::Identifier, stringify!("instrument").to_string());
        self.expect(Token::Colon, stringify!("instrument").to_string());
        self.expect(Token::Type, stringify!("instrument").to_string());
        self.expect(Token::Colon, stringify!("instrument").to_string());
        let type_ = self.expect(Token::Identifier, stringify!("instrument").to_string());
        self.expect(Token::MidiPath, stringify!("instrument").to_string());
        self.expect(Token::Colon, stringify!("instrument").to_string());
        let midi_path = self.expect(Token::Identifier, stringify!("instrument").to_string());
        TopLevel::Instrument(Instrument {
            name,
            type_,
            midi_path,
        })
    }

    fn parse_pattern(&mut self) -> TopLevel {
        self.expect(Token::Pattern, stringify!("Pattern").to_string());
        let name = self.expect(Token::Identifier, stringify!("Pattern").to_string());
        self.expect(Token::LParen, stringify!("Pattern").to_string());
        self.expect(Token::RParen, stringify!("Pattern").to_string());
        self.expect(Token::Colon, stringify!("Pattern").to_string());
        self.expect(Token::Return, stringify!("Pattern").to_string());
        let mut events = vec![];
        while let Some(Token::LBracket) = self.peek() {
            self.expect(Token::LBracket, stringify!("Pattern").to_string());
            let num = self
                .expect(Token::Number, stringify!("Pattern").to_string())
                .parse::<u8>()
                .unwrap();
            self.expect(Token::Colon, stringify!("Pattern").to_string());
            let denom = self
                .expect(Token::Number, stringify!("Pattern").to_string())
                .parse::<u8>()
                .unwrap();
            self.expect(Token::RBracket, stringify!("Pattern").to_string());

            match self.advance() {
                Some((Token::Identifier, ident)) if ident == "Wait" => {
                    self.expect(Token::LParen, stringify!("Pattern").to_string());
                    self.expect(Token::RParen, stringify!("Pattern").to_string());
                    events.push(PatternEvent::Wait {
                        duration: (num, denom),
                    });
                }
                Some((Token::Identifier, ident)) if ident == "Note" => {
                    self.expect(Token::LParen, stringify!("Pattern").to_string());
                    match self.advance() {
                        Some((Token::Identifier, chord)) if self.is_identifier_chord(&chord) => {
                            events.push(PatternEvent::Note {
                                chord,
                                duration: (num, denom),
                            });
                            self.expect(Token::RParen, stringify!("Pattern").to_string());
                        }
                        other => panic!("Note is not accepted as {:?}", other),
                    }
                }
                other => panic!("Unexpected token in pattern event: {:?}", other),
            }

            if let Some(Token::Plus) = self.peek() {
                self.advance();
            }
        }

        TopLevel::Pattern(Pattern { name, events })
    }

    fn parse_section(&mut self) -> TopLevel {
        self.expect(Token::Section, stringify!("Section").to_string());
        let name = self.expect(Token::Identifier, stringify!("Section").to_string());
        self.expect(Token::Colon, stringify!("Section").to_string());

        let mut channels = vec![];

        while let Some(Token::Channel) = self.peek() {
            self.expect(Token::Channel, stringify!("Section-channel").to_string());
            let chan_name =
                self.expect(Token::Identifier, stringify!("Section-channel").to_string());
            self.expect(Token::Colon, stringify!("Section-channel").to_string());

            let mut calls = vec![];

            self.expect(Token::Return, stringify!("Section-channel").to_string());

            calls.push(self.expect(Token::Identifier, stringify!("Section-channel").to_string()));
            self.expect(Token::LParen, stringify!("Section-channel").to_string());
            // We might have some parameters here
            self.expect(Token::RParen, stringify!("Section-channel").to_string());

            while let Some(Token::Plus) = self.peek() {
                self.expect(Token::Plus, stringify!("Section-channel").to_string());
                calls.push(
                    self.expect(Token::Identifier, stringify!("Section-channel").to_string()),
                );
                self.expect(Token::LParen, stringify!("Section-channel").to_string());
                // We might have some parameters here
                self.expect(Token::RParen, stringify!("Section-channel").to_string());
            }

            channels.push(Channel {
                name: chan_name,
                pattern_calls: calls,
            });
        }

        TopLevel::Section(Section { name, channels })
    }

    fn parse_song(&mut self) -> TopLevel {
        self.expect(Token::Song, stringify!("song").to_string());
        let name = self.expect(Token::Identifier, stringify!("song").to_string());
        self.expect(Token::Colon, stringify!("song").to_string());
        self.expect(Token::Return, stringify!("song").to_string());

        let mut sections = vec![];
        sections.push(self.expect(Token::Identifier, stringify!("song").to_string()));
        self.expect(Token::LParen, stringify!("song").to_string());
        // We might have some parameters here
        self.expect(Token::RParen, stringify!("song").to_string());
        while let Some(Token::Plus) = self.peek() {
            self.expect(Token::Plus, stringify!("song").to_string());
            sections.push(self.expect(Token::Identifier, stringify!("song").to_string()));
            self.expect(Token::LParen, stringify!("song").to_string());
            // We might have some parameters here
            self.expect(Token::RParen, stringify!("song").to_string());
        }

        TopLevel::Song(Song {
            name,
            entry_sections: sections,
        })
    }
}

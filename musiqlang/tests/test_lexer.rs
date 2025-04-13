#[cfg(test)]
mod tests {
    use musiqlang::lexer::{Token, tokenize};

    use super::*;

    #[test]
    fn test_basic_tokenization() {
        let input = r#"
            Instrument guitar:
                type: Strings
                midi_path: guitar.mid
        "#;

        let tokens = tokenize(input);

        assert_eq!(tokens[0].0, Token::Instrument);
        assert_eq!(tokens[1].0, Token::Identifier); // guitar
        assert_eq!(tokens[2].0, Token::Colon);
        assert_eq!(tokens[3].0, Token::Type);
        assert_eq!(tokens[4].0, Token::Colon);
        assert_eq!(tokens[5].0, Token::Identifier); // Strings
        assert_eq!(tokens[6].0, Token::MidiPath);
        assert_eq!(tokens[7].0, Token::Colon);
        assert_eq!(tokens[8].0, Token::Identifier); // guitar.mid
    }

    #[test]
    fn test_chord_token() {
        let input = "Note(Am)";
        let tokens = tokenize(input);
        let values: Vec<_> = tokens.iter().map(|(_, v)| v.as_str()).collect();

        assert!(values.contains(&"Am"));
    }
}

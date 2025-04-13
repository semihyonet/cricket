// tests/integration.rs

use musiqlang::{lexer, parser::Parser};

#[test]
fn test_full_song_flow() {
    let input = r#"
    Instrument instrument_xyz:
	type: Strings
	midi_path: xyz

    Pattern intro(): 
	return [1:8] Note(Am)

    Section Intro:
	Channel name_a:
		return intro()

    Song HotlineBling: 
	return Intro() + Intro()
    "#;

    let tokens = lexer::tokenize(&input);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert_eq!(ast.len(), 4);
}

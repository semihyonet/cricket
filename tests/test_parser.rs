#[cfg(test)]
mod tests {
    use cricket::{
        ast::TopLevel,
        lexer::{self, Token},
        parser::Parser,
    };

    #[test]
    fn test_parse_instrument() {
        let input = r#"
            Instrument piano:
                type: Strings
                midi_path: pianoxmid
        "#;

        let tokens = lexer::tokenize(input);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            TopLevel::Instrument(instr) => {
                assert_eq!(instr.name, "piano");
                assert_eq!(instr.midi_path, "pianoxmid");
            }
            _ => panic!("Expected instrument node"),
        }
    }

    #[test]
    fn test_parse_pattern() {
        let input = r#"
            Pattern intro():
                return [1:8] Note(C)
        "#;

        let tokens = lexer::tokenize(input);
        let mut parser = Parser::new(tokens);
        let ast = parser.parse();

        assert_eq!(ast.len(), 1);
        match &ast[0] {
            TopLevel::Pattern(pat) => {
                assert_eq!(pat.name, "intro");
                // Optionally check contents of return_expr
            }
            _ => panic!("Expected pattern node"),
        }
    }
}

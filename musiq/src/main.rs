use musiqlang::lexer;
use musiqlang::parser::Parser;
use musiqlang::semantic::Semantic;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    let tokens = lexer::tokenize(&content);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);

    let mut semantic_analysis = Semantic::new(ast);
    let _ = semantic_analysis.analyze();

    println!("Worked")
}

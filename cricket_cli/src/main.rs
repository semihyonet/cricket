use cricket::lexer;
use cricket::midigen::MidiGen;
use cricket::parser::Parser;
use cricket::semantic::Semantic;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = &args[1];
    let content = std::fs::read_to_string(path).expect("Failed to read file");

    let tokens = lexer::tokenize(&content);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    println!("{:#?}", ast);

    let mut semantic_analysis = Semantic::new(ast.clone());
    let _ = semantic_analysis.analyze();

    println!("Worked");

    let mut midigen = MidiGen::new(&ast);
    let results = midigen.generate();
}

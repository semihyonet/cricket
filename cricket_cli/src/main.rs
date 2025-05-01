use cricket::lexer;
use cricket::midigen::MidiGen;
use cricket::parser::Parser;
use cricket::semantic::Semantic;
use cricket::soundgen::render_midi_to_wav;
use env_logger::Builder;
use log::LevelFilter;
use log::{debug, info};

use clap::Parser as clap_Parser;

use clap::ValueEnum;
use std::process;

#[derive(clap_Parser, Debug)]
#[command(name = "cricket", version, about = "Compile and render Cricket music files", long_about = None)]
struct Cli {
    /// Path to the Cricket source file (e.g. foo.cricket)
    file_path: String,

    /// Output format: generate 'midi' or 'sound' (wav)
    #[arg(short = 'g', long = "generate", value_enum, default_value_t = OutputType::Sound)]
    generate: OutputType,

    #[arg(long = "sf-path")]
    sf_path: Option<String>,

    #[arg(short = 'v', long = "verbose", action)]
    verbose: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputType {
    Midi,
    Sound,
}

fn init_logging(verbose: bool) {
    let mut builder = Builder::new();

    // If verbose, allow debug; otherwise info+
    let min_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    // Apply that filter to all modules
    builder
        .filter(None, min_level)
        // Optionally still allow RUST_LOG to override
        .parse_env("RUST_LOG")
        .init();
}
fn main() {
    let cli = Cli::parse();
    let src = &cli.file_path;

    init_logging(cli.verbose);

    let content = std::fs::read_to_string(src).unwrap_or_else(|e| {
        eprintln!("Error reading '{}': {}", src, e);
        process::exit(1);
    });

    debug!("CLI arguments: {:?}", cli);

    debug!("Reading source file: {}", cli.file_path);
    let tokens = lexer::tokenize(&content);
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    debug!("{:#?}", ast);

    let mut semantic_analysis = Semantic::new(ast.clone());
    let _ = semantic_analysis.analyze();

    debug!("Worked");

    let created_words = match cli.generate {
        OutputType::Midi => {
            let mut midigen = MidiGen::new(&ast);
            let results = midigen.generate();
            results
        }
        OutputType::Sound => {
            let mut midigen = MidiGen::new(&ast);
            let results = midigen.generate();
            let sf2 = cli.sf_path.as_deref().unwrap_or_else(||  {
                eprintln!("No SoundFont Path has been passed while trying to generate a Sound. Please use the --sf-path argument to pass a path to the soundfont.");
                process::exit(1);
            });
            let mut wav_paths = Vec::new();
            for result in results.iter() {
                let wav_path = render_midi_to_wav(result, sf2).unwrap();
                wav_paths.push(wav_path);
            }
            wav_paths
        }
    };
    println!("Created the following files");
    for res in created_words {
        println!("{}", res);
    }
}

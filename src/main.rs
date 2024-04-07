use clap::{Parser, Subcommand};
use interpreter::Interpreter;
use parser::ast;
use std::{fs::File, io::Read, path::PathBuf};

mod interpreter;
mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Run in interpreter mode.
    Interpret {
        /// File to feed to interpreter
        file: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interpret { file }) => {
            let mut file = File::open(file.as_path()).unwrap();
            let mut code = String::new();
            file.read_to_string(&mut code).ok();
            let ast = ast(&code);
            let interpreter = Interpreter::new(ast);
            interpreter.run();
        }
        None => {}
    }
}

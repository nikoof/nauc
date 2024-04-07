use anyhow::Result;
use clap::{Parser, Subcommand};
use interpreter::{Interpreter, InterpreterProps};
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

        /// Disable the wrapping of cell values. If on, IntegerOverflow errors are possible.
        #[arg(short = 'w', long)]
        no_wrap: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interpret { file, no_wrap }) => {
            let mut file = File::open(file.as_path()).unwrap();
            let mut code = String::new();
            file.read_to_string(&mut code).ok();
            let ast = ast(&code)?;

            let props = InterpreterProps { wrapping: !no_wrap };
            let interpreter = Interpreter::new(ast);
            interpreter.run(props)?;
        }
        None => {}
    }

    Ok(())
}

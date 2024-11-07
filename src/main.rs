use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{Cli, Commands};
use compiler::{codegen, compile};
use interpreter::InterpreterBuilder;
use parser::ast;

mod cli;
mod compiler;
mod interpreter;
mod parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Interpret {
            file,
            no_wrap,
            memory,
        }) => {
            let code = std::fs::read_to_string(file.as_path())?;
            let ast = ast(code)?;

            let interpreter = InterpreterBuilder::new()
                .program(ast)
                .wrapping(!no_wrap)
                .memory(memory)
                .build();

            interpreter.run()?;
        }
        Some(Commands::Compile {
            file,
            memory,
            output,
            keep_artifacts,
            debug,
        }) => {
            let code = std::fs::read_to_string(file.as_path())?;
            let ast = ast(code)?;

            let output = output.unwrap_or(
                file.file_name()
                    .ok_or(anyhow!("Output file should be a file"))?
                    .into(),
            );
            compile(
                &codegen(&ast, memory),
                &output,
                debug || keep_artifacts,
                debug,
            )?;
        }
        None => {}
    }

    Ok(())
}

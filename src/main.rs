use anyhow::{anyhow, Result};
use clap::Parser;

use cli::{Cli, Command};
use compiler::{
    arch::{self, x86_64_linux::codegen, Architecture},
    compile,
};
use interpreter::InterpreterBuilder;
use parser::ast;

mod cli;
mod compiler;
mod interpreter;
mod parser;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Command::Interpret {
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
        Some(Command::Compile {
            file,
            memory,
            output,
            keep_artifacts,
            debug,
            architecture,
        }) => {
            let code = std::fs::read_to_string(file.as_path())?;
            let ast = ast(code)?;

            let output = output.unwrap_or(
                file.file_name()
                    .ok_or(anyhow!("Output file should be a file"))?
                    .into(),
            );

            let asm = match architecture {
                Architecture::Aarch64Linux => arch::aarch64_linux::codegen,
                Architecture::X86_64Linux => arch::x86_64_linux::codegen,
            }(&ast, memory);

            compile(&asm, &output, debug || keep_artifacts, debug)?;
        }
        None => {}
    }

    Ok(())
}

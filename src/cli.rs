use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run in interpreter mode.
    Interpret {
        /// File to feed to interpreter.
        file: PathBuf,

        /// Disable the wrapping of cell values. If on, IntegerOverflow errors are possible.
        #[arg(short = 'w', long)]
        no_wrap: bool,

        /// Number of cells in memory
        #[arg(short, long, default_value = "30000")]
        memory: usize,
    },

    /// Run in compiler mode.
    Compile {
        /// Source file.
        file: PathBuf,

        /// Number of cells in memory.
        #[arg(short, long, default_value = "30000")]
        memory: usize,

        /// Output file.
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Keep build artifacts.
        #[arg(short, long, default_value = "false")]
        keep_artifacts: bool,

        /// Include debug symbols (implies --keep-artifacts)
        #[arg(short = 'g', long, default_value = "false")]
        debug: bool,
    },
}

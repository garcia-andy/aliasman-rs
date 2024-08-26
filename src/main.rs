//! The Main process
use aliasman::cli::{commands::Cli, program::Program};
use clap::Parser;

fn main() {
    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
}

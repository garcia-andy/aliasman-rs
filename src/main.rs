use aliasman::cli::{Cli, Program};
use clap::Parser;

fn main() {

    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
}

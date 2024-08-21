use aliasman::{alias::Alias, cli::{Cli, Commands, Program}, setup_aliasman, Printer};
use clap::{Parser, Subcommand};


fn main() {
    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
    
}

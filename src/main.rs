use aliasman::cli::{Cli, Program};
use clap::Parser;

fn main() {

    //println!("Detected Shell: {}", get_shell());
    //println!("Config File: {}", get_shell_config_file());
    //println!("Alias File: {}", get_shell_aliases());

    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
    
}

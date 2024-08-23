use aliasman::{cli::{Cli, Program}, shell_utils::{get_shell, get_shell_aliases, get_shell_config_file}};
use clap::Parser;

fn main() {
    println!("Detected Shell: {}", get_shell());
    println!("Config File: {}", get_shell_config_file());
    println!("Alias File: {}", get_shell_aliases());

    let cli = Cli::parse();
    let mut prog = Program::new();

    prog.resolve(&cli);
    
}

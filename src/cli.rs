use clap::{Parser, Subcommand};

use crate::{alias::{Alias, AliasMan}, setup_aliasman, Printer};



/// Gesture of Aliases
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    // Commands
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Adds alias
    Add { name: String, cmd: Vec<String> },
    /// Replace an already exist alias
    Replace { name: String, cmd: Vec<String> },
    /// Remove an alias
    Remove { name: String },
    /// Remove an alias
    Rm { name: String },
    /// List all Aliases
    List
}

pub struct Program {
    _aliasman: AliasMan,
    _prt: Printer
}


impl Program{
    pub fn new() -> Self {
        let _aman = setup_aliasman().unwrap();
        let printer = Printer::new();

        Self { _aliasman: _aman, _prt: printer }
    }

    pub fn add(&mut self, name: &String, cmd: &Vec<String>) {
        self._aliasman.add( 
            Alias(
                name.clone(), 
                cmd.join(" ")
            ) 
        ).expect("Alias already taken!");

        self._prt.writeln(
            format!(
                "Adding alias {} -> '{}'", 
                name, 
                cmd.join(" ")
            ).as_str()
        ).unwrap();
    }

    pub fn rm(&mut self, name: &String) {
        self._aliasman.rm( name.as_str())
            .expect("Alias not exist!");

        self._prt.writeln(
            format!(
                "Removed alias {}", 
                name
            ).as_str()
        ).unwrap();
    }

    pub fn replace(&mut self, name: &String, cmd: &Vec<String>) {
        self.rm(name);
        self.add(name, cmd);
    }

    pub fn list(&mut self){
        for a in self._aliasman.list(){
            self._prt.writeln(
                format!(
                    "Alias {} = '{}'",
                    a.0,a.1
                ).as_str()
            ).unwrap();
        }
    }

    pub fn resolve(&mut self, cli: &Cli) {
        match &cli.command {
            Commands::Add { name, cmd } => {
                self.add(name, cmd)
            }
            Commands::Replace { name, cmd } => {
                self.replace(name, cmd)
            }
            Commands::Remove { name } => {
                self.rm(name)
            },
            Commands::Rm { name } => {
                self.rm(name)
            },
            Commands::List => {
                self.list()
            },
        }

        self._aliasman.flush_changes().expect("Unable to save changes!");
    }

}
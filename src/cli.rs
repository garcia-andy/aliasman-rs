use clap::{Parser, Subcommand};
use crate::{alias::{Alias, AliasMan}, setup_aliasman, shell_utils::{get_shell, get_shell_aliases, get_shell_config_file}, Printer};


/// Gesture of Aliases
#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Commands
    #[command(subcommand)]
    pub command: Commands,
}

/// Enum with all avaiables commands
#[derive(Subcommand)]
pub enum Commands {
    /// Adds alias
    Add { 
        /// The name of the Alias to create
        name: String, 
        /// A list to create the string (using '.join(" ")') of execute
        cmd: Vec<String> 
    },
    /// Replace an already exist alias
    Replace { 
        /// The name of the Alias to replace
        name: String, 
        /// A list to create the string (using '.join(" ")') of execute
        cmd: Vec<String> 
    },
    /// Remove an alias
    Remove { 
        /// The name of the Alias to remove
        name: String 
    },
    /// Remove an alias
    Rm { 
        /// The name of the Alias to remove
        name: String 
    },
    /// List all Aliases
    List
}

/// Struct representing
/// the alias management and the command resolve
pub struct Program {
    aliasman: AliasMan,
    prt: Printer
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

fn fprint(p: &mut Printer, content: String){
    p.writeln(content.as_str()).expect("Error writing information");
}

impl Program{
    /// Create a new program instance
    /// # Panics
    /// On creation of the AliasMan maybe panic
    pub fn new() -> Self {
        let aman = setup_aliasman().expect("Error on setup");
        let mut printer = Printer::new();

        fprint(
            &mut printer, 
            format!("Detected Shell: {}", get_shell())
        );

        fprint(
            &mut printer, 
            format!("Config File: {}", get_shell_config_file())
        );

        fprint(
            &mut printer, 
            format!("Alias File: {}", get_shell_aliases())
        );


        Self { aliasman: aman, prt: printer }
    }

    /// Interface to add an alias
    /// # Panics
    /// Panic if the alias already exist!
    pub fn add(&mut self, name: &String, cmd: &[String]) {
        self.aliasman.add( 
            Alias(
                name.clone(), 
                cmd.join(" ")
            ) 
        ).expect("Alias already taken!");

        self.prt.writeln(
            format!(
                "Adding alias {} -> '{}'", 
                name, 
                cmd.join(" ")
            ).as_str()
        ).expect("Error on write output!");
    }

    /// Interface to remove an alias
    /// # Panics
    /// Panic if the alias not exist!
    pub fn rm(&mut self, name: &String) {
        self.aliasman.rm( name.as_str())
            .expect("Alias not exist!");

        self.prt.writeln(
            format!(
                "Removed alias {name}"
            ).as_str()
        ).expect("Error on write output!");
    }

    /// Interface to replace an alias
    /// # Panics
    /// Panic if the alias not exist!
    pub fn replace(&mut self, name: &String, cmd: &[String]) {
        self.rm(name);
        self.add(name, cmd);
    }

    /// Interface to list all alias
    /// # Panics
    /// Panic if the writeln fail
    pub fn list(&mut self){
        for a in self.aliasman.list(){
            self.prt.writeln(
                format!(
                    "Alias {} = '{}'",
                    a.0,a.1
                ).as_str()
            ).expect("Error on write output!");
        }
    }

    /// Run the command specified for the CLI 
    /// and save changes
    /// # Panics
    /// Panic on flush_changes
    pub fn resolve(&mut self, cli: &Cli) {
        match &cli.command {
            Commands::Add { name, cmd } => {
                self.add(name, cmd);
            }
            Commands::Replace { name, cmd } => {
                self.replace(name, cmd);
            }
            Commands::Remove { name } 
            | Commands::Rm { name }
            => {
                self.rm(name);
            },
            Commands::List => {
                self.list();
            },
        }

        self.aliasman.flush_changes().expect("Unable to save changes!");
    }

}
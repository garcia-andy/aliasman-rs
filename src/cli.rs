use crate::{
    alias::{Alias, AliasMan},
    setup_aliasman, updateable,
    upgrade::upgrade_release,
    Printer,
};
use clap::{Parser, Subcommand};

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
        cmd: Vec<String>,
    },
    /// Replace an already exist alias
    Replace {
        /// The name of the Alias to replace
        name: String,
        /// A list to create the string (using '.join(" ")') of execute
        cmd: Vec<String>,
    },
    /// Edit an already exist alias
    Edit {
        /// The name of the Alias to Edit
        name: String,
        /// A list to create the string (using '.join(" ")') of execute
        cmd: Vec<String>,
    },
    /// Remove an alias
    Remove {
        /// The name of the Alias to remove
        name: String,
    },
    /// Remove an alias
    Rm {
        /// The name of the Alias to remove
        name: String,
    },
    /// List all Aliases
    List,
    /// Force update the local shells information
    Update,
    /// Try to upgrade the binary
    Upgrade,
}

/// Struct representing
/// the alias management and the command resolve
pub struct Program {
    aliasman: AliasMan,
    prt: Printer,
}

impl Default for Program {
    fn default() -> Self {
        Self::new()
    }
}

impl Program {
    /// Create a new program instance
    /// # Panics
    /// On creation of the `AliasMan` maybe panic
    pub fn new() -> Self {
        let aman = setup_aliasman().expect("Error on setup");
        let printer = Printer::new();

        Self {
            aliasman: aman,
            prt: printer,
        }
    }

    /// Interface to add an alias
    /// # Panics
    /// Panic if the alias already exist!
    pub fn add(&mut self, name: &String, cmd: &[String]) {
        self.aliasman
            .add(Alias(name.clone(), cmd.join(" ")))
            .expect("Alias already taken!");

        self.prt
            .writeln(format!("Adding alias {} -> '{}'", name, cmd.join(" ")).as_str())
            .expect("Error on write output!");
    }

    /// Interface to remove an alias
    /// # Panics
    /// Panic if the alias not exist!
    pub fn rm(&mut self, name: &String) {
        self.aliasman.rm(name.as_str()).expect("Alias not exist!");

        self.prt
            .writeln(format!("Removed alias {name}").as_str())
            .expect("Error on write output!");
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
    pub fn list(&mut self) {
        for a in self.aliasman.list() {
            self.prt
                .writeln(format!("Alias {} = '{}'", a.0, a.1).as_str())
                .expect("Error on write output!");
        }
    }

    /// Update the local configuration from github
    /// # Panics
    /// Panic if the writeln fail
    pub fn update_config(&mut self) {
        let cfg = updateable::update();
        self.prt
            .writeln(format!("New configuration loaded: \n{cfg}").as_str())
            .expect("Error printing configuration loaded");
    }

    /// Update the local config and upgrade the binary
    /// # Panics
    /// Panic if the writeln fail
    pub fn upgrade_bin(&mut self) {
        self.update_config();
        self.prt
            .writeln(upgrade_release().as_str())
            .expect("Error printing configuration loaded");
    }

    /// Run the command specified for the CLI
    /// and save changes
    /// # Panics
    /// Panic on `flush_changes`
    pub fn resolve(&mut self, cli: &Cli) {
        match &cli.command {
            Commands::Add { name, cmd } => {
                self.add(name, cmd);
            }
            Commands::Replace { name, cmd } | Commands::Edit { name, cmd } => {
                self.replace(name, cmd);
            }
            Commands::Remove { name } | Commands::Rm { name } => {
                self.rm(name);
            }
            Commands::List => {
                self.list();
            }
            Commands::Update => {
                self.update_config();
            }
            Commands::Upgrade => {
                self.upgrade_bin();
            }
        }

        self.aliasman
            .flush_changes()
            .expect("Unable to save changes!");
    }
}

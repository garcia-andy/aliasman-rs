use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::str::FromStr;
use strum::IntoEnumIterator;

use super::commands::*;
use crate::{
    alias::{Alias, AliasMan},
    modules::{updateable, upgrade::upgrade_release},
    setup_aliasman,
    utils::print::*,
};

/// Struct representing
/// the alias management and the command resolve
pub struct Program {
    aliasman: AliasMan,
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

        Self { aliasman: aman }
    }

    /// Process the commands options to functions
    fn _process(&mut self, command: &Commands) {
        match &command {
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
            Commands::Prompt => {
                self.prompt_mode();
            }
        }
    }

    /// Request the command name
    fn req_alias() -> String {
        Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Alias name")
            .interact_text()
            .expect("Invalid input")
    }

    /// Request the command
    fn req_cmd() -> Vec<String> {
        let content: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt("Command")
            .interact_text()
            .expect("Invalid input");

        content.split(' ').map(String::from).collect()
    }

    /// Interface to add an alias
    /// # Panics
    /// Panic if the alias already exist!
    pub fn add(&mut self, name: &String, cmd: &[String]) {
        self.aliasman
            .add(Alias(name.clone(), cmd.join(" ")))
            .expect("Alias already taken!");
        info_ln(format!("Adding alias {} -> '{}'", name, cmd.join(" ")).as_str());
    }

    /// Interface to remove an alias
    /// # Panics
    /// Panic if the alias not exist!
    pub fn rm(&mut self, name: &String) {
        self.aliasman.rm(name.as_str()).expect("Alias not exist!");

        info_ln(format!("Removed alias {name}").as_str());
    }

    /// Interface to replace an alias
    /// # Panics
    /// Panic if the alias not exist!
    pub fn replace(&mut self, name: &String, cmd: &[String]) {
        self.rm(name);
        self.add(name, cmd);
    }

    /// Interface to list all alias
    pub fn list(&self) {
        for a in self.aliasman.list() {
            let al = a.0.as_str();
            let cmd = a.1.as_str();

            info_ln(format!("{al} = '{cmd}'").as_str());
        }
    }

    /// Update the local configuration from github
    pub fn update_config(&self) {
        let cfg = updateable::update();
        debug_ln(format!("New configuration loaded: \n{cfg}").as_str());
    }

    /// Update the local config and upgrade the binary
    pub fn upgrade_bin(&self) {
        self.update_config();
        info_ln(upgrade_release().as_str());
    }

    /// Call for loop in prompt mode
    /// # Panics
    /// The selector maybe panic
    pub fn prompt_mode(&mut self) {
        let items: Vec<&str> = CommandsTypes::iter()
            .map(|c| {
                if c == CommandsTypes::Prompt {
                    return "Quit";
                }
                c.into()
            })
            .collect();

        loop {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Comando:")
                .default(0)
                .items(&items)
                .interact()
                .expect("Error on selection");

            if let Some(command_str) = items.get(selection) {
                if command_str.contains("Quit") {
                    break;
                }

                let cmd = CommandsTypes::from_str(command_str).expect("Invalid command");

                let command = match cmd {
                    CommandsTypes::Add => {
                        let name = Self::req_alias();
                        let cmd = Self::req_cmd();

                        Commands::Add { name, cmd }
                    }
                    CommandsTypes::Replace | CommandsTypes::Edit => {
                        let name = Self::req_alias();
                        let cmd = Self::req_cmd();

                        Commands::Replace { name, cmd }
                    }
                    CommandsTypes::Remove | CommandsTypes::Rm => {
                        let name = Self::req_alias();

                        Commands::Remove { name }
                    }
                    CommandsTypes::List => Commands::List,
                    CommandsTypes::Update => Commands::Update,
                    CommandsTypes::Upgrade => Commands::Upgrade,
                    CommandsTypes::Prompt => Commands::Prompt,
                };

                self._process(&command);
            } else {
                break;
            }
        }
    }

    /// Run the command specified for the CLI
    /// and save changes
    /// # Panics
    /// Panic on `flush_changes`
    pub fn resolve(&mut self, cli: &Cli) {
        self._process(&cli.command);

        let error = self.aliasman.flush_changes();

        if error.is_err() {
            err("Unable to save changes!");
        }
    }
}

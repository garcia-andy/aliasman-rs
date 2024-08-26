use clap::{Parser, Subcommand};
use strum::{EnumDiscriminants, EnumIter, EnumString, IntoStaticStr};

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
#[derive(Subcommand, EnumDiscriminants, EnumString)]
#[strum_discriminants(derive(EnumIter, IntoStaticStr, EnumString))]
#[strum_discriminants(name(CommandsTypes))]
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
    /// Use the prompt mode for various operations
    Prompt,
}

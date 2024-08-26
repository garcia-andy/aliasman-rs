//! The main library
/// Crate with the alias Management
pub mod alias;
/// Crate with CLI options and features
pub mod cli;
/// Crate with all functionalities and features
pub mod modules;
/// Crate with all utilities functions
pub mod utils;

use alias::AliasMan;

use anyhow::{Ok, Result};
use std::{fs::read_to_string, io::Write, path::Path};
use utils::file_utils::*;
use utils::shell_utils::{get_info, get_shell};

/// Review the configuration for the shell and setting up if is nedded
/// # Errors
/// First error on `read_to_string`
pub fn setup_aliasman() -> Result<AliasMan> {
    let [cfg, alias] = get_info(get_shell().as_str());

    if Path::new(cfg.as_str()).exists() {
        let bash_content = read_to_string(cfg.as_str())?;

        if !bash_content.contains(alias.as_str()) {
            let import_content = if cfg.contains("fish") {
                "\nsource "
            } else {
                "\n. "
            };

            let mut bash = mod_file(cfg.as_str())?;
            bash.write_all(import_content.as_bytes())?;
            bash.write_all(alias.as_bytes())?;
        }
    }
    Ok(AliasMan::new(alias.as_str()))
}

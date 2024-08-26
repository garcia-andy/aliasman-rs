use curl::easy::Easy;
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::{fs::read_to_string, io::Write, path::Path};

use crate::{truncate_file, utils::shell_utils::home};

/// Information recived from curl of shells
#[derive(Debug, Serialize, Deserialize)]
pub struct ShellInformation {
    /// The name of the shell
    pub name: String,
    /// the name or path from HOME to configuration file
    pub config: String,
    /// the name or path from HOME to alias file
    pub alias: String,
}

/// Information recived from curl
#[derive(Debug, Serialize, Deserialize)]
pub struct ProgramInfo {
    /// Version stable
    pub stable: String,
    /// Shells configurations
    pub shells: Vec<ShellInformation>,
}

const REPO: &str = "https://raw.githubusercontent.com/garcia-andy/aliasman-rs/main/shells.json";
const CFG: &str = "/.aliasman.json";

/// Load from the github repo
fn load_from_git() -> String {
    let mut sp = Spinner::new(
        Spinners::Dots9,
        "Downloading the configuration content".into(),
    );

    let mut content = String::new();
    let mut easy = Easy::new();
    easy.url(REPO).expect("Error connecting to github");

    {
        let mut transfer = easy.transfer();
        transfer
            .write_function(|data| {
                content.push_str(
                    String::from_utf8(Vec::from(data))
                        .expect("Error parsing data")
                        .as_str(),
                );
                Ok(data.len())
            })
            .expect("Error setting up the write function");
        transfer
            .perform()
            .expect("Error in the Transfer perform action");
    }

    sp.stop_with_message("✅ Already download the configuration content".to_string());
    content
}

/// Load content from the github repo or local json
/// # Errors
/// Error connecting to github or reading the conf file
/// # Panics
/// Panic on any error
pub fn load_content() -> ProgramInfo {
    let config_file = home() + CFG;

    let content = if Path::new(config_file.as_str()).exists() {
        read_to_string(config_file).expect("Error reading config file")
    } else {
        update()
    };

    // Parse the string of data into serde_json::Value.
    serde_json::from_str(content.as_str()).expect("Error parsing data")
}

/// Update the local configuration
/// # Errors
/// Maybe file on write the new configuration
/// # Panics
/// If fail on git connection or writing conf
pub fn update() -> String {
    let config_file = home() + CFG;
    let content = load_from_git();

    let mut sp = Spinner::new(Spinners::Dots9, "Writing the configuration content".into());
    let mut config_file = truncate_file(config_file.as_str()).expect("Unable to open config file");

    config_file
        .write_all(content.as_bytes())
        .expect("Error to write content");

    config_file.flush().expect("Error flushing datas");
    sp.stop_with_message("✅ Already writed the configuration content".to_string());

    content
}

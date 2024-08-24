use crate::create_file;
use crate::proc::ProcessMan;
use crate::updateable::{load_content, ShellInformation};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Struct representing the files configurations of an specific shell
pub struct ShellConfig {
    /// The file from save the aliases
    pub alias_file: String,
    /// The file to write the configurations
    pub config_file: String,
}

// const DEFAULT_ALIAS_FILE: &str = ".aliasman";
// static SHELLS_REGISTER_CONFIGS: LazyLock<Vec<[&str; 3]>> = LazyLock::new(|| {
//     vec![
//         ["bash", ".bashrc", DEFAULT_ALIAS_FILE],
//         ["zsh", ".zshrc", DEFAULT_ALIAS_FILE],
//         [
//             "fish",
//             ".config/fish/conf.fish",
//             ".config/fish/conf.d/aliases.fish",
//         ],
//     ]
// });

static SHELLS_REMOTE: LazyLock<Vec<ShellInformation>> = LazyLock::new(load_content);

static SHELLS_INFO: LazyLock<HashMap<String, &ShellInformation>> = LazyLock::new(|| {
    let mut m: HashMap<String, &ShellInformation> = HashMap::new();

    let arr = &*SHELLS_REMOTE;

    for info in arr {
        m.insert((*info.name).to_string(), info);
    }

    m
});

/// Get the name of the shell from the process manager
pub fn get_shell() -> String {
    // getting the process manager
    let pm = ProcessMan::new();
    // getting the parent name from the Process Manager
    pm.get_parent_name()
}

/// Get the HOME env var
/// # Panics
/// Panic on getting env var
pub fn home() -> String {
    std::env::var("HOME").expect("We required the $HOME env variable")
}

/// Getting the name of the config & alias file
/// # Panics
/// Panic on getting env var
pub fn get_info(name: String) -> [String; 2] {
    let homedir = home();

    let cfg = (*SHELLS_INFO)
        .get(name.as_str())
        .expect("Shell configuration file not found!");

    let cfg_file = format!("{homedir}/{}", cfg.config);
    create_file(cfg_file.as_str()).expect("Unable to create/found config file for terminal");

    let alias_file = format!("{homedir}/{}", cfg.alias);
    create_file(alias_file.as_str()).expect("Unable to create/found config file for terminal");

    [cfg_file, alias_file]
}

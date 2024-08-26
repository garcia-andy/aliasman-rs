use crate::{
    create_file,
    modules::updateable::{load_content, ProgramInfo, ShellInformation},
    utils::proc::ProcessMan,
};
use std::collections::HashMap;
use std::sync::LazyLock;

/// Struct representing the files configurations of an specific shell
pub struct ShellConfig {
    /// The file from save the aliases
    pub alias_file: String,
    /// The file to write the configurations
    pub config_file: String,
}

/// Information getting from the global configuration
pub static SHELLS_REMOTE: LazyLock<ProgramInfo> = LazyLock::new(load_content);

/// Parsed shell informations
pub static SHELLS_INFO: LazyLock<HashMap<String, &ShellInformation>> = LazyLock::new(|| {
    let mut m: HashMap<String, &ShellInformation> = HashMap::new();

    let arr = &*SHELLS_REMOTE.shells;

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
pub fn get_info(name: &str) -> [String; 2] {
    let homedir = home();

    let cfg = (*SHELLS_INFO)
        .get(name)
        .expect("Shell configuration file not found!");

    let cfg_file = format!("{homedir}/{}", cfg.config);
    create_file(cfg_file.as_str()).expect("Unable to create/found config file for terminal");

    let alias_file = format!("{homedir}/{}", cfg.alias);
    create_file(alias_file.as_str()).expect("Unable to create/found config file for terminal");

    [cfg_file, alias_file]
}

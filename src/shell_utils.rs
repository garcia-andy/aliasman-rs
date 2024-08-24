use crate::create_file;
use crate::proc::ProcessMan;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Struct representing the files configurations of an specific shell
pub struct ShellConfig {
    /// The file from save the aliases
    pub alias_file: String,
    /// The file to write the configurations
    pub config_file: String,
}

const DEFAULT_ALIAS_FILE: &str = ".aliasman";
static SHELLS_REGISTER_CONFIGS: LazyLock<Vec<[&str; 3]>> = LazyLock::new(|| {
    vec![
        ["bash", ".bashrc", DEFAULT_ALIAS_FILE],
        ["zsh", ".zshrc", DEFAULT_ALIAS_FILE],
        [
            "fish",
            ".config/fish/conf.fish",
            ".config/fish/conf.d/aliases.fish",
        ],
    ]
});

static SHELLS_INFO: LazyLock<HashMap<String, ShellConfig>> = LazyLock::new(|| {
    let mut m: HashMap<String, ShellConfig> = HashMap::new();

    let arr = &*SHELLS_REGISTER_CONFIGS;

    for [name, conf, alias] in arr {
        m.insert(
            (*name).to_string(),
            ShellConfig {
                alias_file: (*alias).to_string(),
                config_file: (*conf).to_string(),
            },
        );
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

/// Getting the name of the config file
/// # Panics
/// Panic on getting env var
pub fn get_shell_config_file() -> String {
    let shell_name = get_shell();
    let homedir = std::env::var("HOME")
        .expect("We required the $HOME Path for determinate the shell config file!");
    let cfg = (*SHELLS_INFO)
        .get(shell_name.as_str())
        .expect("Shell configuration file not found!");

    let cfg_file = format!("{homedir}/{}", cfg.config_file);
    create_file(cfg_file.as_str()).expect("Unable to create/found config file for terminal");
    cfg_file
}

/// Getting the name of the alias file
/// # Panics
/// Panic on getting env var
pub fn get_shell_aliases() -> String {
    let shell_name = get_shell();
    let homedir = std::env::var("HOME")
        .expect("We required the $HOME Path for determinate the shell config file!");
    let cfg = (*SHELLS_INFO)
        .get(shell_name.as_str())
        .expect("Shell configuration file not found!");

    let alias_file = format!("{homedir}/{}", cfg.alias_file);
    create_file(alias_file.as_str()).expect("Unable to create/found alias file for terminal");
    alias_file
}

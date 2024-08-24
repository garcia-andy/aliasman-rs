use crate::file_utils::*;
use crate::string_utils::*;
use std::fs::read_to_string;
use std::io::Write;

use anyhow::anyhow;
use anyhow::Result;

/// Struct with 2 strings:
/// 1. Alias name
/// 2. Alias command
pub struct Alias(pub String, pub String);

/// Struct for manage all the aliases
pub struct AliasMan {
    alias_file: String,
    aliases: Vec<Alias>,
}

impl AliasMan {
    /// Create a new instance of the Alias manager using the file specified
    /// # Panics
    /// Panic on `read_to_string`
    pub fn new(file: &str) -> Self {
        // read aliases file content
        let aliases = read_to_string(file).expect("Unavle to read the aliases config file");
        // vec of lines
        let aliases_vec: Vec<&str> = aliases.split(|c| c == '\n').collect();

        // objects to save
        let mut aliases_objs: Vec<Alias> = vec![];
        // line for line
        for line in aliases_vec {
            if line.trim().is_empty() {
                continue;
            }
            // separete items
            let strs: Vec<&str> = line.split('=').collect();
            // getting the alias name
            let alias_name = remove_init(strs[0], "alias ");
            // getting the command (without the " or ')!
            let alias_cmd = remove_both(strs[1], "'");

            aliases_objs.push(Alias(alias_name, alias_cmd));
        }

        Self {
            alias_file: String::from(file),
            aliases: aliases_objs,
        }
    }

    /// Add a new Alias
    /// # Errors
    /// `AnyHow` error for a repeated alias
    pub fn add(&mut self, a: Alias) -> Result<()> {
        for alias in &self.aliases {
            if alias.0 == a.0 {
                return Err(anyhow!("Repeated Alias!"));
            }
        }

        self.aliases.push(a);
        Ok(())
    }

    /// Remove an existent alias
    /// # Errors
    /// `AnyHow` error for a inexistent alias
    pub fn rm(&mut self, name: &str) -> Result<()> {
        let mut idx = 0;

        for element in &self.aliases {
            if element.0.trim() == name.trim() {
                break;
            }

            idx += 1;
        }

        if idx < self.aliases.len() {
            self.aliases.remove(idx);
        } else {
            return Err(anyhow!("Alias not exist!"));
        }

        Ok(())
    }

    /// List all registreds aliases
    pub fn list(&self) -> &Vec<Alias> {
        &self.aliases
    }

    /// List all aliases (only names)
    pub fn alias_names(&self) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        for e in &self.aliases {
            ret.push(e.0.clone());
        }
        ret
    }

    /// Write all changes in the specified file on construction
    /// # Errors
    /// Error at truncate file
    pub fn flush_changes(&self) -> Result<()> {
        let mut file = truncate_file(self.alias_file.as_str())?;

        for alias in &self.aliases {
            file.write_all(format!("alias {}='{}'\n", alias.0.trim(), alias.1.trim()).as_bytes())?;
        }

        file.flush()?;

        Ok(())
    }
}

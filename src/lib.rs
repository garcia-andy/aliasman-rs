/// Library with all features for the Alias Manager

/// Crate with alias features
pub mod alias;
/// Crate with CLI options and features
pub mod cli;
/// Crate with string utilities
pub mod string_utils;
/// Crate with file utilities
pub mod file_utils;
/// Crate with shell utilities
pub mod shell_utils;
/// Crate with process information utilities
pub mod proc;

use alias::AliasMan;
use file_utils::*;
use anyhow::{Ok, Result};
use shell_utils::{get_shell_aliases, get_shell_config_file};
use std::{fs::read_to_string, io::{self, BufWriter, Stderr, Stdout, Write}, path::Path};

pub struct Printer {
    out: BufWriter<Stdout>,
    err: BufWriter<Stderr>
}

impl Default for Printer {
    fn default() -> Self {
        Self::new()
    }
}

impl Printer {
    /// Create a new Default Printer
    pub fn new() -> Self {
        Self { 
            out: BufWriter::new(io::stdout()), 
            err: BufWriter::new(io::stderr())
        }
    }

    /// Write in the error stream
    pub fn err(&mut self, content: &str){
        let _ = write!(self.err, "{content}");
    }

    /// Write a string with a breakline content in the standard output
    /// # Errors
    /// Error on write call
    pub fn writeln(&mut self, content: &str) -> Result<&mut Self> 
    { self.write(format!("{content}\n").as_str()) }

    /// Write a simple string content in the standard output
    /// # Errors
    /// Error on write macro call
    pub fn write(&mut self, content: &str) -> Result<&mut Self> {
        write!(self.out, "{content}")?;
        Ok(self)
    }

    /// Flush all changes
    /// # Errors
    /// Error on flushing out and err streams
    pub fn flush(&mut self) -> Result<&mut Self> {
        self.out.flush()?;
        self.err.flush()?;
        Ok(self)
    }

}

/// Review the configuration for the shell and setting up if is nedded
/// # Errors
/// First error on `read_to_string`
pub fn setup_aliasman() -> Result<AliasMan> {
    let bash = get_shell_config_file();
    let alias = get_shell_aliases();
    let p = Path::new(bash.as_str());

    if p.exists() {
        let bash_content = read_to_string(bash.as_str())?;

        if !bash_content.contains(alias.as_str()) {
            let import_content = 
                if bash.contains("fish"){
                    "\nsource "
                }else{ "\n. " };

            let mut bash = mod_file(bash.as_str())?;
            bash.write_all(import_content.as_bytes())?;
            bash.write_all(alias.as_bytes())?;
        }

    }

    create_file( alias.as_str() )?;
    Ok(AliasMan::new(alias.as_str()))
}




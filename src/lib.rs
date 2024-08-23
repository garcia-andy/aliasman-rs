pub mod alias;
pub mod string_utils;
pub mod file_utils;
pub mod cli;
pub mod shell_utils;
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
    pub fn new() -> Self {
        Self { 
            out: BufWriter::new(io::stdout()), 
            err: BufWriter::new(io::stderr())
        }
    }

    pub fn err(&mut self, content: &str){
        let _ = write!(self.err, "{content}");
    }

    pub fn writeln(&mut self, content: &str) -> Result<&mut Self> 
    { self.write(format!("{content}\n").as_str()) }

    pub fn write(&mut self, content: &str) -> Result<&mut Self> {
        write!(self.out, "{content}")?;
        Ok(self)
    }

    pub fn flush(&mut self) -> Result<&mut Self> {
        self.out.flush()?;
        self.err.flush()?;
        Ok(self)
    }

}

pub fn setup_aliasman() -> Result<AliasMan> {
    let bash = get_shell_config_file();
    let alias = get_shell_aliases();
    let p = Path::new(bash.as_str());

    if p.exists() {
        let bash_content = read_to_string(bash.as_str())?;

        if !bash_content.contains(alias.as_str()) {
            let mut import_content = "\n. ";
            if bash.contains("fish"){
                import_content = "\nsource ";
            }

            let mut bash = mod_file(bash.as_str())?;
            bash.write_all(import_content.as_bytes())?;
            bash.write_all(alias.as_bytes())?;
        }

    }else{
        println!("Bash file not exist!");
    }

    create_file( alias.as_str() )?;
    Ok(AliasMan::new(alias.as_str()))
}




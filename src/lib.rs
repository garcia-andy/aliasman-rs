pub mod alias;
pub mod string_utils;
pub mod file_utils;
pub mod cli;

use alias::AliasMan;
use file_utils::*;
use anyhow::{Ok, Result};
use std::{fs::read_to_string, io::{self, BufWriter, Stderr, Stdout, Write}, path::Path};

pub struct Printer {
    out: BufWriter<Stdout>,
    err: BufWriter<Stderr>
}

impl Printer {
    pub fn new() -> Self {
        Self { 
            out: BufWriter::new(io::stdout()), 
            err: BufWriter::new(io::stderr())
        }
    }

    pub fn err(&mut self, content: &str){
        let _ = write!(self.err, "{}", content);
    }

    pub fn writeln(&mut self, content: &str) -> Result<&mut Self> 
    { self.write(format!("{}\n",content).as_str()) }

    pub fn write(&mut self, content: &str) -> Result<&mut Self> {
        write!(self.out, "{}", content)?;
        Ok(self)
    }

    pub fn flush(&mut self) -> Result<&mut Self> {
        self.out.flush()?;
        self.err.flush()?;
        Ok(self)
    }

}

pub const BASH_FILE: &str = ".bashrc";
pub const ALIAS_FILE: &str = ".aliasman.aliases";

pub fn get_bash_file() -> String 
{ format!("{}/{BASH_FILE}", std::env::var("HOME").unwrap()) }

pub fn get_alias_file() -> String 
{ format!("{}/{ALIAS_FILE}", std::env::var("HOME").unwrap()) }

pub fn setup_aliasman() -> Result<AliasMan> {
    let bash = get_bash_file();
    let alias = get_alias_file();
    let p = Path::new(bash.as_str());

    if p.exists() {
        let bash_content = read_to_string(bash.as_str())?;

        if !bash_content.contains(alias.as_str()) {
            let mut bash = mod_file(bash.as_str())?;
            bash.write(b"\n. ")?;
            bash.write_all(alias.as_str().as_bytes())?;
        }

    }else{
        println!("Bash file not exist!")
    }

    create_file( alias.as_str() )?;
    Ok(AliasMan::new(alias.as_str()))
}




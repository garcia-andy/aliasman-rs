use std::fs::{OpenOptions, File};
use anyhow::Result;

pub fn open_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().truncate(false).write(true).read(true).create(true).open(file)?)
}

pub fn mod_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().append(true).open(file)?)
}

pub fn truncate_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().create(true).write(true).truncate(true).open(file)?)
}

pub fn onlyread_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().read(true).open(file)?)
}

pub fn create_file(file: &str) -> Result<()> {
    open_file(file)?;
    Ok(())
}

pub fn get_file_size(f: &File) -> Option<usize> 
{ f.metadata().map(|m| m.len() as usize).ok() }

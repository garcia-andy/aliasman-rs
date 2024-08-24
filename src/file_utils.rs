use std::fs::{OpenOptions, File};
use anyhow::Result;

/// Open a file with r/w permissions
/// if not exist error
/// # Errors
/// Error if fail on open options
pub fn open_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().truncate(false).write(true).read(true).create(true).open(file)?)
}

/// Open a file in add mode
/// if not exist error
/// # Errors
/// Error if fail on open options
pub fn mod_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().append(true).open(file)?)
}

/// Open a file in w mode
/// if not exist is created
/// # Errors
/// Error if fail on open options
pub fn truncate_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().create(true).write(true).truncate(true).open(file)?)
}

/// Open a file in r mode
/// if not exist error
/// # Errors
/// Error if fail on open options
pub fn onlyread_file(file: &str) -> Result<File> {
    Ok(OpenOptions::new().read(true).open(file)?)
}

/// If the file not exist, is created
/// # Errors
/// Error if fail on open options
pub fn create_file(file: &str) -> Result<()> {
    open_file(file)?;
    Ok(())
}

/// Getting the size of a file from his metadata
pub fn get_file_size(f: &File) -> Option<usize> 
{ f.metadata().map(|m| usize::try_from(m.len()).expect("Error on u64 -> usize convertion") ).ok() }

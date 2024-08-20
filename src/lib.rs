
use anyhow::Result;
use std::io::{self, BufWriter, Stderr, Stdout, Write};

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
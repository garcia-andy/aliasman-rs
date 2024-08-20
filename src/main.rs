use aliasman::Printer;
use clap::Parser;

/// 
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The Message
    #[arg(short, long, default_value_t = String::from("Hello World"))]
    message: String,
}

fn main() {
    let args = Cli::parse();
    let mut printer = Printer::new();
    
    printer.writeln(&args.message).unwrap()
        .flush().unwrap();
}

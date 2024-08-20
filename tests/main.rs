use std::time::Duration;
use aliasman::Printer;


#[test]
fn print_a_message(){
    let mut printer = Printer::new();
    printer.writeln("Hola Mundo").unwrap();

    std::thread::sleep(Duration::from_secs(1));

    printer.flush().unwrap();
}
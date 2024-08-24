use aliasman::Printer;
/// Testing the Printer
use std::time::Duration;

#[test]
fn print_a_message() {
    let mut printer = Printer::new();
    printer
        .writeln("Hola Mundo")
        .expect("Error for write message");

    std::thread::sleep(Duration::from_secs(1));

    printer.flush().expect("Error on flush streams");
}

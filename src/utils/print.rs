use ansi_rgb::*;
use rgb::RGB8;

/// colored for info
fn info_color(input: &str) -> WithForeground<&str> {
    input.fg(cyan_blue())
}

/// Colorized print
#[allow(clippy::print_stdout)]
pub fn pclr(sms: &str, color: RGB8) {
    print!("{}", sms.fg(color));
}

/// Colorized println
#[allow(clippy::print_stdout)]
pub fn pclrln(sms: &str, color: RGB8) {
    println!("{}", sms.fg(color));
}

/// Show an status and his message
#[allow(clippy::print_stdout)]
pub fn status(state: &str, sms: &str) {
    print!("[{}] {}", state.fg(yellow()), info_color(sms));
}

/// Show general information
#[allow(clippy::print_stdout)]
pub fn info(sms: &str) {
    print!("{}", info_color(sms));
}

/// Show error!
#[allow(clippy::print_stdout)]
pub fn err(sms: &str) {
    eprint!("[{}] {}", "ERROR".fg(red()), sms.fg(orange()));
}

/// Showing info only on debug
#[cfg(debug_assertions)]
#[allow(clippy::print_stdout)]
pub fn debug(sms: &str) {
    print!("{}", info_color(sms));
}

/// Showing info only on debug
#[cfg(not(debug_assertions))]
pub fn debug(_sms: &str) {}

// With BreakLines

/// Show an status and his message
#[allow(clippy::print_stdout)]
pub fn status_ln(state: &str, sms: &str) {
    println!("[{}] {}", state.fg(yellow()), info_color(sms));
}

/// Show general information
#[allow(clippy::print_stdout)]
pub fn info_ln(sms: &str) {
    println!("{}", info_color(sms));
}

/// Show error!
#[allow(clippy::print_stdout)]
pub fn err_ln(sms: &str) {
    eprintln!("[{}] {}", "ERROR".fg(red()), sms.fg(orange()));
}

/// Showing info only on debug
#[cfg(debug_assertions)]
#[allow(clippy::print_stdout)]
pub fn debug_ln(sms: &str) {
    println!("{}", info_color(sms));
}

/// Showing info only on debug
#[cfg(not(debug_assertions))]
pub fn debug_ln(_sms: &str) {}

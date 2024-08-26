//! in build.rs

/// Main entry of the build script
fn main() {
    dotenv_build::output(dotenv_build::Config::default()).expect("cannot .env file load");
}

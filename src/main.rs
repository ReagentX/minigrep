use std::process;

extern crate minigrep;  // Required to not throw import errors (not in the book)
use minigrep::Config;

fn main() {
    // Build config
    let config = Config::new().unwrap_or_else(|err| {
        eprintln!("Unable to parse arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}

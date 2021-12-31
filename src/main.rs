extern crate rust_minigrep;

use std::env;
use std::process;

use rust_minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    if let Err(e) = rust_minigrep::run(config) {
        eprintln!("Application Error {}", e);
        process::exit(1);
    }
}


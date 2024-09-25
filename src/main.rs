use std::{env, process};

use minigrep::{run, Config};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parse arguments: {err}");
        process::exit(1);
    });

    println!("Search query string {}", config.query);
    println!("In file path {}", config.file_path);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

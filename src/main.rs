use std::{env, fs};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_path: String = args[2].clone();

        Config { query, file_path }
    }
}

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path).unwrap();

    println!("with text:\n{contents}");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Search query string {}", config.query);
    println!("In file path {}", config.file_path);

    run(config);
}

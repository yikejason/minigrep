use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let file_path = &args[2];

    println!("Search query string {}", query);
    println!("In file path {}", file_path);

    let contents = fs::read_to_string(file_path).unwrap();

    println!("with text:\n{contents}");
}

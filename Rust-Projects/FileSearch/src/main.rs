use rust_search::SearchBuilder;
use std::io::{self, Write};

fn get_input(text: String) -> String {
    print!("{}", text);
    io::stdout().flush().unwrap();
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("Failed to read line");
    inp.trim().to_string()
}

///Users/octavoprita/Documents/Projects/Rust

fn search_file(location: String, ext: String) {
    let files: Vec<String> = SearchBuilder::default()
        .location(location)
        .ext(ext)
        .build()
        .collect();

    for path in files {
        println!("{}", path);
    }
}

fn main() {
    println!("Simple File Search");
    search_file(
        get_input("Search Path: ".to_string()),
        get_input("Extension: ".to_string()),
    );
}

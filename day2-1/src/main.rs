use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
}

fn read_lines() -> Vec<String> {
    read_to_string("input_file.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

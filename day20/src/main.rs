pub mod structs;
use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let numbers = parse_numbers(file_contents);

    match part.as_str() {
        "p1" => p1(numbers),
        _ => println!(""),
    }
}

fn p1(numbers: Vec<i64>) {
    
}

fn parse_numbers(file_contents: String) -> Vec<i64> {
    file_contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}
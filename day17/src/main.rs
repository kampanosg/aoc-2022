pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let wind_pattern = parse_wind(file_contents);

    match part.as_str() {
        "p1" => p1(wind_pattern),
        _ => println!(""),
    }
}

fn p1(wind_pattern: Vec<structs::Wind>) {
    dbg!(wind_pattern);
}

fn parse_wind(file_contents: String) -> Vec<structs::Wind> {
    file_contents
        .trim_end_matches("\n")
        .chars()
        .map(|c| match c {
            '>' => structs::Wind::Right,
            '<' => structs::Wind::Left,
            _ => panic!("not valid wind input")
        })
        .collect::<Vec<structs::Wind>>()
}

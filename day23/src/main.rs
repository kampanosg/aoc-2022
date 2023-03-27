pub mod structs;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let elves = parse_elves(&file_contents);

    match part.as_str() {
        "p1" => p1(elves),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(elves: HashSet<structs::Coordinate>) {

}

fn parse_elves(input: &str) -> HashSet<structs::Coordinate> {
    let mut elves = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(structs::Coordinate {
                    x: col as i32,
                    y: row as i32,
                });
            }
        }
    }
    elves
}

pub mod rucksack;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");

    let part = env::args().nth(2).expect("param not provided: part");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let rucksacks = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|r| rucksack::Rucksack::new(r))
        .collect::<Vec<rucksack::Rucksack>>();

    println!("{:?}", rucksacks);

    match part.as_str() {
        "p1" => p1(),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1() {}

pub mod rucksack;
use std::{env, fs, collections::HashSet};

use rucksack::Rucksack;

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

    match part.as_str() {
        "p1" => p1(rucksacks),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(rucksacks: Vec<Rucksack>) {
    let mut total_score = 0;
    for rucksack in rucksacks {
        let (first_compartment, second_compartment) = rucksack.compartments();
        let first_set: HashSet<char> = first_compartment.chars().into_iter().collect();
        let second_set: HashSet<char> = second_compartment.chars().into_iter().collect();
        total_score += first_set
            .intersection(&second_set)
            .into_iter()
            .map(|c| get_score(*c))
            .collect::<Vec<i32>>()
            .iter()
            .sum::<i32>();
    }
    println!("total: {}", total_score);
}

fn get_score(c: char) -> i32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => -1,
    }
}

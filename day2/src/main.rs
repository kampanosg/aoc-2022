pub mod enums;
use std::{env, fs};

// Rules:
//
// Points: Loss = 0, Draw = 3, Win = 6
// Points for hand: Rock = 1, Paper = 2, Scissors = 3
//
// p1 = Rock = A, Paper = B, Scissors = C
// p2 = Rock = X, Paper = Y, Scissors = Z

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let rounds = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let mut total_points: u64 = 0;
    for round in rounds.iter() {
        total_points += calculate_round_points(round); 
    }
    println!("total points: {}", total_points);
}

fn transform_round(round: &str) -> (enums::Hand, enums::Hand) {
    let hands: Vec<&str> = round.trim_end_matches("\n").split(" ").collect();

    let p1 = hands[0];
    let p2 = hands[1];

    let hand1 = enums::Hand::transform(p1).expect("malformed input");
    let hand2 = enums::Hand::transform(p2).expect("malformed input");

    (hand1, hand2)
}

fn determine_result(hand1: enums::Hand, hand2: enums::Hand) -> enums::Result {
    match hand2 {
        enums::Hand::Rock => match hand1 {
            enums::Hand::Rock => enums::Result::Draw,
            enums::Hand::Paper => enums::Result::Lose,
            enums::Hand::Scissors => enums::Result::Win,
        },
        enums::Hand::Paper => match hand1 {
            enums::Hand::Rock => enums::Result::Win,
            enums::Hand::Paper => enums::Result::Draw,
            enums::Hand::Scissors => enums::Result::Lose,
        },
        enums::Hand::Scissors => match hand1 {
            enums::Hand::Rock => enums::Result::Lose,
            enums::Hand::Paper => enums::Result::Win,
            enums::Hand::Scissors => enums::Result::Draw,
        },
    };

    enums::Result::Win
}

fn calculate_round_points(round: &str) -> u64 {
    let (p1, p2) = transform_round(round);
    let result = determine_result(p1, p2);
    result.points() + p2.bonus_points()
}

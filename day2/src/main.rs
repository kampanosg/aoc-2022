pub mod enums;
use std::{env, fs};

// Rules:
//
// Points: Loss = 0, Draw = 3, Win = 6
// Points for hand: Rock = 1, Paper = 2, Scissors = 3
//
// p1 = Rock = A, Paper = B, Scissors = C
// Part 1: Rock = X, Paper = Y, Scissors = Z
// Part 2: X = Lose, Y = Draw, Z = Win

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args()
        .nth(2)
        .expect("param not provided: part (valid options: p1 | p2)");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let rounds = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(rounds),
        "p2" => p2(rounds),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(rounds: Vec<&str>) {
    let mut total_points: u64 = 0;
    for round in rounds.iter() {
        let (opponent_hand, player_hand) = transform_round_p1(round);
        total_points += calc_round_points(opponent_hand, player_hand);
    }
    println!("total points: {}", total_points);
}

fn p2(rounds: Vec<&str>) {
    let mut total_points: u64 = 0;
    for round in rounds.iter() {
        let (opponent_hand, desired_outcome) = transform_round_p2(round);
        let player_hand = determine_hand(opponent_hand, desired_outcome);
        total_points += calc_round_points(opponent_hand, player_hand);
    }
    println!("total points: {}", total_points);
}

fn calc_round_points(hand1: enums::Hand, hand2: enums::Hand) -> u64 {
    let result = determine_result(hand1, hand2);
    result.points() + hand2.bonus_points()
}

fn transform_round_p1(round: &str) -> (enums::Hand, enums::Hand) {
    let hands: Vec<&str> = round.trim_end_matches("\n").split(" ").collect();

    let opponent_hand = enums::Hand::transform(hands[1]).expect("malformed input");
    let player_hand = enums::Hand::transform(hands[1]).expect("malformed input");

    (opponent_hand, player_hand)
}

fn transform_round_p2(round: &str) -> (enums::Hand, enums::Outcome) {
    let round: Vec<&str> = round.trim_end_matches("\n").split(" ").collect();

    let opponent_hand = enums::Hand::transform(round[0]).expect("malformed input");
    let desired_outcome = enums::Outcome::transform(round[1]).expect("malformed input");

    (opponent_hand, desired_outcome)
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
    }
}

fn determine_hand(opponent_hand: enums::Hand, desired_outcome: enums::Outcome) -> enums::Hand {
    match desired_outcome {
        enums::Outcome::Win => match opponent_hand {
            enums::Hand::Rock => enums::Hand::Paper,
            enums::Hand::Paper => enums::Hand::Scissors,
            enums::Hand::Scissors => enums::Hand::Rock,
        },
        enums::Outcome::Draw => match opponent_hand {
            enums::Hand::Rock => enums::Hand::Rock,
            enums::Hand::Paper => enums::Hand::Paper,
            enums::Hand::Scissors => enums::Hand::Scissors,
        },
        enums::Outcome::Lose => match opponent_hand {
            enums::Hand::Rock => enums::Hand::Scissors,
            enums::Hand::Paper => enums::Hand::Rock,
            enums::Hand::Scissors => enums::Hand::Paper,
        },
    }
}

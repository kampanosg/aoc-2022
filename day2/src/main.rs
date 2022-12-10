pub mod enums;
pub mod game;
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
    
    for round in rounds.iter() {
        let (p1, p2) = transform_round(round);
        let result = game::determine_result(p1, p2);
        println!("{:?}, {:?}", p1, p2);
        break;
    }
}

fn transform_round(round: &str) -> (enums::Hand, enums::Hand) {
    let hands: Vec<&str> = round
        .trim_end_matches("\n")
        .split(" ")
        .collect();

    let p1 = hands[0];
    let p2 = hands[1];

    let hand1 = enums::Hand::transform(p1).expect("malformed input");
    let hand2 = enums::Hand::transform(p2).expect("malformed input");
    
    return (hand1, hand2);
}

pub fn determine_result(hand1: enums::Hand, hand2: enums::Hand) -> enums::Result {
    match hand2 {
        enums::Hand::Rock => {
            match hand1 {
                enums::Hand::Rock => enums::Result::Draw,
                enums::Hand::Paper => enums::Result::Lose,
                enums::Hand::Scissors => enums::Result::Win,
            }
        },
        enums::Hand::Paper => {
            match hand1 {
                enums::Hand::Rock => enums::Result::Win,
                enums::Hand::Paper => enums::Result::Draw,
                enums::Hand::Scissors => enums::Result::Lose,
            }
        },
        enums::Hand::Scissors => {
            match hand1 {
                enums::Hand::Rock => enums::Result::Lose,
                enums::Hand::Paper => enums::Result::Win,
                enums::Hand::Scissors => enums::Result::Draw,
            }
        },
    };

    return enums::Result::Win;
}

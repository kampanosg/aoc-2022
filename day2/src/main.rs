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
    
    for round in rounds.iter() {
        let (p1, p2) = transform_round(round);
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

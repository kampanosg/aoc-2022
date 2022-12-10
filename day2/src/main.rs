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

fn calculate_round_points(round: &str) -> u64 {
    let (p1, p2) = transform_round(round);
    let result = determine_result(p1, p2);
    result.points() + p2.bonus_points()
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
    }
}

#[cfg(test)]
mod tests {
    use crate::calculate_round_points;

    #[test]
    fn test_calculate_round_points_winner() {
        let round1 = "C X"; // scissors v rock
        let round2 = "A Y"; // rock v paper
        let round3 = "B Z"; // paper v scissors

        let res1 = calculate_round_points(round1);
        let res2 = calculate_round_points(round2);
        let res3 = calculate_round_points(round3);

        assert_eq!(res1, 7);
        assert_eq!(res2, 8);
        assert_eq!(res3, 9);
    }

    #[test]
    fn test_calculate_round_points_draw() {
        let round1 = "A X"; // rock v rock
        let round2 = "B Y"; // paper c paper
        let round3 = "C Z"; // scissors v scissors

        let res1 = calculate_round_points(round1);
        let res2 = calculate_round_points(round2);
        let res3 = calculate_round_points(round3);

        assert_eq!(res1, 4);
        assert_eq!(res2, 5);
        assert_eq!(res3, 6);
    }

    #[test]
    fn test_calculate_round_points_lose() {
        let round1 = "B X"; // paper v rock
        let round2 = "C Y"; // scissors c paper
        let round3 = "A Z"; // rock v scissors

        let res1 = calculate_round_points(round1);
        let res2 = calculate_round_points(round2);
        let res3 = calculate_round_points(round3);

        assert_eq!(res1, 1);
        assert_eq!(res2, 2);
        assert_eq!(res3, 3);
    }
}

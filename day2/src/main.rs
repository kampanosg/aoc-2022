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
    
    let rounds: Vec<&str> = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect();
    
    for round in rounds.iter() {
        println!("{}", round);
        break;
    }
}

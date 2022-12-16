pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let forrest = parse_forrest(file_contents);
    // match part.as_str() {
    //     "p1" => p1(signal),
    //     _ => println!(""),
    // }
}

fn p1(forrest: Vec<structs::Tree>) {}

fn parse_forrest(file_contents: String) {
    let mut x = 1;
    let mut y = 1;
    let mut forrest: Vec<Vec<structs::Tree>> = vec![];

    let rows = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    for row in rows {
        let mut trees: Vec<structs::Tree> = vec![];
        let cols = row.split("").collect::<Vec<&str>>();
        let heights = cols
            .iter()
            .filter(|c| **c != "")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        x = 1;
        for height in heights {
            trees.push(structs::Tree::new(height, y, x));
            x += 1;
        }
        y += 1;

        forrest.push(trees);
    }
    println!("{:?}", forrest);
}

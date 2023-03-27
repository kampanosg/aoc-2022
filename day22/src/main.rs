pub mod structs;
use std::{env, fs};

fn main() {
    println!("Hello, world!");
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let moves = parse_moves(&file_contents);

    match part.as_str() {
        "p1" => p1(moves),
        _ => println!(""),
    }
}

fn p1(moves: (Vec<Vec<structs::Block>>, Vec<structs::Move>)) {
    println!("p1");
}

fn parse_moves(input: &str) -> (Vec<Vec<structs::Block>>, Vec<structs::Move>) {
    let (grid, moves) = input.trim_end().split_once("\n\n").unwrap();
    let mut instructions = Vec::new();
    let mut digits = Vec::new();
    for c in moves.chars() {
        if c.is_numeric() {
            let digit = c.to_digit(10).unwrap() as u8;
            digits.push(digit);
        } else {
            let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
            digits.clear();
            instructions.push(structs::Move::Up(num));

            // parse turn
            let turn = match c {
                'L' => structs::Turn::L,
                'R' => structs::Turn::R,
                _ => panic!("Invalid input"),
            };
            instructions.push(structs::Move::Rotate(turn));
        }
    }
    let num = digits.iter().fold(0, |num, digit| num * 10 + digit);
    instructions.push(structs::Move::Up(num));

    let mut map = Vec::new();
    for line in grid.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            let block = match c {
                '.' => structs::Block::Available,
                '#' => structs::Block::Wall,
                ' ' => structs::Block::Empty,
                _ => panic!("invalid input"),
            };
            row.push(block);
        }
        map.push(row);
    }
    (map, instructions)
}
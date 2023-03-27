pub mod structs;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let map = parse_map(&file_contents);

    match part.as_str() {
        "p1" => p1(map),
        _ => println!(""),
    }
}

fn p1(map: (HashMap<structs::Coordinate, structs::Block>, usize, usize)) {
    println!("p1");
}

fn parse_map(input: &str) -> (HashMap<structs::Coordinate, structs::Block>, usize, usize) {
    let mut map = HashMap::new();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = structs::Coordinate { y, x };
            let block = match c {
                '#' => structs::Block::Wall,
                '^' => structs::Block::Wind(structs::Compass::Up),
                'v' => structs::Block::Wind(structs::Compass::Down),
                '<' => structs::Block::Wind(structs::Compass::Left),
                '>' => structs::Block::Wind(structs::Compass::Right),
                _ => panic!("invalid input"),
            };
            map.insert(coord, block);
        }
    }
    (map, rows, cols)
}

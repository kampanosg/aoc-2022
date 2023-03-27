pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let moves = parse_moves(&file_contents);

    match part.as_str() {
        "p1" => p1(moves),
        _ => println!(""),
    }
}

fn p1(input: (Vec<Vec<structs::Block>>, Vec<structs::Move>)) {
    let (map, instructions) = input;
    let start_col = map[0]
        .iter()
        .position(|tile| *tile == structs::Block::Available)
        .unwrap() as i32;

    let mut current_pos = structs::Coordinate {
        row: 0,
        col: start_col,
    };
    let mut dir = structs::Direction::R;

    for inst in &instructions {
        match inst {
            structs::Move::Rotate(turn) => dir = dir.turn(turn),
            structs::Move::Up(amount) => {
                for _ in 0..*amount {
                    let structs::Coordinate { row: dr, col: dc } = dir.offset();
                    let next_tile = map
                        .get((current_pos.row + dr) as usize)
                        .and_then(|row| row.get((current_pos.col + dc) as usize))
                        .unwrap_or(&structs::Block::Empty);

                    match next_tile {
                        structs::Block::Available => {
                            current_pos = structs::Coordinate {
                                row: current_pos.row + dr,
                                col: current_pos.col + dc,
                            };
                        }
                        structs::Block::Wall => break,
                        structs::Block::Empty => {
                            let new_pos = wrap(&map, &current_pos, &dir);
                            if map[new_pos.row as usize][new_pos.col as usize]
                                == structs::Block::Wall
                            {
                                break;
                            }
                            current_pos = new_pos;
                        }
                    }
                }
            }
        }
    }

    let res = 1000 * (current_pos.row + 1) + 4 * (current_pos.col + 1) + dir.score() as i32;
    println!("res: {}", res)
}

fn wrap(
    map: &[Vec<structs::Block>],
    pos: &structs::Coordinate,
    dir: &structs::Direction,
) -> structs::Coordinate {
    let structs::Coordinate { row: dr, col: dc } = dir.offset();
    let mut curr = pos.clone();
    while let Some(tile) = map
        .get((curr.row - dr) as usize)
        .and_then(|row| row.get((curr.col - dc) as usize))
    {
        if *tile == structs::Block::Empty {
            break;
        }
        curr = structs::Coordinate {
            row: curr.row - dr,
            col: curr.col - dc,
        };
    }
    curr
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

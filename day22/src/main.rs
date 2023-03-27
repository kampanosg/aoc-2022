pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let moves = parse_moves(&file_contents);

    match part.as_str() {
        "p1" => p1(moves),
        "p2" => p2(moves),
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

pub fn p2(input: (Vec<Vec<structs::Block>>, Vec<structs::Move>)) {
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
                    let new_tile = map
                        .get((current_pos.row + dr) as usize)
                        .and_then(|row| row.get((current_pos.col + dc) as usize))
                        .unwrap_or(&structs::Block::Empty);

                    match new_tile {
                        structs::Block::Available => {
                            current_pos = structs::Coordinate {
                                row: current_pos.row + dr,
                                col: current_pos.col + dc,
                            };
                        }
                        structs::Block::Empty => {
                            let (new_pos, new_dir) = turn_around(&current_pos, &dir);
                            if map[new_pos.row as usize][new_pos.col as usize]
                                == structs::Block::Wall
                            {
                                break;
                            }
                            current_pos = new_pos;
                            dir = new_dir
                        }
                        structs::Block::Wall => break,
                    }
                }
            }
        }
    }

    let res = 1000 * (current_pos.row + 1) + 4 * (current_pos.col + 1) + dir.score() as i32;
    println!("res: {}", res);
}

fn turn_around(
    pos: &structs::Coordinate,
    dir: &structs::Direction,
) -> (structs::Coordinate, structs::Direction) {
    let (cube_row, cube_col, new_dir) = match (pos.row / 50, pos.col / 50, dir) {
        (0, 1, structs::Direction::U) => (3, 0, structs::Direction::R),
        (0, 1, structs::Direction::L) => (2, 0, structs::Direction::R),
        (0, 2, structs::Direction::U) => (3, 0, structs::Direction::U),
        (0, 2, structs::Direction::R) => (2, 1, structs::Direction::L),
        (0, 2, structs::Direction::D) => (1, 1, structs::Direction::L),
        (1, 1, structs::Direction::R) => (0, 2, structs::Direction::U),
        (1, 1, structs::Direction::L) => (2, 0, structs::Direction::D),
        (2, 0, structs::Direction::U) => (1, 1, structs::Direction::R),
        (2, 0, structs::Direction::L) => (0, 1, structs::Direction::R),
        (2, 1, structs::Direction::R) => (0, 2, structs::Direction::L),
        (2, 1, structs::Direction::D) => (3, 0, structs::Direction::L),
        (3, 0, structs::Direction::R) => (2, 1, structs::Direction::U),
        (3, 0, structs::Direction::D) => (0, 2, structs::Direction::D),
        (3, 0, structs::Direction::L) => (0, 1, structs::Direction::D),
        _ => unreachable!(),
    };
    let (row_idx, col_idx) = (pos.row % 50, pos.col % 50);

    let i = match dir {
        structs::Direction::L => 49 - row_idx,
        structs::Direction::R => row_idx,
        structs::Direction::U => col_idx,
        structs::Direction::D => 49 - col_idx,
    };

    // find new idxes within the cube
    let new_row = match new_dir {
        structs::Direction::L => 49 - i,
        structs::Direction::R => i,
        structs::Direction::U => 49,
        structs::Direction::D => 0,
    };
    let new_col = match new_dir {
        structs::Direction::L => 49,
        structs::Direction::R => 0,
        structs::Direction::U => i,
        structs::Direction::D => 49 - i,
    };

    let new_pos = structs::Coordinate {
        row: cube_row * 50 + new_row,
        col: cube_col * 50 + new_col,
    };

    (new_pos, new_dir)
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

pub mod structs;
use std::{env, fs};

const CAVE_WIDTH: usize = 7;
const ROCKS: [&[structs::Coord]; 5] = [
    // -
    &[
        structs::Coord { x: 0, y: 0 },
        structs::Coord { x: 1, y: 0 },
        structs::Coord { x: 2, y: 0 },
        structs::Coord { x: 3, y: 0 },
    ],
    // +
    &[
        structs::Coord { x: 0, y: 1 },
        structs::Coord { x: 1, y: 0 },
        structs::Coord { x: 1, y: 1 },
        structs::Coord { x: 1, y: 2 },
        structs::Coord { x: 2, y: 1 },
    ],
    // reversed L
    &[
        structs::Coord { x: 0, y: 0 },
        structs::Coord { x: 1, y: 0 },
        structs::Coord { x: 2, y: 0 },
        structs::Coord { x: 2, y: 1 },
        structs::Coord { x: 2, y: 2 },
    ],
    // |
    &[
        structs::Coord { x: 0, y: 0 },
        structs::Coord { x: 0, y: 1 },
        structs::Coord { x: 0, y: 2 },
        structs::Coord { x: 0, y: 3 },
    ],
    // cube
    &[
        structs::Coord { x: 0, y: 0 },
        structs::Coord { x: 1, y: 0 },
        structs::Coord { x: 0, y: 1 },
        structs::Coord { x: 1, y: 1 },
    ],
];

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let wind_pattern = parse_wind(file_contents);

    match part.as_str() {
        "p1" => p1(wind_pattern),
        _ => println!(""),
    }
}

fn p1(wind_pattern: Vec<structs::Wind>) {
    let target = 2022;
    let mut state = structs::CaveState {
        wind_count: 0,
        rock_count: 0,
        ceiling: 0,
        cave: vec![],
        current_rock: structs::Coord { x: 0, y: 0 },
    };

    while state.rock_count != target {
        let piece = ROCKS[state.rock_count as usize % ROCKS.len()];
        state.current_rock.x = 2;
        state.current_rock.y = state.ceiling + 3;

        loop {
            let jet = &wind_pattern[state.wind_count as usize % wind_pattern.len()];
            let new_curr = match jet {
                structs::Wind::Right => structs::Coord {
                    x: state.current_rock.x + 1,
                    y: state.current_rock.y,
                },
                structs::Wind::Left => structs::Coord {
                    x: state.current_rock.x.saturating_sub(1),
                    y: state.current_rock.y,
                },
            };
            if state.has_not_collided(&new_curr, piece) {
                state.current_rock = new_curr;
            }
            state.wind_count += 1;

            let new_curr = structs::Coord {
                x: state.current_rock.x,
                y: state.current_rock.y.saturating_sub(1),
            };
            if state.current_rock.y == 0 || !state.has_not_collided(&new_curr, piece) {
                break;
            }
            state.current_rock = new_curr;
        }

        for offset in piece {
            let x = state.current_rock.x + offset.x;
            let y = state.current_rock.y + offset.y;
            while state.cave.len() <= y as usize {
                state.cave.push([false; CAVE_WIDTH]);
            }
            state.cave[y as usize][x as usize] = true;
            // y is 0 indexed.
            state.ceiling = state.ceiling.max(y + 1);
        }

        // prepare for next iteration of while loop
        state.rock_count += 1;
    }

    dbg!(state.ceiling);
}

fn parse_wind(file_contents: String) -> Vec<structs::Wind> {
    file_contents
        .trim_end_matches("\n")
        .chars()
        .map(|c| match c {
            '>' => structs::Wind::Right,
            '<' => structs::Wind::Left,
            _ => panic!("not valid wind input"),
        })
        .collect::<Vec<structs::Wind>>()
}

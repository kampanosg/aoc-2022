pub mod structs;
use std::{collections::HashMap, env, fs};

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
        "p2" => p2(wind_pattern),
        _ => println!(""),
    }
}

fn p1(wind_pattern: Vec<structs::Wind>) {
    let rounds = 2022;

    let mut cave_state = structs::CaveState {
        wind_count: 0,
        rock_count: 0,
        ceiling: 0,
        cave: vec![],
        current_rock: structs::Coord { x: 0, y: 0 },
        rock_patterns: HashMap::new(),
        pattern_occurence_count: 0,
    };

    while cave_state.rock_count != rounds {
        let next_rock = ROCKS[cave_state.rock_count as usize % ROCKS.len()];

        cave_state.current_rock.x = 2;
        cave_state.current_rock.y = cave_state.ceiling + 3;

        loop {
            let wind = &wind_pattern[cave_state.wind_count as usize % wind_pattern.len()];

            let new_rock = match wind {
                structs::Wind::Right => structs::Coord {
                    x: cave_state.current_rock.x + 1,
                    y: cave_state.current_rock.y,
                },
                structs::Wind::Left => structs::Coord {
                    x: cave_state.current_rock.x.saturating_sub(1),
                    y: cave_state.current_rock.y,
                },
            };

            if cave_state.has_not_collided(&new_rock, next_rock) {
                cave_state.current_rock = new_rock;
            }
            cave_state.wind_count += 1;

            let new_rock = structs::Coord {
                x: cave_state.current_rock.x,
                y: cave_state.current_rock.y.saturating_sub(1),
            };
            if cave_state.current_rock.y == 0 || !cave_state.has_not_collided(&new_rock, next_rock)
            {
                break;
            }
            cave_state.current_rock = new_rock;
        }

        for offset in next_rock {
            let x = (cave_state.current_rock.x + offset.x) as usize;
            let y = (cave_state.current_rock.y + offset.y) as usize;

            while cave_state.cave.len() <= y as usize {
                cave_state.cave.push([false; CAVE_WIDTH]);
            }
            cave_state.cave[y][x] = true;
            cave_state.ceiling = cave_state.ceiling.max(y as u32 + 1);
        }

        cave_state.rock_count += 1;
    }

    println!("ceiling: {}", cave_state.ceiling)
}

pub fn p2(wind_pattern: Vec<structs::Wind>) {
    let rounds: u64 = 1_000_000_000_000;

    let mut cave_state = structs::CaveState {
        wind_count: 0,
        rock_count: 0,
        ceiling: 0,
        cave: vec![],
        current_rock: structs::Coord { x: 0, y: 0 },
        rock_patterns: HashMap::new(),
        pattern_occurence_count: 0,
    };

    while cave_state.rock_count != rounds {

        let next_rock = ROCKS[cave_state.rock_count as usize % ROCKS.len()];
        cave_state.current_rock.x = 2;
        cave_state.current_rock.y = cave_state.ceiling + 3;

        loop {

            let wind = &wind_pattern[cave_state.wind_count as usize % wind_pattern.len()];
            let new_curr = match wind {
                structs::Wind::Right => structs::Coord {
                    x: cave_state.current_rock.x + 1,
                    y: cave_state.current_rock.y,
                },
                structs::Wind::Left => structs::Coord {
                    x: cave_state.current_rock.x.saturating_sub(1),
                    y: cave_state.current_rock.y,
                },
            };
            if cave_state.has_not_collided(&new_curr, next_rock) {
                cave_state.current_rock = new_curr;
            }

            cave_state.wind_count += 1;

            let new_curr = structs::Coord {
                x: cave_state.current_rock.x,
                y: cave_state.current_rock.y.saturating_sub(1),
            };
            if cave_state.current_rock.y == 0 || !cave_state.has_not_collided(&new_curr, next_rock) {
                break;
            }
            cave_state.current_rock = new_curr;
        }

        for offset in next_rock {
            let x = (cave_state.current_rock.x + offset.x) as usize;
            let y = (cave_state.current_rock.y + offset.y) as usize;

            while cave_state.cave.len() <= y {
                cave_state.cave.push([false; CAVE_WIDTH]);
            }

            cave_state.cave[y][x] = true;
            cave_state.ceiling = cave_state.ceiling.max(y as u32 + 1);
        }

        if cave_state.pattern_occurence_count == 0 {
            let key = (
                cave_state.rock_count as usize % ROCKS.len(),
                cave_state.wind_count as usize % wind_pattern.len(),
            );

            // To make p2 more efficient we have to observe the rock patterns
            // These patterns occur after a few runs of the loop
            // Add to the hashmap to improve efficiency instead of looping a trillion times
            if let Some((2, old_piece_count, old_top)) = cave_state.rock_patterns.get(&key) {
                let delta_top = cave_state.ceiling as usize - old_top;
                let delta_piece_count = cave_state.rock_count as usize - old_piece_count;
                let repeats = (rounds - cave_state.rock_count) as usize / delta_piece_count;
                cave_state.pattern_occurence_count += repeats * delta_top;
                cave_state.rock_count += (repeats * delta_piece_count) as u64;
            }
            
            cave_state
                .rock_patterns
                .entry(key)
                .and_modify(|(amnt, old_piece_count, old_top)| {
                    *amnt += 1;
                    *old_piece_count = cave_state.rock_count as usize;
                    *old_top = cave_state.ceiling as usize;
                })
                .or_insert((
                    1,
                    cave_state.rock_count as usize,
                    cave_state.ceiling as usize,
                ));
        }

        cave_state.rock_count += 1;
    }

    println!(
        "ceiling: {}",
        cave_state.ceiling as usize + cave_state.pattern_occurence_count
    );
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

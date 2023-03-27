pub mod structs;
use std::{collections::{HashSet, HashMap}, env, fs};
use itertools::Itertools;

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let elves = parse_elves(&file_contents);

    match part.as_str() {
        "p1" => p1(elves),
        "p2" => p2(elves),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(e: HashSet<structs::Coordinate>) {
    let mut elves = e.clone();
    let mut checks = [
        structs::Compass::North,
        structs::Compass::South,
        structs::Compass::West,
        structs::Compass::East,
    ];

    for round in 1.. {
        let mut proposals: HashMap<structs::Coordinate, Vec<structs::Coordinate>> = HashMap::new();

        for elf in &elves {
            let neighbours = elf.adjacent_coord();
            if neighbours.iter().all(|coord| !elves.contains(coord)) {
                continue;
            }
            let neighbours = neighbours
                .iter()
                .map(|neighbour| elves.contains(neighbour))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let proposed_dir = checks.iter().find(|dir| dir.check(&neighbours));
            if let Some(dir) = proposed_dir {
                let proposal = elf.add_coord(dir);
                proposals.entry(proposal).or_default().push(*elf);
            }
        }

        for (new_coord, old_coords) in proposals {
            if old_coords.len() == 1 {
                elves.remove(&old_coords[0]);
                elves.insert(new_coord);
            }
        }

        checks.rotate_left(1);
        if round == 10 {
                let (minmax_y, minmax_x) = elves.iter().fold(
                    ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
                    |(minmax_row, minmax_col), structs::Coordinate { y, x}| {
                        (
                            (minmax_row.0.min(*y), minmax_row.1.max(*y)),
                            (minmax_col.0.min(*x), minmax_col.1.max(*x)),
                        )
                    },
                );
                let res = (minmax_y.0..=minmax_y.1)
                    .cartesian_product(minmax_x.0..=minmax_x.1)
                    .filter(|(row, col)| {
                        !elves.contains(&structs::Coordinate {
                            y: *row,
                            x: *col,
                        })
                    })
                    .count()
                    .to_string();
                println!("res: {}", res);
            }
    }
}

pub fn p2(e: HashSet<structs::Coordinate>) {
    let mut elves = e.clone();
    let mut checks = [
        structs::Compass::North,
        structs::Compass::South,
        structs::Compass::West,
        structs::Compass::East,
    ];

    for round in 1.. {
        let mut moved = false;
        // key: proposed coordinate, val: list of old coordinates that proposed going there
        let mut proposals: HashMap<structs::Coordinate, Vec<structs::Coordinate>> = HashMap::new();

        // consideration phase
        for elf in &elves {
            let neighbours = elf.adjacent_coord();
            // If no other Elves are in one of those eight positions, the Elf does not do anything during this round.
            if neighbours.iter().all(|coord| !elves.contains(coord)) {
                continue;
            }
            let neighbours = neighbours
                .iter()
                .map(|neighbour| elves.contains(neighbour))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();

            let proposed_dir = checks.iter().find(|dir| dir.check(&neighbours));
            if let Some(dir) = proposed_dir {
                let proposal = elf.add_coord(dir);
                proposals.entry(proposal).or_default().push(*elf);
            }
        }

        // movement phase
        for (new_coord, old_coords) in proposals {
            if old_coords.len() == 1 {
                moved = true;
                elves.remove(&old_coords[0]);
                elves.insert(new_coord);
            }
        }

        // after round
        if !moved {
            println!("res: {}", round);
            return
        }
        checks.rotate_left(1);
    }

}

fn parse_elves(input: &str) -> HashSet<structs::Coordinate> {
    let mut elves = HashSet::new();
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert(structs::Coordinate {
                    x: col as i32,
                    y: row as i32,
                });
            }
        }
    }
    elves
}

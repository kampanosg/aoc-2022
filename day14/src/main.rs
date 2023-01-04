pub mod structs;
use std::{env, fs, collections::HashSet};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let parsed_cave = parse_cave(&file_contents);
    let cave = structs::Cave {
        rock_paths: parsed_cave,
        sand: structs::Coord { x: 500, y: 0 },
    };

    match part.as_str() {
        "p1" => p1(cave),
        _ => println!(""),
    }
}

fn p1(cave: structs::Cave) {
    let rock = structs::Rock::parse_from_rock_paths(&cave.rock_paths);
    let lowest_y = *rock.hor.keys().max().unwrap();
    let mut rested = HashSet::<structs::Coord>::new();
    let mut sand_particle = cave.sand;

    while sand_particle.y < lowest_y {
        let possible_moves = [
            structs::Coord {
                x: sand_particle.x,
                y: sand_particle.y + 1,
            },
            structs::Coord {
                x: sand_particle.x - 1,
                y: sand_particle.y + 1,
            },
            structs::Coord {
                x: sand_particle.x + 1,
                y: sand_particle.y + 1,
            },
        ];

        let next_pos = possible_moves
            .iter()
            .find(|p| !rock.is_touching(p) && !rested.contains(p));

        match next_pos {
            Some(p) => sand_particle = *p,
            None => {
                rested.insert(sand_particle);
                sand_particle = cave.sand;
            }
        }
    }

    println!("total = {}", rested.len());
}

fn parse_cave(input: &str) -> Vec<structs::Path> {
    let mut paths: Vec<structs::Path> = vec![];
    let input = input.trim().split("\n").collect::<Vec<&str>>();
    for line in input {
        let mut points: Vec<structs::Coord> = vec![];
        let coords = line.trim().split(" -> ").collect::<Vec<&str>>();
        for coord in coords {
            let coord = coord.split(",").collect::<Vec<&str>>();
            let point = structs::Coord {
                x: coord[0].parse::<i32>().unwrap(),
                y: coord[1].parse::<i32>().unwrap(),
            };
            points.push(point);
        }
        paths.push(points);
    }

    paths
}

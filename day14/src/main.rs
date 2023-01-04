pub mod structs;
use std::{collections::HashSet, env, fs};

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
        "p2" => p2(&cave),
        _ => println!(""),
    }
}

fn p1(cave: structs::Cave) {
    let rock = structs::Rock::parse_from_rock_paths(&cave.rock_paths);
    let lowest_y = *rock.hor.keys().max().unwrap();
    let mut counter = HashSet::<structs::Coord>::new();
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
            .find(|p| !rock.is_touching(p) && !counter.contains(p));

        match next_pos {
            Some(p) => sand_particle = *p,
            None => {
                counter.insert(sand_particle);
                sand_particle = cave.sand;
            }
        }
    }

    println!("total = {}", counter.len());
}

fn p2(cave: &structs::Cave) {
    let wall_edges = get_wall_edges(&cave.rock_paths);

    let mut rock_paths: Vec<structs::Path> = cave.rock_paths.clone();
    let cave_floor: structs::Path = vec![
        structs::Coord {
            x: i32::MIN,
            y: wall_edges.edge2.y + 2,
        },
        structs::Coord {
            x: i32::MAX,
            y: wall_edges.edge2.y + 2,
        },
    ];
    rock_paths.push(cave_floor);

    let cave = &structs::Cave {
        sand: cave.sand,
        rock_paths,
    };

    let wall = structs::Rock::parse_from_rock_paths(&cave.rock_paths);
    let mut counter = HashSet::<structs::Coord>::new();
    let mut sand_particle = cave.sand;

    loop {

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

        let next_possible_moves = possible_moves
            .iter()
            .find(|p| !wall.is_touching(p) && !counter.contains(p));

        match next_possible_moves {
            Some(p) => sand_particle = *p,
            None => {
                counter.insert(sand_particle);

                if sand_particle == cave.sand {
                    break;
                }

                sand_particle = cave.sand;
            }
        }
    }

    println!("total: {}", counter.len());
}

fn get_wall_edges(paths: &Vec<structs::Path>) -> structs::WallEdges {
    let rock_paths_iter = paths.iter().flatten();

    let min_x = rock_paths_iter
        .clone()
        .min_by(|a, b| a.x.cmp(&b.x))
        .unwrap()
        .x;
    let max_x = rock_paths_iter
        .clone()
        .max_by(|a, b| a.x.cmp(&b.x))
        .unwrap()
        .x;

    let min_y = rock_paths_iter
        .clone()
        .min_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y;
    let max_y = rock_paths_iter
        .clone()
        .max_by(|a, b| a.y.cmp(&b.y))
        .unwrap()
        .y;

    structs::WallEdges {
        edge1: structs::Coord { x: min_x, y: min_y },
        edge2: structs::Coord { x: max_x, y: max_y },
    }
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

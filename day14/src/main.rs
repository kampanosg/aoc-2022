pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let cave = parse_cave(&file_contents);

    match part.as_str() {
        "p1" => p1(cave),
        _ => println!(""),
    }
}

fn p1(cave: Vec<structs::Path>) {
    
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

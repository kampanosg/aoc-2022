pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();    

    let cubes = parse_cubes(file_contents);

    match part.as_str() {
        "p1" => p1(cubes),
        _ => println!(""),
    }
}

fn p1(cubes: Vec<structs::Coord3D>) {

}

fn parse_cubes(file_contents: String) -> Vec<structs::Coord3D> {
    file_contents
        .trim_end_matches("\n")
        .lines()
        .map(|l| { 
            let coord = l.split(",").collect::<Vec<&str>>();
            let x = coord[0].parse::<u32>().unwrap();
            let y = coord[1].parse::<u32>().unwrap();
            let z = coord[2].parse::<u32>().unwrap();
            structs::Coord3D {x, y, z }
        }).collect::<Vec<structs::Coord3D>>()
}

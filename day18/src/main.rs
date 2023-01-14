pub mod structs;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let cubes = parse_cubes(file_contents);

    match part.as_str() {
        "p1" => p1(cubes),
        "p2" => p2(cubes),
        _ => println!(""),
    }
}

fn p1(cubes: Vec<structs::Coord3D>) {
    let total = cubes
        .iter()
        .flat_map(|c| c.adjacent_coords())
        .filter(|c| !cubes.contains(c))
        .count();
    println!("total = {}", total);
}

fn p2(cubes: Vec<structs::Coord3D>) {
    let cubes: HashSet<structs::Coord3D> = cubes.into_iter().collect();
    let visible = get_visible_cubes(&cubes);

    let total = cubes
        .iter()
        .flat_map(|coord| coord.adjacent_coords())
        .filter(|coord| visible.contains(coord))
        .count();
    println!("total = {}", total);
}

fn get_cube_bounds(cubes: &HashSet<structs::Coord3D>) -> [structs::Coord3D; 2] {
    cubes.iter().fold(
        [structs::Coord3D::default(), structs::Coord3D::default()],
        |[mut min, mut max], cube| {
            min.x = min.x.min(cube.x);
            min.y = min.y.min(cube.y);
            min.z = min.z.min(cube.z);
            max.x = max.x.max(cube.x);
            max.y = max.y.max(cube.y);
            max.z = max.z.max(cube.z);
            [min, max]
        },
    )
}

fn get_visible_cubes(cubes: &HashSet<structs::Coord3D>) -> HashSet<structs::Coord3D> {
    let bounds = get_cube_bounds(cubes);
    let mut visible = HashSet::new();

    let start = structs::Coord3D::default();
    let mut stack = Vec::new();
    let mut seen = HashSet::new();

    stack.push(start);
    seen.insert(start);

    while let Some(coord) = stack.pop() {
        for neighbour in coord.adjacent_coords() {
            if cubes.contains(&neighbour) || !neighbour.is_within_bounds(&bounds) {
                continue;
            }

            if seen.insert(neighbour) {
                stack.push(neighbour);
                visible.insert(neighbour);
            }
        }
    }

    visible
}

fn parse_cubes(file_contents: String) -> Vec<structs::Coord3D> {
    file_contents
        .trim_end_matches("\n")
        .lines()
        .map(|l| {
            let coord = l.split(",").collect::<Vec<&str>>();
            let x = coord[0].parse::<i32>().unwrap();
            let y = coord[1].parse::<i32>().unwrap();
            let z = coord[2].parse::<i32>().unwrap();
            structs::Coord3D { x, y, z }
        })
        .collect::<Vec<structs::Coord3D>>()
}

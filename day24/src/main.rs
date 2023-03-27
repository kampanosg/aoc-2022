pub mod structs;
use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env, fs, iter,
};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let map = parse_map(&file_contents);

    match part.as_str() {
        "p1" => p1(map),
        _ => println!(""),
    }
}

fn p1(mrc: (HashMap<structs::Coordinate, structs::Block>, usize, usize)) {
    let (map, rows, cols) = mrc;
    let walls: HashSet<structs::Coordinate> = map
        .iter()
        .filter(|(_, tile)| **tile == structs::Block::Wall)
        .map(|(pos, _)| *pos)
        .collect();

    let least_common_mult = calc_least_common_multiple(rows - 2, cols - 2);
    let wind_patterns = build_wind_patterns(&map, rows, cols, least_common_mult);
    let start_position = structs::Coordinate { y: 0, x: 1 };
    let end_position = structs::Coordinate {
        y: rows - 1,
        x: cols - 2,
    };

    let mut q = BinaryHeap::new();
    let mut seen = HashSet::new();

    q.push(structs::Node {
        cost: 0,
        pos: start_position,
    });
    seen.insert((start_position, 0));

    while let Some(structs::Node { cost, pos }) = q.pop() {
        if pos == end_position {
            println!("res={}", cost);
            return;
        }

        let new_cost = cost + 1;
        let winds = &wind_patterns[&(new_cost % least_common_mult)];

        let candidates = pos
            .adjacent_coords(rows, cols)
            .into_iter()
            .chain(iter::once(pos))
            .filter(|coord| !walls.contains(coord))
            .filter(|coord| !winds.contains(coord));

        for new_pos in candidates {
            if seen.insert((new_pos, new_cost)) {
                q.push(structs::Node {
                    cost: new_cost,
                    pos: new_pos,
                });
            }
        }
    }
}

fn build_wind_patterns(
    map: &HashMap<structs::Coordinate, structs::Block>,
    rows: usize,
    cols: usize,
    max_time: usize,
) -> HashMap<usize, HashSet<structs::Coordinate>> {
    let mut pattern = HashMap::new();

    let mut winds: Vec<(structs::Coordinate, structs::Compass)> = map
        .iter()
        .filter_map(|(pos, tile)| match tile {
            structs::Block::Wall => None,
            structs::Block::Wind(dir) => Some((*pos, *dir)),
        })
        .collect();

    let coords = winds.iter().map(|(coord, _)| *coord).collect();
    pattern.insert(0, coords);

    for t in 1..max_time {
        for (coord, dir) in winds.iter_mut() {
            *coord = coord.add_adjacent_coord(dir);
            match dir {
                structs::Compass::Left => {
                    if coord.x == 0 {
                        coord.x = cols - 2;
                    }
                }
                structs::Compass::Right => {
                    if coord.x == cols - 1 {
                        coord.x = 1;
                    }
                }
                structs::Compass::Up => {
                    if coord.y == 0 {
                        coord.y = rows - 2;
                    }
                }
                structs::Compass::Down => {
                    if coord.y == rows - 1 {
                        coord.y = 1;
                    }
                }
            }
        }
        let coords = winds.iter().map(|(coord, _)| *coord).collect();
        pattern.insert(t, coords);
    }
    pattern
}

fn calc_least_common_multiple(first: usize, second: usize) -> usize {
    first * second / cacl_greatest_common_divisor(first, second)
}

fn cacl_greatest_common_divisor(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        std::mem::swap(&mut max, &mut min);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn parse_map(input: &str) -> (HashMap<structs::Coordinate, structs::Block>, usize, usize) {
    let mut map = HashMap::new();

    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().chars().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let coord = structs::Coordinate { y, x };
            let block = match c {
                '#' => structs::Block::Wall,
                '^' => structs::Block::Wind(structs::Compass::Up),
                'v' => structs::Block::Wind(structs::Compass::Down),
                '<' => structs::Block::Wind(structs::Compass::Left),
                '>' => structs::Block::Wind(structs::Compass::Right),
                _ => panic!("invalid input"),
            };
            map.insert(coord, block);
        }
    }
    (map, rows, cols)
}

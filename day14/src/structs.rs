use std::{
    cmp,
    collections::{HashMap, HashSet},
};

pub type Path = Vec<Coord>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Cave {
    pub sand: Coord,
    pub rock_paths: Vec<Path>,
}

pub struct TraversalOpts {
    pub start: Coord,
    pub end: Coord,
    pub x: i32,
    pub y: i32,
}

pub struct CaveWall {
    pub min: Coord,
    pub max: Coord,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Range(i32, i32);

#[derive(Debug)]
pub struct Rock {
    pub ver: HashMap<i32, HashSet<Range>>,
    pub hor: HashMap<i32, HashSet<Range>>,
}

pub struct WallEdges {
    pub edge1: Coord,
    pub edge2: Coord,
}

impl Rock {
    pub fn is_touching(self: &Self, coord: &Coord) -> bool {
        let is_touching_h_wall = self.hor.get(&coord.y).map_or(false, |v| {
            v.iter()
                .any(|Range(sx, ex)| Rock::is_within(sx.clone(), coord.x, ex.clone()))
        });

        let is_touching_v_wall = self.ver.get(&coord.x).map_or(false, |v| {
            v.iter()
                .any(|Range(sy, ey)| Rock::is_within(sy.clone(), coord.y, ey.clone()))
        });

        is_touching_v_wall || is_touching_h_wall
    }

    fn is_within(a: i32, coord: i32, b: i32) -> bool {
        let min = cmp::min(a, b);
        let max = cmp::max(a, b);
        min <= coord && coord <= max
    }

    pub fn parse_from_rock_paths(rock_paths: &Vec<Path>) -> Self {
        let mut hor_wall = HashMap::<i32, HashSet<Range>>::new();
        let mut ver_wall = HashMap::<i32, HashSet<Range>>::new();

        for path in rock_paths {
            for i in 0..(path.len() - 1) {
                let curr_coord = path[i];
                let next_coord = path[i + 1];

                if curr_coord.x == next_coord.x {
                    let range = if curr_coord.y > next_coord.y {
                        Range(next_coord.y, curr_coord.y)
                    } else {
                        Range(curr_coord.y, next_coord.y)
                    };
                    ver_wall
                        .entry(curr_coord.x)
                        .and_modify(|rs| {
                            rs.insert(range.clone());
                        })
                        .or_insert(HashSet::from([range.clone()]));
                } else if curr_coord.y == next_coord.y {
                    let range = if curr_coord.x > curr_coord.y {
                        Range(next_coord.x, curr_coord.x)
                    } else {
                        Range(curr_coord.x, next_coord.x)
                    };
                    hor_wall
                        .entry(curr_coord.y)
                        .and_modify(|rs| {
                            rs.insert(range.clone());
                        })
                        .or_insert(HashSet::from([range.clone()]));
                } else {
                    panic!(
                        "Diagonal paths aren't supported {:?} -> {:?}",
                        curr_coord, next_coord
                    );
                }
            }
        }

        Rock {
            ver: ver_wall,
            hor: hor_wall,
        }
    }
}

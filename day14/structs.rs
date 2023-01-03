use collections::{HashSet, HashMap};


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
pub struct Range {
    pub start: i32,
    pub end: i32,
}

#[derive(Debug)]
struct Rock {
    ver: HashMap<i32, HashSet<Range>>, 
    hor: HashMap<i32, HashSet<Range>>,
}

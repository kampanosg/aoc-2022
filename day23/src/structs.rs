#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

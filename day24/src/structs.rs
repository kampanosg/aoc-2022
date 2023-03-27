#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Block {
    Wall,
    Wind(Compass),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Compass {
    Up,
    Right,
    Down,
    Left,
}

pub enum Move {
    Up(u8),
    Rotate(Turn),
}

pub enum Turn {
    L,
    R,
}

#[derive(PartialEq)]
pub enum Block {
    Available,
    Wall,
    Empty,
}

#[derive(Clone)]
pub struct Coordinate {
    pub row: i32,
    pub col: i32,
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn score(&self) -> usize {
        use Direction::*;
        match self {
            Up => 3,
            Right => 0,
            Down => 1,
            Left => 2,
        }
    }

    pub fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (Up, Turn::L) => Left,
            (Up, Turn::R) => Right,
            (Right, Turn::L) => Up,
            (Right, Turn::R) => Down,
            (Down, Turn::L) => Right,
            (Down, Turn::R) => Left,
            (Left, Turn::L) => Down,
            (Left, Turn::R) => Up,
        }
    }

    pub fn offset(&self) -> Coordinate {
        use Direction::*;
        match &self {
            Up => Coordinate { row: -1, col: 0 },
            Right => Coordinate { row: 0, col: 1 },
            Down => Coordinate { row: 1, col: 0 },
            Left => Coordinate { row: 0, col: -1 },
        }
    }
}

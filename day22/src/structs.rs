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
    L,
    R,
    U,
    D,
}

impl Direction {
    pub fn score(&self) -> usize {
        use Direction::*;
        match self {
            R => 0,
            D => 1,
            L => 2,
            U => 3,
        }
    }

    pub fn turn(self, turn: &Turn) -> Direction {
        use Direction::*;
        match (self, turn) {
            (U, Turn::L) => L,
            (U, Turn::R) => R,
            (R, Turn::L) => U,
            (R, Turn::R) => D,
            (D, Turn::L) => R,
            (D, Turn::R) => L,
            (L, Turn::L) => D,
            (L, Turn::R) => U,
        }
    }

    pub fn offset(&self) -> Coordinate {
        use Direction::*;
        match &self {
            U => Coordinate { row: -1, col: 0 },
            R => Coordinate { row: 0, col: 1 },
            D => Coordinate { row: 1, col: 0 },
            L => Coordinate { row: 0, col: -1 },
        }
    }
}

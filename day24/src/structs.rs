use std::cmp::Ordering;

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

#[derive(PartialEq, Eq)]
pub struct Node {
    pub cost: usize,
    pub pos: Coordinate,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinate {
    pub fn adjacent_coords(&self, rows: usize, cols: usize) -> Vec<Self> {
        use Compass::*;
        let mut adj = Vec::new();
        if self.y > 0 {
            adj.push(self.add_adjacent_coord(&Up));
        }
        if self.x < cols - 1 {
            adj.push(self.add_adjacent_coord(&Right));
        }
        if self.y < rows - 1 {
            adj.push(self.add_adjacent_coord(&Down));
        }
        if self.x > 0 {
            adj.push(self.add_adjacent_coord(&Left));
        }
        adj
    }

    pub fn add_adjacent_coord(&self, dir: &Compass) -> Self {
        use Compass::*;
        match dir {
            Up => Coordinate {
                y: self.y - 1,
                x: self.x,
            },
            Right => Coordinate {
                y: self.y,
                x: self.x + 1,
            },
            Down => Coordinate {
                y: self.y + 1,
                x: self.x,
            },
            Left => Coordinate {
                y: self.y,
                x: self.x - 1,
            },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

pub enum Compass {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}


impl Coordinate {
    pub fn adjacent_coord(&self) -> [Self; 8] {
        use Compass::*;
        let n = self.add_coord(&North);
        let ne = self.add_coord(&NorthEast);
        let e = self.add_coord(&East);
        let se = self.add_coord(&SouthEast);
        let s = self.add_coord(&South);
        let sw = self.add_coord(&SouthWest);
        let w = self.add_coord(&West);
        let nw = self.add_coord(&NorthWest);
        [n, ne, e, se, s, sw, w, nw]
    }

    pub fn add_coord(&self, dir: &Compass) -> Self {
        use Compass::*;
        match dir {
            North => Coordinate{
                y: self.y- 1,
                x: self.x,
            },
            NorthEast => Coordinate{
                y: self.y - 1,
                x: self.x + 1,
            },
            East => Coordinate{
                y: self.y,
                x: self.x + 1,
            },
            SouthEast => Coordinate{
                y: self.y + 1,
                x: self.x + 1,
            },
            South => Coordinate{
                y: self.y + 1,
                x: self.x,
            },
            SouthWest => Coordinate{
                y: self.y + 1,
                x: self.x - 1,
            },
            West => Coordinate{
                y: self.y,
                x: self.x - 1,
            },
            NorthWest => Coordinate{
                y: self.y - 1,
                x: self.x - 1,
            },
        }
    }
}


impl Compass {
    pub fn check(&self, neighbours: &[bool; 8]) -> bool {
        let [n, ne, e, se, s, sw, w, nw] = neighbours;
        match &self {
            Compass::North => !n && !ne && !nw,
            Compass::South => !s && !se && !sw,
            Compass::West => !w && !nw && !sw,
            Compass::East => !e && !ne && !se,
            _ => false,
        }
    }
}
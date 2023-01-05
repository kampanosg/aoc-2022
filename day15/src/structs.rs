#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub struct Sensor {
    pub position: Coord,
    pub beacon: Coord,
}

impl Sensor {
    pub fn new(sensor_x: i64, sensor_y: i64, beacon_x: i64, beacon_y: i64) -> Sensor {
        let position = Coord {
            x: sensor_x,
            y: sensor_y,
        };
        let beacon = Coord {
            x: beacon_x,
            y: beacon_y,
        };
        Sensor { position, beacon }
    }
}

impl Coord {
    // As shown in https://en.wikipedia.org/wiki/Taxicab_geometry
    pub fn manhattan_distance(self, other: Self) -> i64 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
    }
}

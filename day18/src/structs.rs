#[derive(Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Coord3D {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

enum ThreeDimension {
    X,
    Y,
    Z,
}

impl Coord3D {
    pub fn adjacent_coords(&self) -> Vec<Coord3D> {
        let mut adjacent = Vec::new();

        for dimension in [ThreeDimension::X, ThreeDimension::Y, ThreeDimension::Z] {
            for offset in [-1, 1] {
                let mut adjacent_coord = self.clone();
                match dimension {
                    ThreeDimension::X => adjacent_coord.x += offset,
                    ThreeDimension::Y => adjacent_coord.y += offset,
                    ThreeDimension::Z => adjacent_coord.z += offset,
                }
                adjacent.push(adjacent_coord);
            }
        }

        adjacent
    }
}

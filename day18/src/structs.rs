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

    pub fn is_within_bounds(&self, bounds: &[Self; 2]) -> bool {
        let [min, max] = bounds;
        self.x >= min.x - 1
            && self.x <= max.x + 1
            && self.y >= min.y - 1
            && self.y <= max.y + 1
            && self.z >= min.z - 1
            && self.z <= max.z + 1
    }
}

mod tests {
    use super::Coord3D;

    #[test]
    fn test_is_within_bounds() {
        let coord = Coord3D { x: 0, y: 0, z: 0 };
        assert!(
            coord.is_within_bounds(&[Coord3D { x: 1, y: 0, z: 0 }, Coord3D { x: 0, y: 1, z: 0 }])
        );
        assert!(
            coord.is_within_bounds(&[Coord3D { x: 0, y: 1, z: 0 }, Coord3D { x: 1, y: 0, z: 0 }])
        );
        assert!(
            coord.is_within_bounds(&[Coord3D { x: 0, y: 0, z: 0 }, Coord3D { x: 0, y: 0, z: 0 }])
        );
    }
}

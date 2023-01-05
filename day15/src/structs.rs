use itertools::Itertools;
use std::ops::RangeInclusive;

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

impl Coord {}

pub struct Helpers {
    pub sensors: Vec<Sensor>,
}

impl Helpers {
    pub fn build_sensor_ranges(&self, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
        let mut sensor_ranges = vec![];
        for sensor in &self.sensors {
            let radius = Helpers::manhattan_distance(sensor.position, sensor.beacon);
            let distance_y = (y - sensor.position.y).abs();
            if distance_y > radius {
                continue;
            }
            let actual_distance = radius - distance_y;
            let middle = sensor.position.x;
            let start = middle - actual_distance;
            let end = middle + actual_distance;
            let range = start..=end;
            sensor_ranges.push(range);
        }
        sensor_ranges.sort_by_key(|r| *r.start());

        sensor_ranges.into_iter().coalesce(|range1, range2| {
            if range2.start() - 1 <= *range1.end() {
                if range2.end() > range1.end() {
                    Ok(*range1.start()..=*range2.end())
                } else {
                    Ok(range1)
                }
            } else {
                Err((range1, range2))
            }
        })
    }

    fn marge_sensor_ranges(&self, y: i64, range_x: RangeInclusive<i64>) -> impl Iterator<Item = RangeInclusive<i64>> {
        self.build_sensor_ranges(y).filter_map(move |sensor_range| {
            let range = *sensor_range.start().max(range_x.start())..=*sensor_range.end().min(range_x.end());
            if range.start() > range.end() {
                None
            } else {
                Some(range)
            }
        })
    }

    pub fn find_beacon_position(&self, range_y: &RangeInclusive<i64>, x_range: &RangeInclusive<i64>) -> Option<Coord> {
        range_y.clone().find_map(|y| {
            self.marge_sensor_ranges(y, x_range.clone())
                .nth(1)
                .map(|range| Coord {
                    x: range.start() - 1,
                    y,
                })
        })
    }

    // As shown in https://en.wikipedia.org/wiki/Taxicab_geometry
    pub fn manhattan_distance(coord1: Coord, coord2: Coord) -> i64 {
        (coord1.x.abs_diff(coord2.x) + coord1.y.abs_diff(coord2.y)) as i64
    }
}

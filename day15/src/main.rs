pub mod structs;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let sensors = parse_sensors(file_contents);

    match part.as_str() {
        "p1" => p1(sensors),
        "p2" => p2(sensors),
        _ => println!(""),
    }
}

fn p1(sensors: Vec<structs::Sensor>) {
    let y_location_to_inspect = 2_000_000;
    let beacon_x_coords = sensors
        .iter()
        .filter(|sensor| sensor.beacon.y == y_location_to_inspect)
        .map(|sensor| sensor.beacon.x)
        .collect::<HashSet<_>>();

    let helpers = structs::Helpers { sensors };

    let unreachable_beacons = helpers
        .build_sensor_ranges(y_location_to_inspect)
        .map(|r| {
            let range_size = (r.end() - r.start() + 1) as usize;
            let num_beacons_in_range = beacon_x_coords.iter().filter(|x| r.contains(x)).count();
            range_size - num_beacons_in_range
        })
        .sum::<usize>();

    println!("unreachable beacons: {}", unreachable_beacons);
}

fn p2(sensors: Vec<structs::Sensor>) {
    let helpers = structs::Helpers { sensors };
    let range = 0..=20;
    let bp = helpers.beacon_position(&range, &range).unwrap();
    dbg!(bp);
    println!("tuning frequency: {}", bp.x * 4_000_000 + bp.y);
}

fn parse_sensors(file_contents: String) -> Vec<structs::Sensor> {
    let mut sensors: Vec<structs::Sensor> = vec![];
    for line in file_contents.lines() {
        let line = line.split(" ").collect::<Vec<&str>>();
        let sensor_x = line[2]
            .replace("x=", "")
            .replace(",", "")
            .parse::<i64>()
            .unwrap();
        let sensor_y = line[3]
            .replace("y=", "")
            .replace(":", "")
            .parse::<i64>()
            .unwrap();
        let beacon_x = line[8]
            .replace("x=", "")
            .replace(",", "")
            .parse::<i64>()
            .unwrap();
        let beacon_y = line[9].replace("y=", "").parse::<i64>().unwrap();

        sensors.push(structs::Sensor::new(sensor_x, sensor_y, beacon_x, beacon_y));
    }

    sensors
}

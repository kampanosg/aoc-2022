pub mod structs;
use std::{collections::HashSet, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let sensors = parse_sensors(file_contents);

    match part.as_str() {
        "p1" => p1(sensors),
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

    // let mut unreachable_beacons = 0;
    // let y_location_to_inspect = 2_000_000;

    // let min_x = i64::MIN;
    // let max_x = i64::MAX;

    // for x in min_x..=max_x {
    //     let point = structs::Coord { x, y: y_location_to_inspect };

    //     if sensors.iter().any(|sensor| sensor.beacon == point) {
    //         continue
    //     } else if sensors.iter().any(|sensor| {
    //         let radius = sensor.position.manhattan_distance(sensor.beacon);
    //         sensor.position.manhattan_distance(point) <= radius
    //     }) {
    //         unreachable_beacons += 1
    //     }
    // }

    // println!("total beacons: {}", unreachable_beacons)
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

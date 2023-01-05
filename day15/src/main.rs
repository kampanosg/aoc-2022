pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let sensors = parse_sensors(file_contents);
    dbg!(sensors);

    match part.as_str() {
        "p1" => p1(),
        _ => println!(""),
    }
}

fn p1() {
    
}

fn parse_sensors(file_contents: String) -> Vec<structs::Sensor> {
    let mut sensors: Vec<structs::Sensor> = vec![]; 
    for line in file_contents.lines() {
        let line = line.split(" ").collect::<Vec<&str>>();
        let sensor_x = line[2].replace("x=", "").replace(",", "").parse::<i64>().unwrap();
        let sensor_y = line[3].replace("y=", "").replace(":", "").parse::<i64>().unwrap();
        let beacon_x = line[8].replace("x=", "").replace(",", "").parse::<i64>().unwrap();
        let beacon_y = line[9].replace("y=", "").parse::<i64>().unwrap();

        println!("{}, {}, {}, {}", sensor_x, sensor_y, beacon_x, beacon_y);
        sensors.push(structs::Sensor::new(sensor_x, sensor_y, beacon_x, beacon_y));
    }

    sensors
}

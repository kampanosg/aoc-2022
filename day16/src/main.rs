pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let valves = parse_valves(file_contents);

    match part.as_str() {
        "p1" => p1(valves),
        _ => println!(""),
    }
}

fn p1(valves: Vec<structs::Valve>) {

}

fn parse_valves(file_contents: String) -> Vec<structs::Valve> {
    let mut valves: Vec<structs::Valve> = vec![];
    for line in file_contents.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let name = parts[1];
        let flow_rate = parts[4].replace("rate=", "").replace(";", "").parse::<u64>().unwrap();
        let next_valves = parts[9..].to_vec();
        let mut adjacent_valves: Vec<String> = vec![];
        for valve in next_valves {
            let s = String::from(valve);
            adjacent_valves.push(s);
        }
        valves.push(structs::Valve{ name: String::from(name), flow_rate, adjacent_valves});
    }
    valves
}

pub mod structs;
use std::{
    collections::{hash_map::Entry, HashMap},
    env, fs,
};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let valves = parse_valves(file_contents);
    let mapped_valves = map_valves(valves.clone());
    let connected_valves = connect_valves(structs::ValveId::new("AA"), mapped_valves.clone());

    for (valve_id, tunnel) in connected_valves {
        println!("We can get to {:?} using tunnel {:?}", valve_id, tunnel);
    }

    match part.as_str() {
        "p1" => p1(valves.clone()),
        _ => println!(""),
    }
}

fn p1(valves: Vec<structs::Valve>) {}

fn parse_valves(file_contents: String) -> Vec<structs::Valve> {
    let mut valves: Vec<structs::Valve> = vec![];
    for line in file_contents.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let name = parts[1];
        let flow_rate = parts[4]
            .replace("rate=", "")
            .replace(";", "")
            .parse::<u64>()
            .unwrap();
        let next_valves = parts[9..].to_vec();
        let mut adjacent_valves: Vec<structs::ValveId> = vec![];
        for valve in next_valves {
            adjacent_valves.push(structs::ValveId::new(valve));
        }
        valves.push(structs::Valve {
            name: structs::ValveId::new(name),
            flow_rate,
            adjacent_valves,
        });
    }
    valves
}

pub fn map_valves(valves: Vec<structs::Valve>) -> HashMap<structs::ValveId, structs::Valve> {
    let mut mv: HashMap<structs::ValveId, structs::Valve> = HashMap::new();
    for valve in valves {
        if mv.contains_key(&valve.name) {
            continue;
        }
        mv.insert(valve.name, valve);
    }
    mv
}

pub fn connect_valves(
    starting_valve: structs::ValveId,
    valves: HashMap<structs::ValveId, structs::Valve>,
) -> HashMap<structs::ValveId, structs::Tunnel> {

    let mut current_valve: HashMap<structs::ValveId, structs::Tunnel> = Default::default();
    current_valve.insert(starting_valve, vec![]);

    let mut connections = current_valve.clone();

    while !current_valve.is_empty() {
        let mut next: HashMap<structs::ValveId, structs::Tunnel> = Default::default();
        for (name, tunnel) in current_valve {
            for link in valves[&name].adjacent_valves.iter().copied() {
                if let Entry::Vacant(e) = connections.entry(link) {
                    let conn_path: structs::Tunnel = tunnel
                        .iter()
                        .copied()
                        .chain(std::iter::once((name, link)))
                        .collect();
                    e.insert(conn_path.clone());
                    next.insert(link, conn_path);
                }
            }
        }
        current_valve = next;
    }

    connections
}

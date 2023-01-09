pub mod structs;
use std::{env, fs};

use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

use itertools::Itertools;

#[derive(Debug)]
pub struct Valve<'a> {
    pub flow_rate: u32,
    pub adjacent_valves: HashSet<&'a str>,
}

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let valves = parse_valves(file_contents.as_str());
    let valve_distances = min_distances(&valves);

    let open_valves: HashSet<_> = valves
        .iter()
        .filter(|(_, valve)| valve.flow_rate> 0)
        .map(|(&name, _)| name)
        .collect();

    match part.as_str() {
        "p1" => p1(&valves, &valve_distances, &open_valves),
        "p2" => p2(&valves, &valve_distances, &open_valves),
        _ => println!(""),
    }
}

fn p1(valves: &HashMap<&str, Valve>, valve_distances: &HashMap<(&str, &str), u32>, open_valves: &HashSet<&str>) { 
    let mut total_relieved_valves = 0;
    let mut moves = VecDeque::new();
    let mut visited_valves = HashSet::new();

    moves.push_back(structs::State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    visited_valves.insert((BTreeSet::new(), 0, 0));

    while let Some(structs::State {opened, curr, elapsed, relieved}) = moves.pop_front()
    {
        if opened.len() == open_valves.len() || elapsed >= 30 {
            let relieved_at_end = wait_for_boom(30, elapsed, relieved, &opened, &valves);
            total_relieved_valves = total_relieved_valves.max(relieved_at_end);
            continue;
        }

        let closed_valves = open_valves.iter().filter(|name| !opened.contains(*name));

        for next_valve in closed_valves {
            let cost = valve_distances[&(curr, *next_valve)] + 1;
            let new_elapsed = elapsed + cost;

            if new_elapsed >= 30 {
                let relieved_at_end = wait_for_boom(30, elapsed, relieved, &opened, &valves);
                total_relieved_valves = total_relieved_valves.max(relieved_at_end);
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| &valves[name].flow_rate).sum();
            let new_relieved = relieved + (relieved_per_min * cost);

            let mut new_opened = opened.clone();
            new_opened.insert(next_valve);

            if visited_valves.insert((new_opened.clone(), new_elapsed, new_relieved)) {
                moves.push_back(structs::State {
                    opened: new_opened,
                    curr: next_valve,
                    elapsed: new_elapsed,
                    relieved: new_relieved,
                });
            }
        }
    }

    println!("total: {}", total_relieved_valves);
}

pub fn p2(valves: &HashMap<&str, Valve>, valve_distances: &HashMap<(&str, &str), u32>, open_valves: &HashSet<&str>) { 
    let mut total_opened_valves: HashMap<BTreeSet<&str>, u32> = HashMap::new();

    let mut moves = VecDeque::new();
    moves.push_back(structs::State {
        curr: "AA",
        opened: BTreeSet::new(),
        elapsed: 0,
        relieved: 0,
    });

    while let Some(structs::State {opened, curr, elapsed, relieved, }) = moves.pop_front() {
        let relieved_at_end = wait_for_boom(26, elapsed, relieved, &opened, &valves);

        total_opened_valves
            .entry(opened.clone())
            .and_modify(|val| *val = relieved_at_end.max(*val))
            .or_insert(relieved_at_end);

        if opened.len() == open_valves.len() || elapsed >= 26 {
            continue;
        }

        let closed_valves = open_valves.iter().filter(|name| !opened.contains(*name));

        for next_valve in closed_valves {
            let cost = valve_distances[&(curr, *next_valve)] + 1;
            let new_elapsed = elapsed + cost;
            if new_elapsed >= 26 {
                continue;
            }

            let relieved_per_min: u32 = opened.iter().map(|name| &valves[name].flow_rate).sum();
            let new_relieved = relieved + (relieved_per_min * cost);

            let mut new_opened = opened.clone();
            new_opened.insert(next_valve);

            moves.push_back(structs::State {
                opened: new_opened,
                curr: next_valve,
                elapsed: new_elapsed,
                relieved: new_relieved,
            });
        }
    }

    let total_opened_valves = total_opened_valves
        .iter()
        .tuple_combinations()
        .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
        .map(|(human, elephant)| human.1 + elephant.1)
        .max()
        .unwrap();

    println!("total: {}", total_opened_valves);
}

// Return min cost to go from one valve to the orher, using Dijkstra's algorithm
fn calc_min_cost(from: &str, to: &str, map: &HashMap<&str, Valve>) -> u32 {
    let mut priority_queue = BinaryHeap::new();
    let mut seen = HashSet::new();

    priority_queue.push(structs::Node { cost: 0, curr: from, });
    seen.insert(from);

    while let Some(structs::Node { cost, curr }) = priority_queue.pop() {
        if curr == to {
            return cost;
        }

        for neighbour in map[curr].adjacent_valves.iter() {
            if seen.insert(neighbour) {
                priority_queue.push(structs::Node {
                    cost: cost + 1,
                    curr: neighbour,
                });
            }
        }
    }

    u32::MAX
}

fn min_distances<'a>(map: &'a HashMap<&str, Valve>) -> HashMap<(&'a str, &'a str), u32> {
    map.iter()
        .filter(|(_, valve)| valve.flow_rate > 0)
        .map(|(&name, _)| name)
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (name1, name2)| {
            acc.entry(("AA", name1))
                .or_insert_with(|| calc_min_cost("AA", name1, map));
            acc.entry(("AA", name2))
                .or_insert_with(|| calc_min_cost("AA", name2, map));

            let dist = calc_min_cost(name1, name2, map);
            acc.insert((name1, name2), dist);
            acc.insert((name2, name1), dist);

            acc
        })
}

fn wait_for_boom(max_time: u32, elapsed: u32, relieved: u32, opened: &BTreeSet<&str>, map: &HashMap<&str, Valve>) -> u32 {
    let time_left = max_time - elapsed;
    let relieved_per_min: u32 = opened.iter().map(|name| &map[name].flow_rate).sum();
    relieved + (relieved_per_min * time_left)
}

fn parse_valves(file_contents: &str) -> HashMap<&str, Valve> {
    let mut valves: HashMap<&str, Valve> = HashMap::new();
    for line in file_contents.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        let name = parts[1];
        let flow_rate = parts[4]
            .replace("rate=", "")
            .replace(";", "")
            .parse::<u32>()
            .unwrap();

        let adjacent_valves = parts[9..]
            .to_vec()
            .iter()
            .map(|f| f.trim_end_matches(","))
            .collect::<HashSet<&str>>();

        valves.insert(
            name,
            Valve {
                flow_rate,
                adjacent_valves,
            },
        );
    }
    valves
}

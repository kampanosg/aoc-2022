pub mod structs;
use std::collections::VecDeque;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let blueprints = parse_blueprints(file_contents);

    match part.as_str() {
        "p1" => p1(blueprints),
        "p2" => p2(blueprints),
        _ => println!(""),
    }
}

fn p1(blueprints: Vec<[[u16; 4]; 4]>) {
    let max_time = 24;
    let res = blueprints
        .iter()
        .map(|blueprint| max_geodes(blueprint, max_time))
        .enumerate()
        .map(|(idx, geodes)| (idx + 1) * geodes as usize)
        .sum::<usize>();
    println!("result = {}", res);
}

fn p2(blueprints: Vec<[[u16; 4]; 4]>) {
    let max_time = 32;
    let res = blueprints
        .iter()
        .take(3)
        .map(|blueprint| usize::from(max_geodes(blueprint, max_time)))
        .product::<usize>();
    println!("result = {}", res);
}

fn parse_blueprints(file_contents: String) -> Vec<[[u16; 4]; 4]> {
    let mut blueprints = Vec::new();

    for line in file_contents.lines() {
        let mut iter = line.split_ascii_whitespace();

        let ore_bot_costs = [iter.nth(6).unwrap().parse().unwrap(), 0, 0, 0];
        let clay_bot_costs = [iter.nth(5).unwrap().parse().unwrap(), 0, 0, 0];
        let obsidian_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            iter.nth(2).unwrap().parse().unwrap(),
            0,
            0,
        ];

        let geode_bot_costs = [
            iter.nth(5).unwrap().parse().unwrap(),
            0,
            iter.nth(2).unwrap().parse().unwrap(),
            0,
        ];

        let blueprint = [
            ore_bot_costs,
            clay_bot_costs,
            obsidian_bot_costs,
            geode_bot_costs,
        ];
        blueprints.push(blueprint);
    }
    blueprints
}

fn max_geodes(blueprint: &[[u16; 4]; 4], max_time: u16) -> u16 {
    let mut max_robots = [u16::MAX; 4];
    for i in 0..3 {
        max_robots[i] = blueprint
            .iter()
            .map(|cost| cost[i])
            .max()
            .unwrap();
    }

    let mut max_geodes = 0;

    let mut q = VecDeque::new();
    q.push_back(structs::State {
        inventory: [0, 0, 0, 0],
        bots: [1, 0, 0, 0],
        elapsed: 0,
    });

    while let Some(structs::State {
        inventory,
        bots,
        elapsed,
    }) = q.pop_front()
    {
        for i in 0..blueprint.len() {
            if bots[i] == max_robots[i] {
                continue;
            }
            let costs = &blueprint[i];
            let wait_time = (0..3)
                .map(|idx| {
                    match costs[idx] {
                        cost if cost <= inventory[idx] => 0,
                        _ if bots[idx] == 0 => max_time + 1,
                        _ => (costs[idx] - inventory[idx] + bots[idx] - 1) / bots[idx],
                    }
                })
                .max()
                .unwrap();

            let new_elapsed = elapsed + wait_time + 1;
            if new_elapsed >= max_time {
                continue;
            }

            let mut new_inventory = [0; 4];
            for idx in 0..bots.len() {
                new_inventory[idx] = inventory[idx] + bots[idx] * (wait_time + 1) - costs[idx];
            }

            let mut new_bots = bots;
            new_bots[i] += 1;

            let remaining_time = max_time - new_elapsed;
            if ((remaining_time - 1) * remaining_time) / 2
                + new_inventory[3]
                + remaining_time * new_bots[3]
                < max_geodes
            {
                continue;
            }

            q.push_back(structs::State {
                inventory: new_inventory,
                bots: new_bots,
                elapsed: new_elapsed,
            })
        }

        let geodes = inventory[3] + bots[3] * (max_time - elapsed);
        max_geodes = geodes.max(max_geodes);
    }

    max_geodes
}

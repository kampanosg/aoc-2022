use std::{collections::VecDeque, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let bays_instructions = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(bays_instructions),
        "p2" => p2(bays_instructions),
        _ => println!(""),
    }
}

fn p1(bays_instructions: Vec<&str>) {
    let mut filled_bays: Vec<VecDeque<char>> = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for b in &bays_instructions[0..8] {
        let mut index = 0;
        let mut bay_index = 0;
        while index < b.len() - 1 {
            let container = b.chars().nth(index + 1).unwrap();
            match container {
                ' ' => (),
                c => filled_bays[bay_index].push_front(c),
            };
            index += 4;
            bay_index += 1
        }
    }

    for instruction in &bays_instructions[10..] {
        println!("{:?}", instruction);
        let parsed_instruction = instruction
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let moves = parsed_instruction[1].parse::<u64>().unwrap();
        let origin = parsed_instruction[3].parse::<usize>().unwrap();
        let destination = parsed_instruction[5].parse::<usize>().unwrap();

        for _ in 0..moves {
            let container = filled_bays[origin - 1].pop_back();
            filled_bays[destination - 1].push_back(container.unwrap());
        }
    }

    for b in filled_bays {
        println!("{:?}", b);
    }
}

fn p2(bays_instructions: Vec<&str>) {
    let mut filled_bays: Vec<VecDeque<char>> = vec![
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
        VecDeque::new(),
    ];

    for b in &bays_instructions[0..8] {
        let mut index = 0;
        let mut bay_index = 0;
        while index < b.len() - 1 {
            let container = b.chars().nth(index + 1).unwrap();
            match container {
                ' ' => (),
                c => filled_bays[bay_index].push_front(c),
            };
            index += 4;
            bay_index += 1
        }
    }

    for instruction in &bays_instructions[10..] {
        let parsed_instruction = instruction
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();

        let moves = parsed_instruction[1].parse::<u64>().unwrap();
        let origin = parsed_instruction[3].parse::<usize>().unwrap();
        let destination = parsed_instruction[5].parse::<usize>().unwrap();

        let mut containers_to_move = VecDeque::new();
        for _ in 0..moves {
            containers_to_move.push_front(filled_bays[origin-1].pop_back());
        }
        for c in containers_to_move {
            filled_bays[destination-1].push_back(c.unwrap());
        }
    }

    for b in filled_bays {
        println!("{:?}", b);
    }
}

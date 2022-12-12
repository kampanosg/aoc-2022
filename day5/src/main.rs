use std::{env, fs, collections::VecDeque};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let bays_instructions = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    let bays = parse_bays(&bays_instructions[0..8]);
    
}

fn parse_bays(bays: &[&str]) -> Vec<VecDeque<char>> {
    let mut filled_bays = vec![
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

    for b in bays {
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
    filled_bays
}

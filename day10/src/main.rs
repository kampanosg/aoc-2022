use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let instructions = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(instructions),
        "p2" => p2(instructions),
        _ => println!(""),
    }
}

fn p1(commands: Vec<&str>) {
    let mut register_x: i64 = 1;
    let mut clock: i64 = 0;
    let mut index = 0;
    let mut signal_strength = 0;

    while index < commands.len() {
        let command = commands[index].split(" ").collect::<Vec<&str>>();
        let instruction = command[0];

        let cycles = match instruction {
            "addx" => 2,
            "noop" => 1,
            _ => panic!("huh?"),
        };

        for _ in 0..cycles {
            clock += 1;
            match clock {
                20 | 60 | 100 | 140 | 180 | 220 => signal_strength += register_x * clock,
                _ => {}
            }
        }

        if instruction == "addx" {
            let val = command[1].parse::<i64>().unwrap();
            register_x += val;
        }

        index += 1;
    }

    println!("singal strength = {}", signal_strength);
}

fn p2(commands: Vec<&str>) {
    let mut register_x = 1;
    let mut sprite = (0, 0, 0);

    let mut command_index = 0;
    let mut command = commands[command_index].split(" ").collect::<Vec<&str>>();
    let mut instruction = command[0];
    let mut cycles = get_instr_cycles(instruction); 
        
    for y in 0..6 {
        for x in 0..39 {
            
            cycles -= 1;

            if cycles == 0 {
                 if instruction == "addx" {
                     let val = command[1].parse::<i64>().unwrap();
                     register_x += val;
                     // TODO: see where beam is
                     println!("y = {}, x = {}, register_x = {}", y, x, register_x);
                     return;
                 }
                 command_index += 1;
                 command = commands[command_index].split(" ").collect::<Vec<&str>>();
                 instruction = command[0];
                 cycles = get_instr_cycles(instruction);
            }
        }
    }

}

fn get_instr_cycles(instruction: &str) -> i64 {
    match instruction {
        "addx" => 2,
        "noop" => 1,
        _ => panic!("huh?"),
    }
}

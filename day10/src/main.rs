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
        _ => println!(""),
    }
}

fn p1(commands: Vec<&str>) {
    let mut register_x: i64 = 1;
    let mut clock: u64 = 1;

    for command in commands {
        let command = command.split(" ").collect::<Vec<&str>>();
        let instruction = command[0];

        match instruction {
            "addx" => {
                let val = command[1].parse::<i64>().unwrap();
                register_x += val;
                clock += 2;
            }
            "noop" => clock += 1,
            _ => panic!("huh"),
        }
    }

    println!("clock = {}", clock);
    println!("register_x = {}", register_x);
}

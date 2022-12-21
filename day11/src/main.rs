use std::{env, fs};

fn main() {

    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let monkeys = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(monkeys),
        _ => println!(""),
    }

}

fn p1(monkeys: Vec<&str>) {
    
    println!("{:?}", monkeys);

}

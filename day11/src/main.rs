pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let monkeys = file_contents.split("\n\n").collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(monkeys),
        _ => println!(""),
    }
}

fn p1(monkeys: Vec<&str>) {
    let mut monkes: Vec<structs::Monke> = Vec::new();

    for monkey in monkeys {
        let monkey_array = monkey.split("\n").collect::<Vec<&str>>();
        let monkey_items = monkey_array[1].trim().replace("Starting items: ", "");
        
        let items = monkey_items
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|e| e.trim().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();

        let op = monkey_array[2].trim().replace("Operation: new = ", "");
        let divident = monkey_array[3].trim().replace("Test: divisible by ", "").parse::<u64>().unwrap();
        let dest_t = monkey_array[4].trim().replace("If true: throw to monkey ", "").parse::<u64>().unwrap();
        let dest_f = monkey_array[5].trim().replace("If false: throw to monkey ", "").parse::<u64>().unwrap();
        let monke = structs::Monke{ items, divident, dest_f, dest_t, op }; 

        monkes.push(monke);
    }

    println!("{:?}", monkes);
}

use std::fs;
use std::env;

fn main() {
    let file_path = env::args().nth(1)
        .expect("param not provided");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let elf_cals: Vec<&str> = file_contents.split("\n\n").collect();

    let mut highest_cals = 0;

    for elf in elf_cals {
        let elf = elf.trim_end_matches("\n");
        let elf_cals = elf
            .split("\n")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();

       if elf_cals > highest_cals {
            highest_cals = elf_cals
       }
    }
    println!("highest cals: {}", highest_cals);
}


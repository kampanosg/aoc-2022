use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: part (valid options: p1 | p2)");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let elf_cals: Vec<&str> = file_contents.split("\n\n").collect();

    match part.as_str() {
        "p1" => p1(elf_cals),
        "p2" => p2(elf_cals),
        _ => println!("invalid part param (valid options: p1 | p2)")
    }
}

fn p1(elf_cals: Vec<&str>) {
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

fn p2(elf_cals: Vec<&str>) {
    let mut all_cals: Vec<i64> = vec![];
    for elf in elf_cals {
        let elf = elf.trim_end_matches("\n");
        let elf_cals = elf
            .split("\n")
            .map(|e| e.parse().unwrap())
            .collect::<Vec<i64>>()
            .iter()
            .sum::<i64>();
        all_cals.push(elf_cals);
    }
    all_cals.sort();
    all_cals.reverse();
    let (top_elves, _other_elves) = all_cals.split_at(3);
    let total_cals = top_elves.to_vec().iter().sum::<i64>();
    println!("top elves: {:?}", top_elves);
    println!("top cals (total): {}", total_cals);

}

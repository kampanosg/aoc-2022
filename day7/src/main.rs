pub mod structs;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args()
        .nth(2)
        .expect("param not provided: part (valid options: p1 | p2)");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let commands = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(commands),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(commands: Vec<&str>) {
    let mut current_dir = structs::Directory::new(String::from("/"));

    for c in &commands[1..] {
        let command = c.split(" ").collect::<Vec<&str>>();

        println!("{:?}", command);
        match command[0] {
            "$" => {
                if command[1] == "cd" {
                    if command[1] == ".." {
                    } else {
                    }
                }
            }
            "dir" => {
                let dir_name = command[1];
                current_dir.append_dir(dir_name);
            }
            _ => {}
        }
    }

    println!("{:?}", current_dir);
}

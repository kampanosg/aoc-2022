use std::fs;
use std::env;

fn main() {
    let file_path = env::args().nth(1)
        .expect("param not provided");

    let contents = fs::read_to_string(file_path)
        .expect("unable to open file provided");

    let split = contents.split("\n");
    let vec: Vec<&str> = split.collect();

    let mut max = 0;
    let mut current = 0;

    for num in vec {
        match num {
            "" => {
                if current > max {
                    max = current;
                }
                current = 0;
            },
            _ => {
                let calories: i32 = num.parse().unwrap();
                current += calories;
            }
        }
    }
    println!("max: {}", max);
}

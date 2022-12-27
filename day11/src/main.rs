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
        let divident = monkey_array[3]
            .trim()
            .replace("Test: divisible by ", "")
            .parse::<i64>()
            .unwrap();
        let dest_t = monkey_array[4]
            .trim()
            .replace("If true: throw to monkey ", "")
            .parse::<i64>()
            .unwrap();
        let dest_f = monkey_array[5]
            .trim()
            .replace("If false: throw to monkey ", "")
            .parse::<i64>()
            .unwrap();
        let monke = structs::Monke {
            items,
            divident,
            dest_f,
            dest_t,
            op,
        };

        monkes.push(monke);
    }

    for _ in 0..1 {
        for monke_number in 0..monkes.len() {
            println!("Monke {}:", monke_number);
            let monke = monkes[monke_number].clone();
            for loop_index in 0..monke.items.len() {
                let item = monke.items[loop_index];
                print!("item = {}, ", item);
                let worry_before_inspection =
                    calc_worry_lvl(item.clone(), item.clone(), monke.op.clone());
                let worry_after_inspection = worry_before_inspection / 3;
                let is_divisible = (worry_after_inspection % monke.divident) == 0;
                let next_monke_index: usize;

                if is_divisible {
                    println!(
                        "give = {:?}, to monke = {:?}",
                        worry_after_inspection, monke.dest_t
                    );
                    next_monke_index = usize::try_from(monke.dest_t).unwrap();
                } else {
                    println!(
                        "give = {:?}, to monke = {:?}",
                        worry_after_inspection, monke.dest_f
                    );
                    next_monke_index = usize::try_from(monke.dest_f).unwrap();
                }

                monkes[monke_number].items.remove(0);
                monkes[next_monke_index].items.push(worry_after_inspection);
            }

            println!();
            println!();
        }
    }

    for (index, monke) in monkes.iter().enumerate() {
        print!("monke {}: ", index);
        for item in monke.items.clone() {
            print!("{} ", item);
        }
        println!();
    }
}

fn calc_worry_lvl(worry: i64, old_worry: i64, op: String) -> i64 {
    match op.as_str() {
        "old * 19" => worry * 19,
        "old + 6" => worry + 6,
        "old * old" => old_worry * old_worry,
        "old + 3" => old_worry + 3,
        &_ => 0,
    }
}

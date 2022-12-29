pub mod structs;
use std::{collections::HashMap, env, fs};

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
    let mut monke_biz: HashMap<i64, i64> = HashMap::new();

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

    for _ in 0..20 {
        for monke_number in 0..monkes.len() {
            // println!("Monke {}:", monke_number);
            let monke = monkes[monke_number].clone();
            for loop_index in 0..monke.items.len() {
                let item = monke.items[loop_index];
                // print!("item = {}, ", item);
                let worry_before_inspection = calc_worry_lvl(item.clone(), monke.op.clone());
                let worry_after_inspection = worry_before_inspection / 3;
                let next_monke_index: usize;

                if worry_after_inspection % monke.divident == 0 {
                    next_monke_index = usize::try_from(monke.dest_t).unwrap();
                } else {
                    next_monke_index = usize::try_from(monke.dest_f).unwrap();
                }

                // println!(
                //     "give = {:?}, to monke = {:?}",
                //     worry_after_inspection, next_monke_index
                // );

                monkes[next_monke_index].items.push(worry_after_inspection);

                let monke_number = i64::try_from(monke_number).unwrap();

                if monke_biz.contains_key(&monke_number) {
                    let existing_monke_biz = monke_biz.get(&monke_number).unwrap();
                    monke_biz.insert(monke_number, existing_monke_biz + 1);
                    continue;
                }
                monke_biz.insert(monke_number, 1);
            }

            monkes[monke_number].items.clear();

            // println!();
            // println!();
        }
    }

    //     print!("monke {}: ", index);
    //     for item in monke.items.clone() {
    //         print!("{} ", item);
    //     }
    // println!();
    // }

    // println!();
    // println!();

    //     for (_monke, biz) in monke_biz.iter().enumerate() {
    //         println!("monke {}, total biz: {:?}", biz.0, biz.1);
    //     }

    let mut l: Vec<_> = monke_biz.iter().map(|e| e.1).collect();
    l.sort();
    l.reverse();

    //     println!();
    //     println!("biz list = {:?}", l);

    let total_monke_biz = l[0] * l[1];
    println!();
    println!("total monke biz = {}", total_monke_biz);
}

fn calc_worry_lvl(old_worry: i64, op: String) -> i64 {
    match op.as_str() {
        "old * 19" => old_worry * 19,
        "old + 6" => old_worry + 6,
        "old * old" => old_worry * old_worry,
        "old + 3" => old_worry + 3,
        "old * 13" => old_worry * 13,
        "old + 7" => old_worry + 7,
        "old + 5" => old_worry + 5,
        "old + 8" => old_worry + 8,
        "old * 5" => old_worry * 5,
        "old + 2" => old_worry + 2,
        &_ => 0,
    }
}

use std::{env, fs, collections::HashMap};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let monkes = parse_monkes(&file_contents);

    match part.as_str() {
        "p1" => p1(monkes),
        "p2" => p2(monkes),
        _ => println!(""),
    }
}

fn p1(monkes: HashMap<&str, Monke>) {
    let res = traverse_the_monke_tree("root", &monkes);
    println!("res: {}", res);
}

fn p2(monkes: HashMap<&str, Monke>) {
    let Monke::Mathematical(_, lhs, rhs) = &monkes["root"] else {
        panic!("root has to be a calculated monke");
    };

    let (name, value) = if should_depend_on_hooman(lhs, &monkes) {
        let rhs_num = traverse_the_monke_tree(rhs, &monkes);
        (lhs, rhs_num)
    } else {
        let lhs_num = traverse_the_monke_tree(lhs, &monkes);
        (rhs, lhs_num)
    };

    let res = hooman_calc(name, value, &monkes);
    println!("res: {}", res)
}

fn parse_monkes(input: &str) -> HashMap<&str, Monke> {
    input
        .lines()
        .map(|line| {
            let (name, right) = line.split_once(": ").unwrap();
            let monke = match right.parse() {
                Ok(n) => Monke::Yelling(n),
                Err(_) => {
                    let mut iter = right.split_ascii_whitespace();
                    let lhs = iter.next().unwrap();
                    let operator = match iter.next().unwrap() {
                        "+" => MathOperation::Addition,
                        "-" => MathOperation::Substitution,
                        "*" => MathOperation::Multiplication,
                        "/" => MathOperation::Division,
                        _ => panic!("Invalid math operator"),
                    };
                    let rhs = iter.next().unwrap();
                    Monke::Mathematical(operator, lhs, rhs)
                }
            };

            (name, monke)
        })
        .collect()
}

fn traverse_the_monke_tree(name: &str, monkes: &HashMap<&str, Monke>) -> i64 {
    match &monkes[name] {
        Monke::Yelling(n) => *n,
        Monke::Mathematical(operator, lhs, rhs) => {
            let lhs_num = traverse_the_monke_tree(lhs, monkes);
            let rhs_num = traverse_the_monke_tree(rhs, monkes);
            match operator {
                MathOperation::Addition => lhs_num + rhs_num,
                MathOperation::Substitution => lhs_num - rhs_num,
                MathOperation::Multiplication => lhs_num * rhs_num,
                MathOperation::Division => lhs_num / rhs_num,
            }
        }
    }
}

fn should_depend_on_hooman(name: &str, monkes: &HashMap<&str, Monke>) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkes[name] {
        Monke::Yelling(_) => false,
        Monke::Mathematical(_, lhs, rhs) => {
            should_depend_on_hooman(lhs, monkes) || should_depend_on_hooman(rhs, monkes)
        }
    }
}

fn hooman_calc(name: &str, value: i64, monkes: &HashMap<&str, Monke>) -> i64 {
    if name == "humn" {
        return value;
    }

    match &monkes[name] {
        Monke::Yelling(n) => *n,
        Monke::Mathematical(operator, lhs, rhs) => {
            let (new_name, new_value) = if should_depend_on_hooman(lhs, monkes) {
                let rhs_num = traverse_the_monke_tree(rhs, monkes);
                let new_value = match operator {
                    MathOperation::Addition => value - rhs_num,
                    MathOperation::Substitution => value + rhs_num,
                    MathOperation::Multiplication => value / rhs_num,
                    MathOperation::Division => value * rhs_num,
                };
                (lhs, new_value)
            } else {
                let lhs_num = traverse_the_monke_tree(lhs, monkes);
                let new_value = match operator {
                    MathOperation::Addition => value - lhs_num,
                    MathOperation::Substitution => lhs_num - value,
                    MathOperation::Multiplication => value / lhs_num,
                    MathOperation::Division => lhs_num / value,
                };
                (rhs, new_value)
            };
            hooman_calc(new_name, new_value, monkes)
        }
    }
}

enum Monke<'a> {
    Yelling(i64),
    Mathematical(MathOperation, &'a str, &'a str),
}

#[derive(Debug)]
enum MathOperation {
    Addition,
    Substitution,
    Multiplication,
    Division,
}
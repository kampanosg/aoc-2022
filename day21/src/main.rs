use std::{env, fs, collections::HashMap};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let monkes = parse_monkes(&file_contents);

    match part.as_str() {
        "p1" => p1(monkes),
        _ => println!(""),
    }
}

fn p1(monkes: HashMap<&str, Monke>) {
    let res = traverse_the_monke_tree("root", &monkes);
    println!("res: {}", res);
}

fn parse_monkes(input: &str) -> HashMap<&str, Monke> {
    input
        .lines()
        .map(|line| {
            let (name, right) = line.split_once(": ").unwrap();
            let monkey = match right.parse() {
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

            (name, monkey)
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
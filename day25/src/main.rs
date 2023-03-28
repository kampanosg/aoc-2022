use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();

    match part.as_str() {
        "p1" => p1(&file_contents),
        _ => println!(""),
    }
}

fn p1(input: &str) {
    let sum = input.lines().map(from_str_to_decimal).sum();
    let res = to_code(sum);
    println!("res = {}", res);
}

fn from_str_to_decimal(input: &str) -> i64 {
    input.chars().fold(0, |decimal, snafu_digit| {
        let decimal_digit = ['=', '-', '0', '1', '2']
            .into_iter()
            .position(|c| c == snafu_digit)
            .unwrap() as i64
            - 2;
        decimal * 5 + decimal_digit
    })
}

fn to_code(d: i64) -> String {
    if d == 0 {
        return String::new();
    }

    let next_decimal = (d + 2) / 5;
    let mut code = to_code(next_decimal);

    let rem = d % 5;
    let code_char = ['0', '1', '2', '=', '-'][rem as usize];
    code.push(code_char);
    code
}

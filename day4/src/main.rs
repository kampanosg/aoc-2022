use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let assignments = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(assignments),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(assignments: Vec<&str>) {
    for assignment in assignments {
        println!("{}", assignment);
        let pair = assignment.split(",").collect::<Vec<&str>>();
        println!("{:?}", pair);
        let range1 = gen_range(pair[0]);
        let range2 = gen_range(pair[1]);
        println!("{:?} {:?}", range1, range2);
        break;
    }
}

fn gen_range(pair: &str) -> Vec<u64> {
    let parsed_pair = parse_pair(pair);
    (parsed_pair[0]..parsed_pair[1] + 1).collect::<Vec<u64>>()
}

fn parse_pair(pair: &str) -> Vec<u64> {
    pair.split("-")
        .collect::<Vec<&str>>()
        .iter()
        .map(|e| e.parse().unwrap())
        .collect::<Vec<u64>>()
}

#[cfg(test)]
mod tests {
    use crate::{parse_pair, gen_range};

    #[test]
    fn test_parse_pair() {
        assert_eq!(parse_pair("1-1"), vec![1, 1]);
        assert_eq!(parse_pair("1-2"), vec![1, 2]);
        assert_eq!(parse_pair("1-69"), vec![1, 69]);
    }

    #[test]
    fn test_gen_range() {
        assert_eq!(gen_range("1-1"), vec![1]);
        assert_eq!(gen_range("1-2"), vec![1, 2]);
        assert_eq!(gen_range("1-10"), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}

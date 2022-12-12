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
        "p2" => p2(assignments),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(assignments: Vec<&str>) {
    let mut total = 0;
    for assignment in assignments {
        let pair = assignment.split(",").collect::<Vec<&str>>();
        let range1 = gen_range(pair[0]);
        let range2 = gen_range(pair[1]);
        if is_exactly_contained(range1, range2) {
            total += 1;
        }
    }
    println!("{}", total);
}


fn p2(assignments: Vec<&str>) {
    let mut total = 0;
    for assignment in assignments {
        let pair = assignment.split(",").collect::<Vec<&str>>();
        let range1 = gen_range(pair[0]);
        let range2 = gen_range(pair[1]);
        if is_overlapping(range1, range2) {
            total += 1;
        }
    }
    println!("{}", total);
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

fn is_exactly_contained(range1: Vec<u64>, range2: Vec<u64>) -> bool {
    if range1.len() > range2.len() {
        return is_contained(range2, range1);
    }
    is_contained(range1, range2)
}

fn is_contained(smaller: Vec<u64>, larger: Vec<u64>) -> bool {
    let (sm_fst, sm_lst) = (smaller[0], smaller[smaller.len() - 1]);
    let (lg_fst, lg_lst) = (larger[0], larger[larger.len() - 1]);
    sm_fst >= lg_fst && sm_lst <= lg_lst
}

fn is_overlapping(range1: Vec<u64>, range2: Vec<u64>) -> bool {
    if range1.len() > range2.len() {
        return overlaps(range2, range1);
    }
    overlaps(range1, range2)
}

fn overlaps(smaller: Vec<u64>, larger: Vec<u64>) -> bool {
    let (sm_fst, sm_lst) = (smaller[0], smaller[smaller.len() - 1]);
    let (lg_fst, lg_lst) = (larger[0], larger[larger.len() - 1]);
    (sm_fst >= lg_fst && sm_fst <= lg_lst) || (sm_lst >= lg_fst && sm_lst <= lg_lst)
}

#[cfg(test)]
mod tests {
    use crate::{parse_pair, gen_range, is_exactly_contained, is_overlapping};

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

    #[test]
    fn test_is_exactly_contained() {
        assert!(is_exactly_contained(vec![1], vec![1]));
        assert!(is_exactly_contained(vec![1], vec![1, 2]));
        assert!(is_exactly_contained(vec![2], vec![1, 2]));
        assert!(is_exactly_contained(vec![1, 2], vec![1]));
        assert!(is_exactly_contained(vec![1, 2], vec![2]));

        assert!(!is_exactly_contained(vec![1], vec![2]));
        assert!(!is_exactly_contained(vec![2], vec![1]));
        assert!(!is_exactly_contained(vec![1, 2], vec![2, 3]));
        assert!(!is_exactly_contained(vec![2, 3], vec![1, 2]));
        assert!(!is_exactly_contained(vec![1, 2, 3], vec![4, 5, 6]));
        assert!(!is_exactly_contained(vec![4, 5, 6], vec![1, 2, 3]));

        // examples from problem definition
        assert!(is_exactly_contained(vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]));
        assert!(is_exactly_contained(vec![6], vec![4, 5, 6]));
        assert!(!is_exactly_contained(vec![2, 3, 4], vec![6, 7, 8]));
        assert!(!is_exactly_contained(vec![3, 4], vec![4, 5]));
        assert!(!is_exactly_contained(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(!is_exactly_contained(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(!is_exactly_contained(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(!is_exactly_contained(vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]));
    }

    #[test]
    fn test_is_overlapping() {
        assert!(is_overlapping(vec![1], vec![1]));
        assert!(is_overlapping(vec![1], vec![1, 2]));
        assert!(is_overlapping(vec![2], vec![1, 2]));
        assert!(is_overlapping(vec![1, 2], vec![1]));
        assert!(is_overlapping(vec![1, 2], vec![2]));
        assert!(is_overlapping(vec![1, 2], vec![2, 3]));
        assert!(is_overlapping(vec![2, 3], vec![1, 2]));

        assert!(!is_overlapping(vec![1], vec![2]));
        assert!(!is_overlapping(vec![2], vec![1]));
        assert!(!is_overlapping(vec![1, 2, 3], vec![4, 5, 6]));
        assert!(!is_overlapping(vec![4, 5, 6], vec![1, 2, 3]));

        // examples from problem definition
        assert!(is_overlapping(vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]));
        assert!(is_overlapping(vec![6], vec![4, 5, 6]));
        assert!(!is_overlapping(vec![2, 3, 4], vec![6, 7, 8]));
        assert!(is_overlapping(vec![3, 4], vec![4, 5]));
        assert!(is_overlapping(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(is_overlapping(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(is_overlapping(vec![5, 6, 7], vec![7, 8, 9]));
        assert!(is_overlapping(vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]));
    }

}

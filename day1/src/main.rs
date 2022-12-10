use std::env;
use std::fs;

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args()
        .nth(2)
        .expect("param not provided: part (valid options: p1 | p2)");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let elf_cals: Vec<&str> = file_contents.split("\n\n").collect();

    match part.as_str() {
        "p1" => p1(elf_cals),
        "p2" => p2(elf_cals),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(elf_cals: Vec<&str>) {
    let mut highest_cals = 0;

    for elf in elf_cals {
        let elf = elf.trim_end_matches("\n");
        let elf_cals = sum_str_vec(elf);
        if elf_cals > highest_cals {
            highest_cals = elf_cals
        }
    }
    println!("highest cals: {}", highest_cals);
}

fn p2(elf_cals: Vec<&str>) {
    let mut all_cals: Vec<i64> = vec![];

    for elf in elf_cals {
        let elf_cals = sum_str_vec(elf);
        all_cals.push(elf_cals);
    }
    all_cals.sort();
    all_cals.reverse();

    let (top_elves, _other_elves) = all_cals.split_at(3);
    let total_cals = top_elves.to_vec().iter().sum::<i64>();
    println!("top elves: {:?}", top_elves);
    println!("top cals (total): {}", total_cals);
}

fn sum_str_vec(elf: &str) -> i64 {
    if elf.is_empty() {
        return 0;
    }

    let elf = elf.trim_end_matches("\n");
    return elf
        .split("\n")
        .map(|e| e.parse().unwrap())
        .collect::<Vec<i64>>()
        .iter()
        .sum::<i64>();
}

#[cfg(test)]
mod tests {
    use crate::sum_str_vec;

    #[test]
    fn test_sum_str_vec_given_empty_str_return_zero() {
        let elf = "";
        let res = sum_str_vec(elf);
        assert_eq!(res, 0);
    }

    #[test]
    fn test_sum_str_vec_given_single_element_then_return_correct_res() {
        let elf1 = "69\n";
        let elf2 = "69";
        let res1 = sum_str_vec(elf1);
        let res2 = sum_str_vec(elf2);
        assert_eq!(res1, 69);
        assert_eq!(res2, 69);
    }

    #[test]
    fn test_sum_str_vec_given_multiple_elements_then_return_correct_res() {
        let elf = "100\n100\n100";
        let res = sum_str_vec(elf);
        assert_eq!(res, 300);
    }
}

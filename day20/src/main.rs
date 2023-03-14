use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let numbers = parse_numbers(file_contents);

    match part.as_str() {
        "p1" => p1(numbers),
        "p2" => p2(numbers),
        _ => println!(""),
    }
}

fn p1(numbers: Vec<i64>) {
    let mut decrypted_signal: Vec<_> = (0..numbers.len()).collect();
    for (index, &num) in numbers.iter().enumerate() {
        let decrypted_signal_index = find_index(decrypted_signal.clone(), index);
        decrypted_signal.remove(decrypted_signal_index);
        let new_mixed_idx = get_new_index(decrypted_signal_index, num, decrypted_signal.len());
        decrypted_signal.insert(new_mixed_idx, index);
    }

    let zero_index = numbers.iter().position(|&num| num == 0).unwrap();
    let zero_decrypted_signal_index =
        get_decrytped_signal_zero_index(decrypted_signal.clone(), zero_index);

    let res = [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_decrypted_signal_index + offset) % decrypted_signal.len();
            let nums_idx = decrypted_signal[mixed_idx];
            numbers[nums_idx]
        })
        .sum::<i64>();

    println!("res = {}", res);
}

pub fn p2(numbers: Vec<i64>) {
    let decryption_key = 811589153;
    let original_signal: Vec<_> = numbers.iter().map(|num| num * decryption_key).collect();
    let mut decrypted_signal: Vec<_> = (0..original_signal.len()).collect();

    for _ in 0..10 {
        for (idx, &num) in original_signal.iter().enumerate() {
            let decrypted_signal_index = get_decrytped_signal_zero_index(decrypted_signal.clone(), idx);
            decrypted_signal.remove(decrypted_signal_index);
            let new_decrypted_signal_index =
                (decrypted_signal_index as i64 + num).rem_euclid(decrypted_signal.len() as i64) as usize;
            decrypted_signal.insert(new_decrypted_signal_index, idx);
        }
    }

    let zero_index = original_signal.iter().position(|&num| num == 0).unwrap();
    let zero_decrypted_signal_index = get_decrytped_signal_zero_index(decrypted_signal.clone(), zero_index);

    let res = [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_idx = (zero_decrypted_signal_index + offset) % decrypted_signal.len();
            let nums_idx = decrypted_signal[mixed_idx];
            original_signal[nums_idx]
        })
        .sum::<i64>();

    println!("res = {}", res);
}

fn parse_numbers(file_contents: String) -> Vec<i64> {
    file_contents
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn find_index(mixed: Vec<usize>, index: usize) -> usize {
    mixed.iter().position(|&mix_num| mix_num == index).unwrap()
}

fn get_new_index(mixed_idx: usize, num: i64, signal_len: usize) -> usize {
    (mixed_idx as i64 + num).rem_euclid(signal_len as i64) as usize
}

fn get_decrytped_signal_zero_index(
    decrypted_signal: Vec<usize>,
    signal_zero_index: usize,
) -> usize {
    decrypted_signal
        .iter()
        .position(|&mix_num| mix_num == signal_zero_index)
        .unwrap()
}

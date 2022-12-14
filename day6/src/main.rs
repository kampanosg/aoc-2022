use std::{env, fs, collections::HashSet};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let signal = fs::read_to_string(file_path).unwrap();

    match part.as_str() {
        "p1" => p1(signal),
        "p2" => p2(signal),
        _ => println!(""),
    }
}

fn p1(signal: String) {
    let (marker, index) = get_first_marker(signal);
    println!("{:?}, {:?}", marker, index);
}

fn p2(signal: String) {
    let index = get_first_msg(signal);
    println!("{:?}", index)
}

fn get_first_marker(signal: String) -> (char, usize) {
    let mut counter_a = 0;
    let mut counter_b = 1;
    let mut counter_c = 2;
    let mut counter_d = 3;

    while counter_d < signal.len() - 1 {

        let signal_bit_a = signal.chars().nth(counter_a).unwrap();
        let signal_bit_b = signal.chars().nth(counter_b).unwrap();
        let signal_bit_c = signal.chars().nth(counter_c).unwrap();
        let signal_bit_d = signal.chars().nth(counter_d).unwrap();

        if is_marker_seq(signal_bit_a, signal_bit_b, signal_bit_c, signal_bit_d) {
            return (signal_bit_d, counter_d + 1);
        }

        counter_a += 1;
        counter_b += 1;
        counter_c += 1;
        counter_d += 1;
    }
    (' ', 0)
}

fn is_marker_seq(signal_bit_a: char, signal_bit_b: char, signal_bit_c: char, signal_bit_d: char) -> bool {
    let bits = HashSet::from([
        signal_bit_a,
        signal_bit_b,
        signal_bit_c,
        signal_bit_d
    ]);
   bits.len() == 4 
}

// fn get_first_marker(signal: String) -> (char, usize) {
//     let mut parsed_signal: Vec<char> = vec![];
//     for c in signal.chars() {
//         parsed_signal.push(c);
//         if parsed_signal.len() < 4 {
//             continue;
//         }

//         if contains_marker(&parsed_signal[0..&parsed_signal.len() - 1], c) {
//             return (c, parsed_signal.len()); 
//         }
//     }
//     (' ', 0)
// }

#[cfg(test)]
mod tests {
    use crate::{get_first_marker, is_marker_seq};


#[test]
    fn test_is_marker_seq() {
        assert!(is_marker_seq('a', 'b', 'c', 'd'));
        assert!(is_marker_seq('p', 'q', 'm', 'j'));
        assert!(!is_marker_seq('m', 'j', 'q', 'j'));
        assert!(!is_marker_seq('a', 'a', 'a', 'a'));
    }

    #[test]
    fn test_get_first_marker() {
        // examples from problem definition
        assert_eq!(get_first_marker(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")), ('j', 5));
        assert_eq!(get_first_marker(String::from("nppdvjthqldpwncqszvftbrmjlhg")), ('j', 6));
        assert_eq!(get_first_marker(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), ('t', 10));
        assert_eq!(get_first_marker(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), ('r', 11));
    }


}

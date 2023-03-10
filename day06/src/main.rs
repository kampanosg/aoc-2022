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

fn get_first_msg(signal: String) -> usize {
    let mut counter_a = 0;
    let mut counter_b = 1;
    let mut counter_c = 2;
    let mut counter_d = 3;
    let mut counter_e = 4;
    let mut counter_f = 5;
    let mut counter_g = 6;
    let mut counter_h = 7;
    let mut counter_i = 8;
    let mut counter_j = 9;
    let mut counter_k = 10;
    let mut counter_l = 11;
    let mut counter_m = 12;
    let mut counter_n = 13;

    while counter_e < signal.len() - 1 {

        let signal_bits = vec![
            signal.chars().nth(counter_a).unwrap(),
            signal.chars().nth(counter_b).unwrap(),
            signal.chars().nth(counter_c).unwrap(),
            signal.chars().nth(counter_d).unwrap(),
            signal.chars().nth(counter_e).unwrap(),
            signal.chars().nth(counter_f).unwrap(),
            signal.chars().nth(counter_g).unwrap(),
            signal.chars().nth(counter_h).unwrap(),
            signal.chars().nth(counter_i).unwrap(),
            signal.chars().nth(counter_j).unwrap(),
            signal.chars().nth(counter_k).unwrap(),
            signal.chars().nth(counter_l).unwrap(),
            signal.chars().nth(counter_m).unwrap(),
            signal.chars().nth(counter_n).unwrap(),
        ];

        if is_msg_seq(signal_bits) {
            return counter_n + 1;
        }

        counter_a += 1;
        counter_b += 1;
        counter_c += 1;
        counter_d += 1;
        counter_e += 1;
        counter_f += 1;
        counter_g += 1;
        counter_h += 1;
        counter_i += 1;
        counter_j += 1;
        counter_k += 1;
        counter_l += 1;
        counter_m += 1;
        counter_n += 1;
    }
    0
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

fn is_msg_seq(signal_bits: Vec<char>) -> bool {
    let mut bits = HashSet::new();
    for c in signal_bits {
        bits.insert(c);
    }
    bits.len() == 14
}

#[cfg(test)]
mod tests {
    use crate::{get_first_marker, is_marker_seq, is_msg_seq};


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

    #[test]
    fn test_is_msg_seq() {
       assert!(is_msg_seq(vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n'])); 
       assert!(!is_msg_seq(vec!['a', 'b', 'b', 'a', 'e', 'w', 'g', 'a', 'i', 'e', 'k', 'l', 'n', 'n']));
    }

}

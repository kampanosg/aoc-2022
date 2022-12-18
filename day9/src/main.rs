use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");
    let file_contents = fs::read_to_string(file_path).unwrap();

    let instructions = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(instructions),
        _ => println!(""),
    }
}

fn p1(instructions: Vec<&str>) {
    let mut head: (i64, i64) = (0, 0);
    let mut _tail = (0, 0);

    for instruction in instructions {
        let instruction = instruction.split(" ").collect::<Vec<&str>>();
        let direction = instruction[0];
        let count = instruction[1].parse::<i64>().unwrap();

        print!("{:?} {:?} - ", direction, count);
        let (mut current_x, mut current_y) = head;

        match direction {
            "R" => current_x += count,
            "L" => current_x -= count,
            "U" => current_y += count,
            "D" => current_y -= count,
            _ => println!("huh?"),
        }
        


        head = (current_x, current_y);
        println!("{:?}", head);
    }

    println!("{:?}", head);
}

fn is_adjacent(h: (i64, i64), t: (i64, i64)) -> bool {
    if are_overlapping(h, t) {
        return true;
    }
    false    
 }

fn are_overlapping(h: (i64, i64), t: (i64, i64)) -> bool {
    h == t
}


#[cfg(test)]
mod tests {
    use crate::are_overlapping;


    #[test]
    fn test_are_overlapping() {
        assert!(are_overlapping((0, 0), (0, 0)));
        assert!(are_overlapping((1, 1), (1, 1)));
        assert!(are_overlapping((69, 69), (69, 69)));
        assert!(are_overlapping((-10, -10), (-10, -10)));
        assert!(!are_overlapping((0, 1), (1, 0)));
        assert!(!are_overlapping((-10, -10), (10, 10)));
        assert!(!are_overlapping((1, 2), (3, 4)));
    }

}

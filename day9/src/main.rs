use std::{collections::HashSet, env, fs};

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
    let mut tail = (0, 0);
    let mut tail_positions: HashSet<(i64, i64)> = HashSet::new();
    tail_positions.insert((0, 0));

    for instruction in instructions {
        let instruction = instruction.split(" ").collect::<Vec<&str>>();
        let direction = instruction[0];
        let count = instruction[1].parse::<i64>().unwrap();

        let (mut current_hx, mut current_hy) = head;
        let (mut current_tx, mut current_ty) = tail;

        for _ in 0..count {
            match direction {
                "R" => current_hx += 1,
                "L" => current_hx -= 1,
                "U" => current_hy += 1,
                "D" => current_hy -= 1,
                _ => println!("huh?"),
            }

            head = (current_hx, current_hy);

            if are_adjacent(current_hx, current_hy, current_tx, current_ty) {
                continue;
            }

            let diff_x = current_hx - current_tx;
            let diff_y = current_hy - current_ty;

            // move the tail
            // + 1 if the diff is positive
            // - 1 if the diff is negative
            // 0 don't move
            current_tx += diff_x.signum();
            current_ty += diff_y.signum();
            tail = (current_tx, current_ty);
            tail_positions.insert(tail);
        }
    }

    println!("{}", tail_positions.len());
}

fn are_adjacent(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    let diff_x = (hx - tx).abs();
    let diff_y = (hy - ty).abs();
    diff_x < 2 || diff_y < 2
}

#[cfg(test)]
mod tests {
    use crate::are_adjacent;

    #[test]
    fn test_are_adjacent() {
        assert!(are_adjacent(0, 0, 1, 1));
        assert!(are_adjacent(0, 0, 0, 1));
        assert!(are_adjacent(0, 0, 1, 0));
        assert!(are_adjacent(0, 0, 0, 0));
        assert!(are_adjacent(0, 0, -1, -1));
        assert!(are_adjacent(0, 0, 1, 2));
        assert!(are_adjacent(0, 0, 2, 1));
        assert!(!are_adjacent(0, 0, 2, 2));
        assert!(!are_adjacent(0, 0, -2, -2));
    }
}

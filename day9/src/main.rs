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
    let mut tail = (0, 0);

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
        println!("head = {:?}, tail = {:?}", head, tail);

        if is_adjacent(head, tail) {
            println!("adjacent");
        }
    }

    println!("{:?}", head);
}

fn is_adjacent(h: (i64, i64), t: (i64, i64)) -> bool {
    if are_overlapping(h, t) {
        return true;
    }

    let (hx, hy) = h;
    let (tx, ty) = t;

    are_adjacent_top(hx, hy, tx, ty)
        || are_adjacent_tr(hx, hy, tx, ty)
        || are_adjacent_right(hx, hy, tx, ty)
        || are_adjacent_br(hx, hy, tx, ty)
        || are_adjacent_bottom(hx, hy, tx, ty)
        || are_adjacent_bl(hx, hy, tx, ty)
        || are_adjacent_left(hx, hy, tx, ty)
        || are_adjacent_tl(hx, hy, tx, ty)
}

fn are_overlapping(h: (i64, i64), t: (i64, i64)) -> bool {
    h == t
}

fn are_adjacent_bottom(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hy - 1) == ty) && (hx == tx)
}

fn are_adjacent_top(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hy + 1) == ty) && (hx == tx)
}

fn are_adjacent_right(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx + 1) == tx) && (hy == ty)
}

fn are_adjacent_left(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx - 1) == tx) && (hy == ty)
}

fn are_adjacent_tl(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx - 1) == tx) && ((hy + 1) == ty)
}

fn are_adjacent_tr(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx + 1) == tx) && ((hy + 1) == ty)
}

fn are_adjacent_bl(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx - 1) == tx) && ((hy - 1) == ty)
}

fn are_adjacent_br(hx: i64, hy: i64, tx: i64, ty: i64) -> bool {
    ((hx + 1) == tx) && ((hy - 1) == ty)
}

#[cfg(test)]
mod tests {
    use crate::{
        are_adjacent_bl, are_adjacent_bottom, are_adjacent_br, are_adjacent_left,
        are_adjacent_right, are_adjacent_tl, are_adjacent_top, are_adjacent_tr, are_overlapping,
    };

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

    #[test]
    fn test_are_adjacent_bottom() {
        assert!(are_adjacent_bottom(10, 9));
        assert!(are_adjacent_bottom(1, 0));
        assert!(are_adjacent_bottom(0, -1));
        assert!(!are_adjacent_bottom(10, 8));
        assert!(!are_adjacent_bottom(1, -1));
    }

    #[test]
    fn test_are_adjacent_top() {
        assert!(are_adjacent_top(9, 10));
        assert!(are_adjacent_top(1, 2));
        assert!(are_adjacent_top(-1, 0));
        assert!(!are_adjacent_top(10, 12));
        assert!(!are_adjacent_top(1, 4));
    }

    #[test]
    fn test_are_adjacent_right() {
        assert!(are_adjacent_right(0, 1));
        assert!(are_adjacent_right(1, 2));
        assert!(are_adjacent_right(-1, 0));
        assert!(!are_adjacent_right(10, 12));
        assert!(!are_adjacent_right(1, 4));
    }

    #[test]
    fn test_are_adjacent_left() {
        assert!(are_adjacent_left(1, 0));
        assert!(are_adjacent_left(2, 1));
        assert!(are_adjacent_left(0, -1));
        assert!(!are_adjacent_left(10, 12));
        assert!(!are_adjacent_left(-1, -3));
    }

    #[test]
    fn test_are_adjacent_tl() {
        assert!(are_adjacent_tl(1, 0, 0, 1));
        assert!(!are_adjacent_tl(1, 0, 1, 1));
        assert!(!are_adjacent_tl(1, 0, 2, 1));
        assert!(!are_adjacent_tl(1, 0, 0, 0));
    }

    #[test]
    fn test_are_adjacent_tr() {
        assert!(are_adjacent_tr(1, 0, 2, 1));
        assert!(!are_adjacent_tr(1, 0, 1, 0));
        assert!(!are_adjacent_tr(1, 0, 1, 1));
        assert!(!are_adjacent_tr(1, 0, 0, 0));
    }

    #[test]
    fn test_are_adjacent_bl() {
        assert!(are_adjacent_bl(1, 1, 0, 0));
        assert!(!are_adjacent_bl(1, 1, 1, 0));
        assert!(!are_adjacent_bl(1, 1, 0, 1));
        assert!(!are_adjacent_bl(1, 1, 0, 2));
    }

    #[test]
    fn test_are_adjacent_br() {
        assert!(are_adjacent_br(1, 1, 2, 0));
        assert!(!are_adjacent_br(1, 1, 2, 1));
        assert!(!are_adjacent_br(1, 1, 1, 2));
        assert!(!are_adjacent_br(1, 1, 0, 0));
    }
}

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
        let (mut current_hx, mut current_hy) = head;
        let (mut current_tx, mut current_ty) = tail;

        match direction {
            "R" => {
                for _ in 0..count {
                    current_hx += 1;
                    head = (current_hx, current_hy);

                    if is_adjacent(head, tail) {
                        continue;
                    }

                    if is_tail_horizontally_far_back(head, tail) {
                        if current_hy > current_ty {
                            // head has moved top-right
                            // tail needs to move diagonally top-right
                            current_ty += 1;
                        } else {
                            // head has moved bottom-right
                            // tail needs to move bottom bottom-right
                            current_ty -= 1;
                        }
                        current_tx += 1;
                    }
                    current_tx += 1;
                    tail = (current_tx, current_ty);
                }
            }
            "L" => {
                for _ in 0..count {
                    current_hx -= 1;
                    head = (current_hx, current_hy);

                    if is_adjacent(head, tail) {
                        continue;
                    }

                    if is_tail_horizontally_far_back(head, tail) {
                        if current_hy > current_ty {
                            // head has moved top-right
                            // tail needs to move diagonally top-right
                            current_ty += 1;
                        } else {
                            // head has moved bottom-right
                            // tail needs to move bottom bottom-right
                            current_ty -= 1;
                        }
                    }
                    current_tx -= 1;
                    tail = (current_tx, current_ty)
                }
            }
            "U" => {
                for _ in 0..count {
                    current_hy += 1;
                    head = (current_hx, current_hy);
                    // print!("head = {:?}", head);
                    if is_adjacent(head, tail) {
                        // print!(" (adjacent) tail = {:?} | ", tail);
                        continue;
                    }
                    if is_tail_vetically_far_back(head, tail) {
                        // print!(" (failling behind) ");
                        if current_hx > current_ty {
                            // head has moved towards top-right
                            // so tail has to move diagonally top-right
                            current_tx += 1;
                        } else {
                            // head has moved towards top-left
                            // so tail has to move diagonally top left
                            current_tx -= 1;
                        }
                        // print!("tail = {:?} ", tail);
                    }
                    current_ty += 1;
                    tail = (current_tx, current_ty);
                    // print!(" tail = {:?} | ", tail);
                }
                // println!();
            }
            "D" => {
                for _ in 0..count {
                    current_hy -= 1;
                    head = (current_hx, current_hy);

                    if is_adjacent(head, tail) {
                        continue;
                    }

                    if is_tail_vetically_far_back(head, tail) {
                        if current_hx > current_ty {
                            // head has moved towards top-right
                            // so tail has to move diagonally top-right
                            current_tx += 1;
                        } else {
                            // head has moved towards top-left
                            // so tail has to move diagonally top left
                            current_tx -= 1;
                        }
                    }
                    current_ty -= 1;
                    tail = (current_tx, current_ty);
                }
            }
            _ => println!("huh?"),
        }

        //         head = (current_x, current_y);
        // println!("head = {:?}, tail = {:?}", head, tail);

        //         if is_adjacent(head, tail) {
        //             println!("adjacent");
        //         }
    }

    println!();
    println!("head = {:?}, tail = {:?}", head, tail);
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

fn is_tail_vetically_far_back(h: (i64, i64), t: (i64, i64)) -> bool {
    let (hx, hy) = h;
    let (tx, ty) = t;

    let is_diff_col = hx != tx;

    let is_head_further_up = (hy + 2) == ty;
    let is_head_further_down = (hy - 2) == ty;

    is_diff_col && (is_head_further_up || is_head_further_down)
}

fn is_tail_horizontally_far_back(h: (i64, i64), t: (i64, i64)) -> bool {
    let (hx, hy) = h;
    let (tx, ty) = t;

    let is_diff_row = hy != ty;

    let is_head_further_right = (tx + 2) == hx;
    let is_head_further_left = (tx - 2) == hx;

    is_diff_row && (is_head_further_left || is_head_further_right)
}

#[cfg(test)]
mod tests {
    use crate::{
        are_adjacent_bl, are_adjacent_bottom, are_adjacent_br, are_adjacent_left,
        are_adjacent_right, are_adjacent_tl, are_adjacent_top, are_adjacent_tr, are_overlapping,
        is_tail_horizontally_far_back, is_tail_vetically_far_back,
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
        assert!(are_adjacent_bottom(10, 10, 10, 9));
        assert!(!are_adjacent_bottom(10, 10, 10, 10));
        assert!(!are_adjacent_bottom(10, 10, 10, 11));
        assert!(!are_adjacent_bottom(10, 10, 9, 9));
    }

    #[test]
    fn test_are_adjacent_top() {
        assert!(are_adjacent_top(10, 10, 10, 11));
        assert!(!are_adjacent_top(10, 10, 10, 9));
        assert!(!are_adjacent_top(10, 10, 10, 12));
    }

    #[test]
    fn test_are_adjacent_right() {
        assert!(are_adjacent_right(10, 10, 11, 10));
        assert!(!are_adjacent_right(10, 10, 10, 8));
    }

    #[test]
    fn test_are_adjacent_left() {
        assert!(are_adjacent_left(10, 10, 9, 10));
        assert!(!are_adjacent_left(10, 10, 10, 10));
        assert!(!are_adjacent_left(10, 10, 10, 12));
        assert!(!are_adjacent_left(10, 10, 11, 11));
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

    #[test]
    fn test_is_tail_vertically_further_back() {
        assert!(is_tail_vetically_far_back((4, 2), (3, 4)));
        assert!(is_tail_vetically_far_back((4, 2), (5, 4)));
        assert!(is_tail_vetically_far_back((4, 2), (3, 0)));
        assert!(is_tail_vetically_far_back((4, 2), (5, 0)));
        assert!(!is_tail_vetically_far_back((4, 2), (4, 4)));
        assert!(!is_tail_vetically_far_back((4, 2), (5, 2)));
        assert!(!is_tail_vetically_far_back((4, 2), (4, 0)));
        assert!(!is_tail_vetically_far_back((4, 2), (3, 2)));
    }

    #[test]
    fn test_is_tail_horizontally_further_back() {
        assert!(is_tail_horizontally_far_back((2, 2), (0, 3)));
        assert!(is_tail_horizontally_far_back((2, 2), (4, 3)));
        assert!(is_tail_horizontally_far_back((2, 2), (4, 1)));
        assert!(is_tail_horizontally_far_back((2, 2), (0, 1)));
        assert!(!is_tail_horizontally_far_back((2, 2), (1, 3)));
        assert!(!is_tail_horizontally_far_back((2, 2), (2, 3)));
        assert!(!is_tail_horizontally_far_back((2, 2), (3, 3)));
        assert!(!is_tail_horizontally_far_back((2, 2), (3, 2)));
        assert!(!is_tail_horizontally_far_back((2, 2), (3, 1)));
        assert!(!is_tail_horizontally_far_back((2, 2), (2, 1)));
        assert!(!is_tail_horizontally_far_back((2, 2), (1, 1)));
        assert!(!is_tail_horizontally_far_back((2, 2), (1, 2)));
    }
}

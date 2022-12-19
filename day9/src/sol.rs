use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
struct Coord {
    x: isize,
    y: isize,
}

pub fn part_1() -> usize {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let start = Coord { x: 0, y: 0 };
    let mut head = start;
    let mut tail = start;
    let mut seen = HashSet::new();
    seen.insert(tail);

    for line in input.lines() {
        let (dir, amount) = line.split_once(' ').unwrap();
        let amount = amount.parse().unwrap();

        for _ in 0..amount {
            // move head
            match dir {
                "U" => head.y -= 1,
                "D" => head.y += 1,
                "L" => head.x -= 1,
                "R" => head.x += 1,
                _ => panic!("tried to move in invalid direction"),
            };

            // determine if head and tail are touching
            let diff = Coord {
                x: head.x - tail.x,
                y: head.y - tail.y,
            };
            let not_touching = diff.x.abs() > 1 || diff.y.abs() > 1;

            // update tail and insert it into the seen set if needed
            if not_touching {
                tail.x += diff.x.signum();
                tail.y += diff.y.signum();
                seen.insert(tail);
            }
        }
    }

    seen.len()
}

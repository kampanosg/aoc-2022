pub mod structs;
use std::collections::HashMap;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let map = parse_map(file_contents);

    match part.as_str() {
        "p1" => p1(map),
        _ => println!(""),
    }
}


fn p1(map: structs::AreaMap) {
    dbg!(map);
}

fn parse_map(file_contents: String) -> structs::AreaMap {
    let hills: Vec<Vec<_>> = file_contents
            .lines()
            .map(|row| row.chars().map(structs::Hill::from).collect())
            .collect();

        let mut graph = HashMap::new();

        let last_row = hills.len().saturating_sub(1);
        let last_col = hills
            .first()
            .map(|r| r.len())
            .unwrap_or_default()
            .saturating_sub(1);

        let mut start_at = (0, 0);
        let mut end_at = (0, 0);

        for (row_idx, row) in hills.iter().enumerate() {
            for (col_idx, hill) in row.iter().enumerate() {
                
                let mut neighbors = [None; 4];
                if row_idx > 0 && hill.can_reach(&hills[row_idx - 1][col_idx]) {
                    neighbors[0] = Some((row_idx - 1, col_idx));
                }
                if col_idx > 0 && hill.can_reach(&hills[row_idx][col_idx - 1]) {
                    neighbors[1] = Some((row_idx, col_idx - 1));
                }
                if row_idx < last_row && hill.can_reach(&hills[row_idx + 1][col_idx]) {
                    neighbors[2] = Some((row_idx + 1, col_idx));
                }
                if col_idx < last_col && hill.can_reach(&hills[row_idx][col_idx + 1]) {
                    neighbors[3] = Some((row_idx, col_idx + 1));
                }

                if let structs::Hill::Start(_) = hill {
                    start_at = (row_idx, col_idx);
                }
                if let structs::Hill::End(_) = hill {
                    end_at = (row_idx, col_idx);
                }
                graph.insert((row_idx, col_idx), neighbors);
            }
        }

        structs::AreaMap {
            hills,
            graph,
            start_at,
            end_at,
        }
}

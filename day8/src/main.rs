pub mod structs;
use std::{env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args().nth(2).expect("param not provided: path");

    let file_contents = fs::read_to_string(file_path).unwrap();
    let forrest = parse_forrest(file_contents);
    match part.as_str() {
        "p1" => p1(forrest),
        "p2" => p2(forrest),
        _ => println!(""),
    }
}

fn p1(forrest: structs::Forrest) {
    let mut visible_trees = 0;
    for tree_row in forrest.trees.clone() {
        for tree in tree_row {
            if forrest.is_edge(tree.clone()) {
                visible_trees += 1;
                continue;
            }

            let mut vertical_index = 0;
            let mut horizontal_index = tree.coords.x;
            let mut found_bigger_tree = false;

           while vertical_index != tree.coords.y {
                let next_tree =
                    forrest.trees[vertical_index as usize][horizontal_index as usize].clone();
                // print!("{:?} ", next_tree.height);
                if next_tree.height >= tree.height {
                    found_bigger_tree = true;
                    break;
                }
                vertical_index += 1;
            }

            if !found_bigger_tree {
                visible_trees += 1;
                continue;
            }

            found_bigger_tree = false;
            vertical_index = forrest.edge.y;
            horizontal_index = tree.coords.x;
            while vertical_index != tree.coords.y {
                let next_tree =
                    forrest.trees[vertical_index as usize][horizontal_index as usize].clone();
                if next_tree.height >= tree.height {
                    found_bigger_tree = true;
                    break;
                }
                vertical_index -= 1;
            }

            if !found_bigger_tree {
                visible_trees += 1;
                continue;
            }

            found_bigger_tree = false;
            vertical_index = tree.coords.y;
            horizontal_index = 0;
            while horizontal_index != tree.coords.x {
                let next_tree =
                    forrest.trees[vertical_index as usize][horizontal_index as usize].clone();
                if next_tree.height >= tree.height {
                    found_bigger_tree = true;
                    break;
                }
                horizontal_index += 1;
            }

            if !found_bigger_tree {
                visible_trees += 1;
                continue;
            }

            found_bigger_tree = false;
            vertical_index = tree.coords.y;
            horizontal_index = forrest.edge.x;
            while horizontal_index != tree.coords.x {
                let next_tree =
                    forrest.trees[vertical_index as usize][horizontal_index as usize].clone();
                if next_tree.height >= tree.height {
                    found_bigger_tree = true;
                    break;
                }
                horizontal_index -= 1;
            }

            if !found_bigger_tree {
                visible_trees += 1;
                continue;
            }
        }
    }
    println!("visible trees: {:?}", visible_trees);
}

fn p2(forrest: structs::Forrest) {
    
}

fn parse_forrest(file_contents: String) -> structs::Forrest {
    let mut y = 0;
    let mut forrest: Vec<Vec<structs::Tree>> = vec![];

    let rows = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    for row in rows {
        let mut x = 0;
        let mut trees: Vec<structs::Tree> = vec![];

        let cols = row.split("").collect::<Vec<&str>>();
        let heights = cols
            .iter()
            .filter(|c| **c != "")
            .map(|c| c.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        for height in heights {
            trees.push(structs::Tree::new(height, y, x));
            x += 1;
        }
        y += 1;

        forrest.push(trees);
    }
    structs::Forrest::new(forrest)
}

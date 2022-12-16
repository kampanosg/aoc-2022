pub mod structs;
use id_tree::InsertBehavior::*;
use id_tree::*;
use std::{collections::HashMap, env, fs};

fn main() {
    let file_path = env::args().nth(1).expect("param not provided: file_path");
    let part = env::args()
        .nth(2)
        .expect("param not provided: part (valid options: p1 | p2)");

    let file_contents = fs::read_to_string(file_path).unwrap();

    let commands = file_contents
        .trim_end_matches("\n")
        .split("\n")
        .collect::<Vec<&str>>();

    match part.as_str() {
        "p1" => p1(commands),
        "p2" => p1(commands),
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(commands: Vec<&str>) {
    let mut tree: Tree<structs::Directory> = TreeBuilder::new().with_node_capacity(5).build();

    let root = structs::Directory::new("/".to_string(), None);
    let root: NodeId = tree.insert(Node::new(root), AsRoot).unwrap();
    let mut current_dir = root.clone();

    let mut files: HashMap<NodeId, Vec<structs::File>> = HashMap::new();
    let mut folders: HashMap<NodeId, HashMap<&str, NodeId>> = HashMap::new();

    for c in &commands[1..] {
        let command = c.split(" ").collect::<Vec<&str>>();

        match command[0] {
            "$" => match command[1] {
                "cd" => {
                    if command[2] == ".." {
                        let dir = tree.get(&current_dir).unwrap().clone();
                        let papa = dir.parent().unwrap().clone();
                        current_dir = papa;
                    } else {
                        let name = command[2];
                        let children = folders.get(&current_dir.clone()).unwrap().clone();
                        let next_dir = children.get(name).unwrap().clone();
                        current_dir = next_dir;
                    }
                }
                _ => (),
            },
            "dir" => {
                let dir_name = command[1];
                let nested_dir =
                    structs::Directory::new(dir_name.to_string(), Some(current_dir.clone()));
                let child_id: NodeId = tree
                    .insert(Node::new(nested_dir), UnderNode(&current_dir))
                    .unwrap();

                if folders.contains_key(&&current_dir.clone()) {
                    let mut children = folders.get(&current_dir.clone()).unwrap().clone();
                    children.insert(dir_name.clone(), child_id.clone());
                    folders.insert(current_dir.clone(), children.clone());
                } else {
                    let mut h = HashMap::new();
                    h.insert(dir_name.clone(), child_id.clone());
                    folders.insert(current_dir.clone(), h);
                }
            }
            _ => {
                // catch the ls command
                let file_name = command[1].to_string();
                let file_size = command[0].parse::<u64>().unwrap();
                let file = structs::File::new(file_name, file_size);
                if files.contains_key(&current_dir.clone()) {
                    let mut dir_files = files.get(&current_dir.clone()).unwrap().to_vec();
                    let mut seen = false;
                    for f in dir_files.clone() {
                        if f.name == file.name {
                            seen = true;
                        }
                    }
                    if !seen {
                        dir_files.push(file);
                        files.insert(current_dir.clone(), dir_files.clone());
                    }
                } else {
                    files.insert(current_dir.clone(), vec![file]);
                }
            }
        }
    }

    let mut sizes: HashMap<NodeId, u64> = HashMap::new();
    for node in tree.traverse_pre_order_ids(&root).unwrap() {
        let size = calc_size(&tree, node.clone(), files.clone());
        sizes.insert(node.clone(), size);
    }

    let mut small_sizes = 0;
    for (_k, v) in &sizes {
        if v <= &100000 {
            small_sizes += v;
        }
    }
    println!("total: {}", small_sizes);
}

fn calc_size(
    tree: &Tree<structs::Directory>,
    node_id: NodeId,
    files: HashMap<NodeId, Vec<structs::File>>,
) -> u64 {
    let mut total: u64 = 0;

    total += files
        .get(&node_id.clone())
        .unwrap()
        .iter()
        .map(|f| f.size)
        .sum::<u64>();

    let children = tree.get(&node_id).unwrap().children();
    for child_node in children {
        total += calc_size(tree, child_node.clone(), files.clone());
    }

    total
}

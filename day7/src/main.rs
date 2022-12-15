pub mod structs;
use std::{collections::HashMap, env, fs};
use id_tree::*;
use id_tree::InsertBehavior::*;

fn main() {


//     let mut tree: Tree<i32> = TreeBuilder::new()
//         .with_node_capacity(5)
//         .build();

//     let root_id: NodeId = tree.insert(Node::new(0), AsRoot).unwrap();
//     let child_id: NodeId = tree.insert(Node::new(1), UnderNode(&root_id)).unwrap();
//     tree.insert(Node::new(2), UnderNode(&root_id)).unwrap();
//     tree.insert(Node::new(3), UnderNode(&child_id)).unwrap();
//     tree.insert(Node::new(4), UnderNode(&child_id)).unwrap();
// tree.insert(Node::new(5), UnderNode(&child_id)).unwrap();
// tree.insert(Node::new(6), UnderNode(&child_id)).unwrap();

// let mut s = String::new();
// tree.write_formatted(&mut s).unwrap();
// println!("{}", s);


    // println!("Pre-order:");
    // for node in tree.traverse_pre_order(&root_id).unwrap() {
    //     print!("{}, ", node.data());
    // }


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
        _ => println!("invalid part param (valid options: p1 | p2)"),
    }
}

fn p1(commands: Vec<&str>) {

    let mut tree: Tree<structs::Directory> = TreeBuilder::new()
        .with_node_capacity(5)
        .build();

    let root = structs::Directory::new("/".to_string(), None);
    let current_dir: NodeId = tree.insert(Node::new(root), AsRoot).unwrap();

    // for c in &commands[1..] {
    //     let command = c.split(" ").collect::<Vec<&str>>();

    //     println!("{:?}", command);
    //     match command[0] {
    //         "$" => {
    //             if command[1] == "cd" {
    //                 if command[1] == ".." {
    //                 } else {
    //                     let dir_name = command[2];
    //                     let next_dir = current_dir.get_dir(dir_name);
    //                     current_dir = next_dir.clone();
    //                 }
    //             }
    //         }
    //         "dir" => {
    //             let dir_name = command[1];
    //             current_dir.append_dir(dir_name);
    //         }
    //         _ => {
    //             let file_name = command[1];
    //             let file_size = command[0].parse::<u64>().unwrap();
    //             current_dir.append_file(file_name, file_size);
    //         }
    //     }
    // }

    // println!("{:?}", current_dir);
}

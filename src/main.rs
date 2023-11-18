mod r#move;
mod constants;
mod board;
mod helpers;
mod magic;
mod orchestradirector;
mod engine;
mod evaluation;
mod zobrist;
mod zobrist_impl;
mod timer;
mod tests;
mod book;
mod move_manager;
mod tree;


use std::{io, process::exit};
use constants::COLOR;
use std::sync::{Arc, Mutex};

use crate::board::Board;


use tree::Node;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::mem;

fn main() {

    // // Create a sample tree
    // let mut root = Node::new("Root".to_string(), 10);
    // let child1 = Node::new("Child1".to_string(), 5);
    // let child2 = Node::new("Child2".to_string(), 8);
    // root.add_child(child1);
    // root.add_child(child2);

    // // Serialize the tree to a JSON string
    // let json_string = serde_json::to_string_pretty(&root).unwrap();

    // // Write the JSON string to a file
    // let mut file = File::create("example_tree.json").expect("Unable to create file");
    // file.write_all(json_string.as_bytes()).expect("Unable to write to file");

    // // Read the JSON file into a string
    // let mut file = File::open("tree.json").expect("Unable to open file");
    // let mut json_string = String::new();
    // file.read_to_string(&mut json_string).expect("Unable to read file");

    // // Deserialize the JSON string into a Node tree
    // let deserialized_tree: Node = serde_json::from_str(&json_string).expect("Unable to deserialize JSON");
    // let node_size = mem::size_of::<Node>();
    // println!("Size of Node: {} bytes", node_size);
    // println!("Number of nodes: {}", deserialized_tree.score);

    // // Print the deserialized tree
    // println!("{:?}", deserialized_tree);


    // let the_file = fs::read_to_string("node.json").expect("Unable to read file");
    // println!("{}", the_file);

    // let node: Node = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    // println!("{:?}", node);

    let mut orchestra_director = orchestradirector::new_orchestra_director();
    // orchestra_director.handle_command("position", "startpos moves e2e4 e7e5 g1f3 b8c6 f1b5 c6d4 b5c4 d4b3 c4b3 d7d5 e4d5 c7c6 d5c6 d8d1 c6b");

    // orchestra_director.handle_command("position", "6qk/6pp/8/8/8/n3n3/8/1Q1Q3K w - - 0 1");
    // println!("{}", orchestra_director.eng.board.static_evaluation());
    // let x = orchestra_director.eng.quiescence_search(-250000, 250000, 0);
    // println!("{}", x);
    // exit(0);
    // orchestra_director.handle_command("go", "depth 14");

    loop {
        let mut message = String::new();

        // Read input from the user
        io::stdin().read_line(&mut message).expect("Failed to read input");

        let message = message.trim(); // Remove trailing newline

        // Split the message into command and options
        let mut parts = message.splitn(2, ' ');
        let command = parts.next().unwrap_or("");
        let options = parts.next().unwrap_or("");

        // Call a function to handle the command
        orchestra_director.handle_command(command, options);
    }
}



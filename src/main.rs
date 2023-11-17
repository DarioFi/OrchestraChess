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

    // orchestra_director.handle_command("position", "fen 3k4/1p6/1P3p1p/4p1pP/4P1P1/8/1K6/8 b - - 0 25");
    // orchestra_director.handle_command("go", "depth 14");
    // exit(0);

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



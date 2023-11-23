mod muve;
mod board;
mod utils;
mod magic;
mod orchestradirector;
mod engine;
mod evaluation;
mod zobrist;
mod timer;
mod tests;
mod book;
mod move_manager;
mod nnue;
mod accumulator;


use std::{io};
use std::process::exit;

fn main() {

    // region tree test
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
    // endregion

    // let nnue = Nnue::init();
    //
    //
    let mut orchestra_director = orchestradirector::new_orchestra_director();
    orchestra_director.handle_command("position", "fen 4k3/4p3/8/8/8/8/8/4K3 w - - 0 1");
    orchestra_director.handle_command("go", "depth 2");
    //
    // orchestra_director.handle_command("position", "startpos moves e2e4 d7d6 g1f3");
    // orchestra_director.handle_command("go", "movetime 1000");
    //
    //
    // orchestra_director.handle_command("position", "startpos moves e2e4 g8f6 b1c3 e7e5 d2d3 f8b4 c1d2 d7d6 f1e2 f6e4 c3e4 b4d2 d1d2 d6d5 e4g3 a7a6 g1f3 e5e4 d3e4 d5e4 g3e4 d8d2 e1d2 c8g4 a1e1 g4f3 e2f3 f7f5 e4c5 e8f7 c5b7");
    // orchestra_director.handle_command("go", "depth 4");
    //
    // exit(0);
    let mut orchestra_director = orchestradirector::new_orchestra_director();
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



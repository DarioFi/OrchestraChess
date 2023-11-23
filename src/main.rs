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


use std::{io};


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
    orchestra_director.handle_command("position", "startpos");
    orchestra_director.eng.benchmark_perf(6);

    // let mask: u64 = 0b10111111;
    // let keyy: u64 = 0b10101110;
    // let hash = hash_on_mask(keyy, mask);
    // let inverse = inverse_hash_on_mask(hash, mask);
    // println!("inverse {:b}", inverse);
    // println!("inverse {:b}", hash.pdep(mask));

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



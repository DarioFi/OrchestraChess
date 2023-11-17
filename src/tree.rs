use serde::{Deserialize, Serialize};
use std::fs;


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Node {
    pub mov: String,
    pub score: i32,  // number of times this continuation has been played in the database.
    pub children: Vec<Node>, 
}

impl Node {
    pub fn new(mov: String, score: i32) -> Node {
        Node {
            mov,
            score,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Node) {
        self.children.push(child);
    }
}

fn main() {

    // let mut root = Node::new("a2a3".to_string(), 100);
    // println!("{:?}", root);

    let the_file = fs::read_to_string("node.json").expect("Unable to read file");
    println!("{}", the_file);

    let node: Node = serde_json::from_str(&the_file).expect("JSON was not well-formatted");
    println!("{:?}", node);

    return;
}

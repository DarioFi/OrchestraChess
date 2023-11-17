use std::fs::File;
use crate::tree::Node;
use std::io::prelude::*;
use rand::Rng;

// pub struct OpeningBook {
//     list_position: Vec<String>,
// }
// impl OpeningBook {
//     pub fn new(path_to_file: &str) -> OpeningBook {
//         let mut positions: Vec<String> = Vec::new();

//         let file = File::open(path_to_file).expect("Unable to open file");

//         for line in BufReader::new(file).lines() {
//             positions.push(line.unwrap());
//         }

//         OpeningBook {
//             list_position: positions,
//         }
//     }

//     pub fn query(&self, moves: &str) -> Option<String> {
//         for line in self.list_position.iter() {
//             if line.trim_start().starts_with(moves.trim_start()) {

//                 // Zip the iterators over words
//                 let length_moves = moves.split_whitespace().count();
//                 let split_result: Vec<&str> = line.split_whitespace().collect();
//                 return Option::from(split_result[length_moves].to_string());
//             }
//         }
//         None
//     }
// }


// todo: add a temperature parameter and softmax.
pub struct OpeningBook {
    root: Node,
}

impl OpeningBook {
    pub fn new(path_to_file: &str) -> OpeningBook {
        // Read the JSON file into a string
        let mut file = File::open(path_to_file).expect("Unable to open file");
        let mut json_string = String::new();
        file.read_to_string(&mut json_string).expect("Unable to read file");

        // Deserialize the JSON string into a Node tree
        let deserialized_root: Node = serde_json::from_str(&json_string).expect("Unable to deserialize JSON");
        OpeningBook {
            root: deserialized_root,
        }
    }

    pub fn query(&self, moves: &str) -> Option<String> {
        let mut current_node = &self.root;
        for mov in moves.split_whitespace() {
            let mut found = false;
            for child in current_node.children.iter() {
                if child.mov == mov {
                    current_node = child;
                    found = true;
                    break;
                }
            }
            if !found {
                return None;
            }
        }
        if current_node.children.len() == 0 {
            return None;
        }

        // Select a random move with probability proportional to the score of each child
        let total_score = current_node.score;
        let mut rng = rand::thread_rng();
        let random_score = rng.gen::<i32>() % (total_score + 1);
        let mut current_score = 0;
        for child in current_node.children.iter() {
            current_score += child.score;
            if current_score >= random_score {
                return Option::from(child.mov.to_string());
            }
        }
        return None;
    }
}

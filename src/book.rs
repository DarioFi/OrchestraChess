use std::fs::File;
use crate::tree::Node;
use std::io::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use core::option::{Option::None};

const USE_BEST_MOVE: bool = false;
const USE_BOOK: bool = true;

pub struct OpeningBook {
    root: Node,
    seed: Option<u64>

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
            // seed: Option::from(11122001_u64)
            seed: None
        }
    }

    pub fn query(&self, moves: &str) -> Option<String> {
        if !USE_BOOK{
            return None;
        }
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

        let mut rng;
        if self.seed.is_some() {
            rng = StdRng::seed_from_u64(self.seed.unwrap());
        } else {
            rng = StdRng::from_entropy();
        }

        if USE_BEST_MOVE {
            let mut best_score = -1;
            let mut best_move = "";
            for child in current_node.children.iter() {
                if child.score > best_score {
                    best_score = child.score;
                    best_move = &child.mov;
                }
            }
            if best_score == -1{
                return None;
            }
            return Option::from(best_move.to_string());


        } else {
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
}

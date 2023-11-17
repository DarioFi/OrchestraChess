use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct OpeningBook {
    list_position: Vec<String>,
}


impl OpeningBook {
    pub fn new(path_to_file: &str) -> OpeningBook {
        let mut positions: Vec<String> = Vec::new();

        let file = File::open(path_to_file).expect("Unable to open file");

        for line in BufReader::new(file).lines() {
            positions.push(line.unwrap());
        }

        OpeningBook {
            list_position: positions,
        }
    }

    pub fn query(&self, moves: &str) -> Option<String> {
        return None;
        for line in self.list_position.iter() {
            if line.trim_start().starts_with(moves.trim_start()) {

                // Zip the iterators over words
                let length_moves = moves.split_whitespace().count();
                let split_result: Vec<&str> = line.split_whitespace().collect();
                return Option::from(split_result[length_moves].to_string());
            }
        }
        None
    }
}


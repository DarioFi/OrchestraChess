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
use crate::accumulator::make_index;

fn main() {
    use crate::accumulator::make_index;

    // let piece_index = 5;
    // let is_mine = 0;
    // let piece_square = 29;
    // let king_square = 29;
    //
    // let x = make_index(piece_index, is_mine, piece_square, king_square);
    // println!("{}", x);

    // let nnue = Nnue::init();
    //
    //
    // let mut orchestra_director = orchestradirector::new_orchestra_director();
    // orchestra_director.handle_command("position", "startpos moves e2e4");
    // orchestra_director.handle_command("go", "depth 3");
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



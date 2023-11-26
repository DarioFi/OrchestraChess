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
    // use crate::accumulator::make_index;

    // let piece_index = 5;
    // let is_mine = 0;
    // let piece_square = 37;
    // let king_square = 4;
    
    // let x = make_index(piece_index, is_mine, piece_square, king_square);
    // println!("{}", x);

    // // let nnue = Nnue::init();
    // //
    //position fen 4k3/QQ6/8/8/8/6P1/4qPP1/6K1 b - - 0 1 moves e2d1 g1h2 d1h5 h2g1 h5d1
    //     go depth
    // let mut orchestra_director = orchestradirector::new_orchestra_director();
    // orchestra_director.handle_command("position", "startpos moves e2e4 e7e5 g1f3 b8c6 f1b5 a7a6 b5a4 g8f6 e1g1 f8e7 f1e1 b7b5 a4b3 e8g8 c2c3 d7d6 h2h3 c6a5 b3c2 c7c5 d2d4 d8c7 d4d5 c8d7 b2b3 h7h6 c1e3 g7g5 b1d2 g8g7 d2f1 f8h8 f1g3 b5b4 c3b4 c5b4 c2d3 a5b7 a1c1 b7c5 d3e2 g5g4 f3h4 f6e4 g3e4 e7h4 e4c5 d6c5 e2g4 h4e7 g4d7 c7d7 e3c5 a8c8 d5d6 e7f6 c5b4 c8c1 d1c1 h8c8 c1b1 f6g5 b1d3 c8b8 b4c3 f7f6 d3a6 b8d8 e1d1 d8c8 c3e1 c8c2 a2a4 e5e4 a6b5 d7b5 a4b5 f6f5 e1a5 e4e3 f2e3 g5e3 g1h1 c2c5 b3b4 c5b5 d6d7 e3g5 d1d4 b5e5 h1h2 e5e4 d7d8q g5d8 d4d8 g7g6 d8d2 g6f7 b4b5 e4e5 d2b2 e5e7 b5b6 e7b7 a5d2 h6h5 d2e3 f7e6 b2b5 e6f6 h2g1 f6g6 g1f2 g6f6 f2f3 h5h4 f3f4 f6g6 b5b4 g6f6 e3f2 f6e6 f2h4 e6d6 h4f2 d6d5 f4f3 d5c6 b4b1 b7g7 g2g3 g7h7 h3h4 h7b7 b1c1 c6d5 c1e1 b7g7 e1c1 g7b7 c1c7 b7b8");
    // //
    // orchestra_director.handle_command("go", "depth 10");


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



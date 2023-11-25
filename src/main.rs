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
    let mut orchestra_director = orchestradirector::new_orchestra_director();
    orchestra_director.handle_command("position", "fen 2r5/r5k1/6pp/8/8/8/3Q1PPP/6K1 w - - 0 1");
    // orchestra_director.handle_command("position", "fen 6N1/8/8/8/4K3/8/5k2/8 b - - 0 5");
    // orchestra_director.handle_command("position", "fen 6Q1/8/8/8/4K3/8/5k2/8 b - - 0 6");
    //
    orchestra_director.handle_command("go", "depth 8");


    exit(0);
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



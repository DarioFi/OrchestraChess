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
    // //
    // let mut orchestra_director = orchestradirector::new_orchestra_director();
    // orchestra_director.handle_command("position", "fen r1r3k1/p3ppb1/1npp1npp/7q/2PP2bP/2N1P1P1/1PQN1PB1/R1B2RK1 b - - 1 15");
    // orchestra_director.handle_command("position", "startpos");
    // orchestra_director.handle_command("go", "depth 0");

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



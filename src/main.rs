mod r#move;
mod constants;
mod board;
mod helpers;
mod magic;
mod orchestradirector;
mod engine;
mod evaluation;
mod zobrist;
mod zobrist_impl;
mod timer;





use std::{io, process::exit};
use crate::board::from_fen;

fn main() {

    // todo: known bug
    // 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1
    // at depth 6 this position gives perft = 11030082 but it should give 11030083

    let mut orchestra_director = orchestradirector::new_orchestra_director();

    orchestra_director.eng.board = from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    orchestra_director.handle_command("position", "startpos moves e2e4");
    orchestra_director.handle_command("go", "movetime 5000");
    exit(0);
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



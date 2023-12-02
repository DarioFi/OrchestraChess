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

use std::io;
use std::process::exit;

// use jemallocator::Jemalloc;
//
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let mut orchestra_director = orchestradirector::new_orchestra_director();
    // orchestra_director.handle_command("position", "startpos");
    // orchestra_director.handle_command("go", "depth 6");
    // exit(0);

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

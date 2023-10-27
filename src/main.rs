mod r#move;
mod constants;
mod board;
mod helpers;
mod magic;
mod orchestradirector;
mod engine;
mod evaluation;

use std::time::{Instant, Duration};
use orchestradirector::OrchestraDirector;

use std::fmt::Debug;
use std::io;

fn main() {

    // todo: known bug
    // 8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1
    // at depth 6 this position gives perft = 11030082 but it should give 11030083

    //
    // let start_time = Instant::now(); // Record the start time
    //
    // let x: u64 = 4503599627370560;
    // println!("{:064b}", x);
    //
    // let fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1";
    //
    let mut orchestra_director = orchestradirector::new_orchestra_director();
    //
    // orchestra_director.eng.board = board::from_fen(fen);
    //
    // let end_time = Instant::now(); // Record the end time
    // let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
    // println!("Process time: {:?}", elapsed_time);
    //
    // let start_time = Instant::now();
    //
    // let depth = 5;
    // let x = orchestra_director.eng.perft(depth, depth);
    // println!("Perft: {:}", x);
    //
    // let end_time = Instant::now(); // Record the end time
    // let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
    // println!("Process time: {:?}", elapsed_time);
    //
    // println!("Knodes per second {:}", x as f64 / elapsed_time.as_secs_f64() / 1000.0);


    // }
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



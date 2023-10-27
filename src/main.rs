mod r#move;
mod constants;
mod board;
mod helpers;
mod magic;
mod orchestradirector;
mod engine;

use std::time::{Instant, Duration};
use orchestradirector::OrchestraDirector;

use std::fmt::Debug;
use std::io;
use crate::constants::PieceType;
use crate::helpers::lsb;
use crate::magic::{square_num_to_bitboard, hash_on_mask, inverse_hash_on_mask};
use crate::r#move::{Move};

fn main() {
    let start_time = Instant::now(); // Record the start time

    let x: u64 = 4503599627370560;
    println!("{:064b}", x);

    // let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let fen = "\
   8/8/3p4/KPp3kr/1R2Pp2/8/6P1/8 w - c6 0 3";

    let mut orchestra_director = orchestradirector::new_orchestra_director();

    orchestra_director.eng.board = board::from_fen(fen);

    let end_time = Instant::now(); // Record the end time
    let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
    println!("Process time: {:?}", elapsed_time);

    let start_time = Instant::now();

    let depth = 1;
    let x = orchestra_director.eng.perft(depth, depth);
    println!("Perft: {:}", x);

    let end_time = Instant::now(); // Record the end time
    let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
    println!("Process time: {:?}", elapsed_time);

    println!("Knodes per second {:}", x as f64 / elapsed_time.as_secs_f64() / 1000.0);


    let fen = "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1";

    // orchestra_director.eng.board = board::from_fen(fen);
    // let m = orchestra_director.eng.board.move_from_str("d7d5");
    // orchestra_director.eng.board.make_move(Move{
    //     start_square: 51,
    //     end_square: 35,
    //     piece_moved: PieceType::Pawn,
    //     piece_captured: PieceType::Null,
    //     promotion: PieceType::Null,
    //     is_castling: false,
    //     is_en_passant: false,
    // });
    // let ms = orchestra_director.eng.board.generate_moves(false);
    // println!("{:}", ms.len());
    //
    // for a in ms{
    //     println!("{:}", a.to_string());
    // }
    // loop {
    //     let mut message = String::new();
    //
    //     // Read input from the user
    //     io::stdin().read_line(&mut message).expect("Failed to read input");
    //
    //     let message = message.trim(); // Remove trailing newline
    //
    //     // Split the message into command and options
    //     let mut parts = message.splitn(2, ' ');
    //     let command = parts.next().unwrap_or("");
    //     let options = parts.next().unwrap_or("");
    //
    //     // Call a function to handle the command
    //     orchestra_director.handle_command(command, options);
    // }
}



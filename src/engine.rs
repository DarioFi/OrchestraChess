use std::backtrace::Backtrace;
use crate::board::Board;
use crate::r#move::Move;

pub struct Engine {
    pub(crate) board: Board,
}

pub fn new_engine(board: Board) -> Engine {
    Engine {
        board,
    }
}

impl Engine {
    pub fn search(&mut self, depth: i32) -> Move {
        todo!()
    }

    pub fn perft(&mut self, depth: i32, sd: i32) -> u64 {
        let moves = self.board.generate_moves(false);

        if depth == 0 {
            return 1;
            // return moves.len() as u64;
        }
        let mut counter: u64 = 0;

        for mov in moves {
            self.board.make_move(mov);
            let c = self.perft(depth - 1, sd);
            counter += c;
            self.board.unmake_move();

            if depth == sd {
                println!("{}: {}", mov.to_string(), c);
                // println!("a");
            }
        }

        counter
    }
}

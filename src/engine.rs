use std::backtrace::Backtrace;
use crate::board::Board;
use crate::constants::{COLOR, PieceType};
use crate::r#move::{Move, create_move};


const MATING_SCORE: i32 = 250000;

fn null_move() -> Move {
    create_move(0, 0, PieceType::Null, PieceType::Null, PieceType::Null, false, false)
}

pub struct Engine {
    pub(crate) board: Board,
    node_count: u64,
}

pub fn new_engine(board: Board) -> Engine {
    Engine {
        board,
        node_count: 0,
    }
}


fn move_score(m: &Move) -> i32 {
    match m.piece_captured {
        PieceType::Null => { 0 }
        PieceType::Pawn => { 100 }
        PieceType::Knight => { 300 }
        PieceType::Bishop => { 330 }
        PieceType::Rook => { 500 }
        PieceType::Queen => { 900 }
        PieceType::King => { 2500 }
    }
}

impl Engine {
    pub fn search(&mut self, depth: u64) -> (i32, Move) {
        let mut best_move = null_move();
        let mut score = 0;
        self.node_count = 0;

        let start_time = std::time::Instant::now();
        for dep in 1..(depth + 1) {
            let x = self.negamax(dep, -MATING_SCORE, MATING_SCORE, self.board.color_to_move);
            best_move = x.1;
            score = x.0;
            println!("info depth {} score cp {} nodes {} pv {}", dep, score, self.node_count, best_move.to_uci_string());
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
            if elapsed_time.as_secs_f64() > 0.5 {
                break;
            }
        }

        return (score, best_move);
    }

    fn negamax(&mut self, depth: u64, alpha: i32, beta: i32, color: COLOR) -> (i32, Move) {
        self.node_count += 1;

        // todo: 3fold

        // todo: trasposition

        if depth == 0 {
            return (self.board.static_evaluation(), create_move(0, 0, PieceType::Null, PieceType::Null, PieceType::Null, false, false));
        }

        let mut best_move = null_move();
        let mut best_score = -MATING_SCORE;
        let mut alpha = alpha;

        let moves = self.board.generate_moves(false);

        if moves.len() == 0 {
            if self.board.is_check() {
                return (-MATING_SCORE + depth as i32, null_move());
            } else {
                return (0, null_move());
            }
        }

        for mov in moves {
            self.board.make_move(mov);
            let score = -self.negamax(depth - 1, -beta, -alpha, color.flip()).0;
            self.board.unmake_move();

            if score > best_score {
                if score > MATING_SCORE - 100 {
                    best_score = score - 1;
                    best_move = mov;
                } else {
                    best_score = score;
                    best_move = mov;
                }
            }
            if best_score > alpha {
                alpha = best_score;
            }

            if alpha >= beta {
                break;
            }
        }

        return (best_score, best_move);
    }

    pub fn perft(&mut self, depth: i32, sd: i32) -> u64 {
        let mut moves = self.board.generate_moves(false);

        if depth == 1 {
            // return 1;
            return moves.len() as u64;
        }
        let mut counter: u64 = 0;

        moves.sort_by_key(move_score);
        for mov in moves {
            self.board.make_move(mov);
            let c = self.perft(depth - 1, sd);
            counter += c;
            self.board.unmake_move();

            if depth == sd {
                println!("{}: {}", mov.to_uci_string(), c);
                // println!("a");
            }
        }

        counter
    }
}

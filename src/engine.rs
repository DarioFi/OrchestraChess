use std::backtrace::Backtrace;
use std::cmp::max;
use crate::board::Board;
use crate::constants::{COLOR, PieceType};
use crate::r#move::{Move, create_move};
use std::collections::HashMap;

const MATING_SCORE: i32 = 250000;

fn null_move() -> Move {
    create_move(0, 0, PieceType::Null, PieceType::Null, PieceType::Null, false, false)
}

pub struct Engine {
    pub(crate) board: Board,
    node_count: u64,
    max_selective: i32,
    transposition_table: HashMap<u64, (u64, i32, Move, bool)>,
}

pub fn new_engine(board: Board) -> Engine {
    Engine {
        board,
        node_count: 0,
        max_selective: 0,
        transposition_table: HashMap::new(),
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
        self.transposition_table = HashMap::new();

        let mut best_move = null_move();
        let mut score = 0;
        self.node_count = 0;
        self.max_selective = 0;

        let start_time = std::time::Instant::now();
        for dep in 1..(depth + 1) {
            let x = self.negamax(dep, -MATING_SCORE, MATING_SCORE, self.board.color_to_move);
            best_move = x.1;
            score = x.0;
            let score_string;
            if score > MATING_SCORE - 100 {
                score_string = format!("mate {}", (MATING_SCORE - score) / 2);
            } else if score < -MATING_SCORE + 100 {
                score_string = format!("mate {}", (-MATING_SCORE - score) / 2);
            } else {
                score_string = format!("cp {}", score);
            }
            println!("info depth {} seldepth {} score {} nodes {} pv {}", dep, self.max_selective, score_string, self.node_count, best_move.to_uci_string());

            println!("Transposition table size {}", self.transposition_table.len());
            let end_time = std::time::Instant::now();
            let elapsed_time = end_time.duration_since(start_time); // Calculate the elapsed time
            if elapsed_time.as_secs_f64() > 0.1 {
                break;
            }
        }

        return (score, best_move);
    }

    fn negamax(&mut self, depth: u64, alpha: i32, beta: i32, color: COLOR) -> (i32, Move) {
        self.node_count += 1;

        let hash = self.board.zobrist.hash;
        if self.transposition_table.contains_key(&hash) {
            let result = self.transposition_table[&hash];
            let old_depth = result.0;
            let old_score = result.1;
            let old_move = result.2;
            let old_exact = result.3;

            if old_depth >= depth {
                if old_exact || old_score >= beta {
                    return (old_score, old_move);
                }
            }
        }

        if self.board.is_3fold() {
            return (0, null_move());
        }

        if depth == 0 {
            let eval = self.quiescence_search(alpha, beta, 0);
            return (eval, null_move());
        }

        let moves;
        moves = self.board.generate_moves(false);

        let mut best_move = null_move();
        let mut best_score = -MATING_SCORE;
        let mut alpha = alpha;
        let mut is_exact = true;


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
                is_exact = false;
                break;
            }
        }

        self.update_transposition_table(depth, best_score, best_move, is_exact);

        return (best_score, best_move);
    }

    fn quiescence_search(&mut self, alpha: i32, beta: i32, depth: i32) -> i32 {
        self.max_selective = max(self.max_selective, depth);
        let mut eval = self.board.static_evaluation();
        let mut alpha = alpha;
        if eval > beta {
            return beta;
        }
        if eval > alpha {
            alpha = eval;
        }

        let mut moves = self.board.generate_moves(true);
        moves.sort_by_key(move_score);

        for mov in moves.iter() {
            self.board.make_move(*mov);
            eval = -self.quiescence_search(-beta, -alpha, depth + 1);
            self.board.unmake_move();

            if eval >= beta {
                return beta;
            }
            if eval > alpha {
                alpha = eval;
            }
        }
        alpha
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
            }
        }

        counter
    }

    pub fn benchmark_perf(&mut self, depth: u32) {
        let now = std::time::Instant::now();
        let res = self.perft(depth as i32, depth as i32);
        let elapsed = now.elapsed();
        let elapsed_ms = elapsed.as_millis();
        println!("{} nodes in {} ms", res, elapsed_ms);
        println!("{} Mps", res as f64 / (elapsed_ms as f64 / 1000.0) / 1000_000.0);
    }
}


impl Engine {
    fn update_transposition_table(&mut self, depth: u64, score: i32, mov: Move, is_exact: bool) {
        let hash = self.board.zobrist.hash;

        if self.transposition_table.contains_key(&hash) {
            let res = self.transposition_table[&hash];
            if res.0 >= depth {
                return;
            }
        }

        // self.transposition_table[&hash] = (depth, score, mov, is_exact);
        self.transposition_table.insert(hash, (depth, score, mov, is_exact));
    }

    fn query_transposition_table(&self) {
        let hash = self.board.zobrist.hash;

        if self.transposition_table.contains_key(&hash) {}
    }
}
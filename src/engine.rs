use std::cmp::max;
use std::os::unix::fs::MetadataExt;
use std::vec;
use crate::board::Board;
use crate::constants::{COLOR, PieceType};
use crate::r#move::{Move, create_move};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use crate::book::OpeningBook;
use crate::timer::start_timer_maximum_allocable;


const MATING_SCORE: i32 = 250000;
const BOOK_DEPTH: u64 = 20;
const BOOK_FILE: &str = "tree.json";
const TIME_ELAPSED_ITERATIVE_DEEPENING: f32 = 0.5;

fn null_move() -> Move {
    create_move(0, 0, PieceType::Null, PieceType::Null, PieceType::Null, false, false)
}

pub struct Engine {
    pub(crate) board: Board,
    node_count: u64,
    max_selective: i32,
    transposition_table: HashMap<u64, (u64, i32, Move, bool)>,
    pub book: OpeningBook,
    pub position_loaded: String,
    pub moves_loaded: String,
}

pub fn new_engine(board: Board) -> Engine {
    Engine {
        board,
        node_count: 0,
        max_selective: 0,
        transposition_table: HashMap::new(),
        book: OpeningBook::new(BOOK_FILE),
        position_loaded: "".to_string(),
        moves_loaded: "".to_string(),
    }
}


impl Engine {
    pub fn search(&mut self, depth: u64, max_time: u128) -> (i32, Move) {
        if self.position_loaded == "startposa" {
            let moves = self.moves_loaded.split(" ");
            if moves.collect::<Vec<_>>().len() < BOOK_DEPTH as usize {
                let mov = self.book.query(&self.moves_loaded);
                if mov.is_some() {
                    return (0, self.board.move_from_str(&mov.unwrap()));
                }
            }
        }


        self.transposition_table = HashMap::new();

        let start_time = std::time::Instant::now();
        let stop_hook = Arc::new(Mutex::new(false));

        if max_time != 0 {
            start_timer_maximum_allocable(max_time, stop_hook.clone());
        }

        let mut best_move = null_move();
        let mut score = 0;
        self.node_count = 0;
        self.max_selective = 0;

        println!("max time {}", max_time);

        for dep_it in 1..(depth + 1) {
            let dep = dep_it * 2;
            let x = self.principal_variation(dep, -MATING_SCORE, MATING_SCORE, &stop_hook, true);

            if *stop_hook.lock().unwrap() {
                break;
            }

            best_move = x.1;
            score = x.0;
            let score_string;
            if score > MATING_SCORE - 100 {
                score_string = format!("mate {}", (MATING_SCORE - score + 1) / 2);
            } else if score < -MATING_SCORE + 100 {
                score_string = format!("mate {}", (-MATING_SCORE - score - 1) / 2);
            } else {
                score_string = format!("cp {}", score);
            }
            println!("info depth {} seldepth {} score {} nodes {} pv {}", dep, self.max_selective, score_string, self.node_count, best_move.to_uci_string());
            // println!("Transposition table size {}", self.transposition_table.len());

            // logic to check whether to stop the search
            let elapsed = start_time.elapsed().as_millis();

            if max_time != 0 && elapsed as f32 > max_time as f32 * TIME_ELAPSED_ITERATIVE_DEEPENING {
                println!("Stopped iterative deepening early because previous iteration already long enough");
                *stop_hook.lock().unwrap() = true;
                break;
            }
        }

        println!("Time elapsed {}", start_time.elapsed().as_millis());
//2.23
        return (score, best_move);
    }

    /* pub fn negamax(&mut self, depth: u64, alpha: i32, beta: i32, stop_search: &Arc<Mutex<bool>>) -> (i32, Move) {
    //     if *stop_search.lock().unwrap() {
    //         return (0, null_move());
    //     }


    //     let hash = self.board.zobrist.hash;
    //     if self.board.is_3fold() {
    //         return (0, null_move());
    //     }

    //     if self.transposition_table.contains_key(&hash) {
    //         let result = self.transposition_table[&hash];
    //         let old_depth = result.0;
    //         let old_score = result.1;
    //         let old_move = result.2;
    //         let old_exact = result.3;

    //         if old_depth >= depth {
    //             if old_exact || old_score >= beta {
    //                 if old_score > MATING_SCORE - 100 {
    //                     return (old_score - 1, old_move);
    //                 }
    //                 return (old_score, old_move);
    //             }
    //         }
    //     }
    //     self.node_count += 1;


    //     if depth == 0 {
    //         // let eval = self.quiescence_search(alpha, beta, 0);
    //         let eval = self.board.static_evaluation();
    //         return (eval, null_move());
    //     }

    //     let mut moves;
    //     moves = self.board.generate_moves(false);
    //     moves.sort();

    //     let mut best_move = null_move();
    //     let mut best_score = -MATING_SCORE;
    //     let mut alpha = alpha;
    //     let mut is_exact = true;


    //     if moves.len() == 0 {
    //         if self.board.is_check() {
    //             return (-MATING_SCORE, null_move());
    //         } else {
    //             return (0, null_move());
    //         }
    //     }

    //     for mov in moves.iter() {
    //         let mov = *mov;
    //         self.board.make_move(mov);
    //         let score = -self.negamax(depth - 1, -beta, -alpha,  &stop_search).0;
    //         self.board.unmake_move();

    //         if score > best_score {
    //             if score > MATING_SCORE - 100 {
    //                 best_score = score - 1;
    //                 best_move = mov;
    //             } else {
    //                 best_score = score;
    //                 best_move = mov;
    //             }
    //         }
    //         if best_score > alpha {
    //             alpha = best_score;
    //         }

    //         if alpha >= beta {
    //             is_exact = false;
    //             break;
    //         }
    //     }

    //     self.update_transposition_table(depth, best_score, best_move, is_exact);

    //     return (best_score, best_move);
    // }
    */


    fn principal_variation(&mut self, depth: u64, alpha: i32, beta: i32, stop_search: &Arc<Mutex<bool>>, genuine: bool) -> (i32, Move) {

        if *stop_search.lock().unwrap() {
            return (0, null_move());
        }
        self.node_count += 1;

        let mut old_move: Move = null_move();
        let hash = self.board.zobrist.hash;
        if self.transposition_table.contains_key(&hash) {
            let result = self.transposition_table[&hash];
            let old_depth = result.0;
            let old_score = result.1;
            old_move = result.2;
            let old_exact = result.3;

            if old_depth >= depth {
                if old_exact || old_score >= beta {
                    if old_score > MATING_SCORE - 100 {
                        return (old_score - 1, old_move);
                    }
                    return (old_score, old_move);
                }
            }
        }

        //todo: check this
        if self.board.is_3fold() {
            return (0, null_move());
        }

        let mut moves;
        moves = self.board.generate_moves(false);
        if moves.len() == 0 {
            if self.board.is_check() {
                return (-MATING_SCORE, null_move());
            } else {
                return (0, null_move());
            }
        }
        if depth == 0 {
            // let eval = self.quiescence_search(alpha, beta, 0);
            let eval = self.board.static_evaluation();
            return (eval, null_move());
        }

        let mut best_move = null_move();
        let mut best_score = -MATING_SCORE;
        let mut alpha = alpha;
        let mut is_exact = true;
        let mut is_first = true;
        let mut alpha_overwritten = false;
        moves.sort();
        if old_move != null_move() {
            // moves.add_priority_move(old_move);
        }
        for mov in moves.iter() {
            let mov = *mov;
            self.board.make_move(mov);
            let mut score;
            if is_first {
                is_first = false;
                score = -self.principal_variation(depth - 1, -beta, -alpha, &stop_search, true).0;
                best_move = mov;
            } else {
                score = -self.principal_variation(depth - 1, -alpha - 1, -alpha, &stop_search, false).0;
                if (alpha < score && score < beta) {
                    score = -self.principal_variation(depth - 1, -beta, -alpha, &stop_search, true).0;
                }
            }
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
                alpha_overwritten = true;
                alpha = best_score;
            }
            if alpha >= beta {
                is_exact = false;
                break;
            }
        }

        if genuine || alpha_overwritten {
            self.update_transposition_table(depth, best_score, best_move, is_exact);
        }

        (best_score, best_move)
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
        moves.sort();

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


    pub fn benchmark_perf(&mut self, depth: u32) {
        let now = std::time::Instant::now();
        let res = self.board.perft(depth as i32, depth as i32, true);
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
}
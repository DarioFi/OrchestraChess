use crate::board::Board;
use crate::muve::{Move, null_move};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::book::OpeningBook;
use crate::timer::start_timer_maximum_allocable;
use std::cmp::{max, min};
use crate::move_heuristic::MovesHeuristic;


pub const MATING_SCORE: i32 = 250000;
const BOOK_DEPTH: u64 = 12;
const BOOK_FILE: &str = "tree.json";
const TIME_ELAPSED_ITERATIVE_DEEPENING: f32 = 0.5;

pub struct Engine {
    pub(crate) board: Board,
    pub node_count: u64,
    max_selective: i32,
    transposition_table: HashMap<u64, (u64, i32, Move, bool)>,
    pub book: OpeningBook,
    pub position_loaded: String,
    pub moves_loaded: String,
    pub move_heuristic: MovesHeuristic,
    curr_max_depth: i32,
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
        move_heuristic: MovesHeuristic::new(),
        curr_max_depth: 0,
    }
}


impl Engine {
    pub fn search(&mut self, depth: i32, max_time: u128) -> (i32, Move) {
        if self.position_loaded == "startpos" {
            let moves = self.moves_loaded.split(" ");
            if moves.collect::<Vec<_>>().len() < BOOK_DEPTH as usize {
                let mov = self.book.query(&self.moves_loaded);
                if mov.is_some() {
                    return (0, self.board.move_from_str(&mov.unwrap()));
                }
            }
        }

        if depth == 0 {
            let score = self.board.static_evaluation(false);
            println!("info score cp {}", score);
            return (score, null_move());
        }

        self.transposition_table = HashMap::new();
        self.move_heuristic = MovesHeuristic::new();

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

        for dep in 1..(depth + 1) {
            self.curr_max_depth = dep;
            let dep = dep as u64;
            let pv_result = self.principal_variation(0, dep, -MATING_SCORE, MATING_SCORE, &stop_hook, true, true);

            if *stop_hook.lock().unwrap() {
                if pv_result.1 == null_move() {
                    // println!("Wasted iterative");
                    break;
                } else {
                    // println!("Used iterative");
                    best_move = pv_result.1;
                    score = pv_result.0;
                    break;
                }
            }

            best_move = pv_result.1;
            score = pv_result.0;
            let score_string;
            if score > MATING_SCORE - 100 {
                score_string = format!("mate {}", (MATING_SCORE - score + 3) / 2);
                // score_string = format!("mate {}", score);
            } else if score < -MATING_SCORE + 100 {
                score_string = format!("mate {}", -(score + MATING_SCORE + 2) / 2);
            } else {
                score_string = format!("cp {}", score);
            }
            println!("info depth {} seldepth {} score {} nodes {} pv {}", dep, self.max_selective, score_string, self.node_count, best_move.to_uci_string());
            // println!("Transposition table size {}", self.transposition_table.len());

            // logic to check whether to stop the search
            let elapsed = start_time.elapsed().as_millis();

            if max_time != 0 && elapsed as f32 > max_time as f32 * TIME_ELAPSED_ITERATIVE_DEEPENING {
                // println!("Stopped iterative deepening early because previous iteration already long enough");
                *stop_hook.lock().unwrap() = true;
                break;
            }
        }

        println!("Time elapsed {}", start_time.elapsed().as_millis());
        return (score, best_move);
    }


    fn principal_variation(&mut self, distance_from_root: u64, depth: u64, alpha: i32, beta: i32, stop_search: &Arc<Mutex<bool>>, genuine: bool, is_root: bool) -> (i32, Move) {
        if *stop_search.lock().unwrap() {
            return (0, null_move());
        }
        self.node_count += 1;
        let retrieved_hash: bool;

        if !is_root && self.board.rule50 >= 100 {
            return (0, null_move());
        }

        if (!is_root) && self.board.is_3fold() {
            return (0, null_move());
        }

        let mut old_move: Move = null_move();
        let hash = self.board.zobrist.hash;
        if self.transposition_table.contains_key(&hash) {
            retrieved_hash = true;

            let result = self.transposition_table[&hash];
            let old_depth = result.0;
            let old_score = result.1;
            old_move = result.2;
            let old_exact = result.3;

            if old_depth >= depth {
                if old_exact || old_score >= beta {
                    let mut is_3_fold = false;

                    if !(old_move.is_en_passant || old_move.is_castling) {

                        // dirty
                        let old_hash = self.board.zobrist.hash;
                        self.board.update_hash(old_move);
                        let new_hash = self.board.zobrist.hash;
                        self.board.zobrist.hash = old_hash;

                        let stack_size = self.board.zobrist_stack.len();
                        let moves_to_see = min(stack_size, self.board.rule50 as usize);
                        if moves_to_see > 1 {
                            let start = (stack_size - moves_to_see) + (stack_size - moves_to_see) % 2;
                            if self.board.zobrist_stack[start..].iter().step_by(2).filter(|x| **x == new_hash).count() >= 2 {
                                is_3_fold = true;
                            }
                        }
                    }

                    if !is_3_fold {
                        if old_score > MATING_SCORE - 100 {
                            return (old_score - 1, old_move);
                        }
                        return (old_score, old_move);
                    }
                }
            }
        } else {
            retrieved_hash = false;
        }


        if depth == 0 {
            let eval = self.quiescence_search(-MATING_SCORE, MATING_SCORE, 0);
            return (eval, null_move());
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


        let mut best_move = null_move();
        let mut best_score = -MATING_SCORE;
        let mut alpha = alpha;
        let mut is_exact = true;
        let mut has_first_not_been_completed = true;
        let mut alpha_overwritten = false;

        moves.sort(&self.move_heuristic, distance_from_root as usize, self.board.moves_stack.last(), &self.board.utility);
        if retrieved_hash {
            moves.add_priority_move(old_move);
        }

        for mov in moves.iter() {
            let mov = *mov;

            self.board.make_move(mov);
            let mut score;

            if has_first_not_been_completed {
                score = -self.principal_variation(distance_from_root + 1, depth - 1, -beta, -alpha, &stop_search, genuine, false).0;
                best_move = mov;
            } else {
                score = -self.principal_variation(distance_from_root + 1, depth - 1, -alpha - 1, -alpha, &stop_search, false, false).0;
                if alpha < score && score < beta {
                    score = -self.principal_variation(distance_from_root + 1, depth - 1, -beta, -alpha, &stop_search, genuine, false).0;
                }
            }


            self.board.unmake_move();

            if *stop_search.lock().unwrap() {
                if !has_first_not_been_completed && is_root && retrieved_hash {
                    assert_ne!(best_move, null_move());
                    return (best_score, best_move);
                } else {
                    return (0, null_move());
                }
            }

            has_first_not_been_completed = false;

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
                self.move_heuristic.failed_high(depth, distance_from_root as usize, mov, self.board.moves_stack.last());
                is_exact = false;
                break;
            }
            // self.move_heuristic.tested_move(mov, depth, distance_from_root as usize);
        }

        if genuine || alpha_overwritten {
            self.update_transposition_table(depth, best_score, best_move, is_exact);
        }

        (best_score, best_move)
    }


    pub(crate) fn quiescence_search(&mut self, mut alpha: i32, beta: i32, depth: i32) -> i32 {
        self.max_selective = max(self.max_selective, depth);

        if self.board.is_3fold() {
            return 0;
        }

        let mut moves = self.board.generate_moves(true);
        if self.board.is_check() {
            moves = self.board.generate_moves(false);
            if moves.len() == 0 {
                return -MATING_SCORE;
            }
        }

        let eval = self.board.static_evaluation(true);
        if depth >= self.curr_max_depth {
            return eval;
        }

        if eval > beta {
            return eval;
        }
        if eval > alpha {
            alpha = eval;
        }


        // we are ignoring stalemates in quiescence search!

        moves.sort_quiescence(&self.board.utility);
        for mov in moves.iter() {
            self.board.make_move(*mov);
            let eval = -self.quiescence_search(-beta, -alpha, depth + 1);
            self.board.unmake_move();

            if eval > alpha {
                alpha = eval;
            }

            if alpha >= beta {
                break;
            }
        }
        alpha
    }


    #[allow(dead_code)]
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

        self.transposition_table.insert(hash, (depth, score, mov, is_exact));
    }
}

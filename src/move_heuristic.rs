use crate::muve::Move;
use crate::utils::{PieceType};

pub struct MovesHeuristic {
    killers: Vec<(Option<Move>, Option<Move>)>,
    counter_move: [[Option<Move>; 64]; 7],
    history_score: [[[i32; 64]; 7]; 2],
    // bf_score: [[[i32; 64]; 7]; 2],
}

const INIT_LEN: usize = 100;
// const SCALE: i32 = 1000;

impl MovesHeuristic {
    pub fn new() -> MovesHeuristic {
        MovesHeuristic {
            killers: Vec::with_capacity(INIT_LEN),
            counter_move: [[None; 64]; 7],
            history_score: [[[0; 64]; 7]; 2],
            // bf_score: [[[0; 64]; 7]; 2],
        }
    }

    pub(crate) fn get_killers(&self, depth: usize) -> Vec<Move> {
        if self.killers.len() > depth {
            let (k1, k2) = self.killers[depth];
            let mut res = vec![];
            if k1.is_some() {
                res.push(k1.unwrap());
            }
            if k2.is_some() {
                res.push(k2.unwrap());
            }
            res
        } else {
            vec![]
        }
    }

    pub(crate) fn failed_high(&mut self, depth: u64, distance_from_root: usize, m: Move, prev_m: Option<&Move>) {
        if m.piece_captured != PieceType::Null {
            return;
        }

        // killers
        if self.killers.len() <= distance_from_root {
            self.killers.resize(distance_from_root + 1, (None, None));
        }
        let k1 = self.killers[distance_from_root].0;
        self.killers[distance_from_root] = (Some(m), k1);

        // countermove
        if prev_m.is_some() {
            let prev_m = prev_m.unwrap();
            self.counter_move[prev_m.piece_moved as usize][prev_m.end_square as usize] = Option::from(m);
        }

        // history heuristic
        self.history_score[distance_from_root % 2][m.piece_moved as usize][m.end_square as usize] += (depth * depth) as i32;
    }

    // pub fn tested_move(&mut self, m: Move, depth: u64, distance_from_root: usize) {
        // this feature does not show any improvement so it is temporary disabled
        // if m.piece_captured != PieceType::Null {
        //     return;
        // }
        //
        // history heuristic
        // self.bf_score[distance_from_root % 2][m.piece_moved as usize][m.end_square as usize] += (depth * depth) as i32;
    // }

    pub fn get_counter_move(&self, m: &Move) -> Option<Move> {
        self.counter_move[m.piece_moved as usize][m.end_square as usize]
    }

    pub fn get_history_score(&self, m: &Move, distance_from_root: usize) -> i32 {
        self.history_score[distance_from_root % 2][m.piece_moved as usize][m.end_square as usize]
        // self.history_score[distance_from_root % 2][m.piece_moved as usize][m.end_square as usize] * SCALE / (self.bf_score[distance_from_root % 2][m.piece_moved as usize][m.end_square as usize] + 1)
    }
}


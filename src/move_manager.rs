use crate::utils::{PieceType};
use crate::engine::MATING_SCORE;
use crate::move_heuristic::MovesHeuristic;
use crate::muve::Move;
use crate::board::UtilityBitBoards;
use crate::magic::square_num_to_bitboard;

fn piece_score(piece: PieceType) -> i32 {
    match piece {
        PieceType::Null => { 0 }
        PieceType::Pawn => { 100 }
        PieceType::Knight => { 300 }
        PieceType::Bishop => { 330 }
        PieceType::Rook => { 500 }
        PieceType::Queen => { 900 }
        PieceType::King => { MATING_SCORE }
    }
}

fn attacked_score(utility_bit_boards: &UtilityBitBoards, sq: u8) -> i32 {
    let x = square_num_to_bitboard(sq);
    let mut score = 0;
    if utility_bit_boards.opponent_pawn_attacks & x != 0 {
        score -= 300;
    }
    score
}

fn move_score_capture(m: &Move, utilities: &UtilityBitBoards) -> i32 {
    piece_score(m.piece_captured) + piece_score(m.promotion) - piece_score(m.piece_moved)
        + attacked_score(utilities, m.end_square)
}

pub struct MoveManager {
    pub(crate) quiet_moves: Vec<Move>,
    capture_moves: Vec<Move>,
    priority_moves: Vec<Move>,
    killers: Vec<Move>,
}

const MOVE_CAP: usize = 15;

impl MoveManager {
    pub fn new() -> MoveManager {
        MoveManager { quiet_moves: Vec::with_capacity(MOVE_CAP), capture_moves: Vec::with_capacity(MOVE_CAP), priority_moves: Vec::with_capacity(MOVE_CAP), killers: Vec::with_capacity(3) }
    }

    pub fn add_move(&mut self, m: Move) {
        if m.piece_captured != PieceType::Null || m.is_en_passant || m.promotion != PieceType::Null {
            self.capture_moves.push(m);
        } else {
            self.quiet_moves.push(m);
        }
    }

    // must be called in reversed order of priority
    pub fn add_priority_move(&mut self, m: Move) {
        let l = self.len();
        self.quiet_moves.retain(|x| *x != m);
        self.capture_moves.retain(|x| *x != m);

        if self.len() != l {
            self.priority_moves.push(m);
        }
    }

    pub fn sort(&mut self, moves_heuristic: &MovesHeuristic, distance_from_root: usize, prev_move: Option<&Move>, utilities: &UtilityBitBoards) {
        self.capture_moves.sort_by_key(|a| -move_score_capture(a, utilities));

        self.sort_killers(moves_heuristic.get_killers(distance_from_root));


        // history heuristic
        self.quiet_moves.sort_by_key(|a| -moves_heuristic.get_history_score(a, distance_from_root));

        // countermove heuristic
        if prev_move.is_some() {
            let m = moves_heuristic.get_counter_move(prev_move.unwrap());
            if m.is_some() {
                let m = m.unwrap();
                if m.piece_captured == PieceType::Null {
                    for i in 0..self.quiet_moves.len() {
                        if self.quiet_moves[i] == m {
                            (self.quiet_moves[i], self.quiet_moves[0]) = (self.quiet_moves[0], self.quiet_moves[i]);
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn sort_killers(&mut self, killers: Vec<Move>) {
        self.killers.clear();
        for k in killers {
            for i in 0..self.quiet_moves.len() {
                if self.quiet_moves[i] == k {
                    self.quiet_moves.remove(i);
                    self.killers.push(k);
                    break;
                }
            }
        }
    }

    pub fn sort_quiescence(&mut self, utilities: &UtilityBitBoards) {
        self.capture_moves.sort_by_key(|a| -move_score_capture(a, utilities));
    }

    pub fn iter(&self) -> impl Iterator<Item=&Move> {
        return self.priority_moves.iter().chain(self.capture_moves.iter().chain(self.killers.iter()).chain(self.quiet_moves.iter()));
    }

    pub fn len(&self) -> usize {
        return self.quiet_moves.len() + self.killers.len() + self.capture_moves.len() + self.priority_moves.len();
    }
}


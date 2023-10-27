use crate::board::{Board, PieceBitBoards};
use crate::constants::COLOR;
use crate::helpers::{remove_lsb, lsb, pop_count};


impl Board {
    pub fn static_evaluation(&self) -> i32 {
        let mut score = 0;
        score += self.evaluate_pos(self.my_pieces);
        score -= self.evaluate_pos(self.opponent_pieces);
        score
    }


    fn evaluate_pos(&self, pbb: PieceBitBoards) -> i32 {
        let mut score = 0;

        score += self.evaluate_pawns(pbb.pawns);
        score += self.evaluate_knights(pbb.knight);
        score += self.evaluate_bishops(pbb.bishop);
        score += self.evaluate_rooks(pbb.rook);
        score += self.evaluate_queens(pbb.queen);

        score
    }
    fn evaluate_pawns(&self, p0: u64) -> i32 {
        (pop_count(p0) * 100) as i32
    }
    fn evaluate_knights(&self, p0: u64) -> i32 {
        (pop_count(p0) * 300) as i32
    }
    fn evaluate_bishops(&self, p0: u64) -> i32 {
        (pop_count(p0) * 330) as i32
    }
    fn evaluate_rooks(&self, p0: u64) -> i32 {
        (pop_count(p0) * 500) as i32
    }
    fn evaluate_queens(&self, p0: u64) -> i32 {
        (pop_count(p0) * 900) as i32
    }
}
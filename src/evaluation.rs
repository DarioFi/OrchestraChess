
use crate::board::{Board, PieceBitBoards};
use crate::constants::{COLOR};
use crate::helpers::{remove_lsb, lsb, pop_count};

const PAWN_SCORES: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    5, 10, 10, -20, -20, 10, 10, 5,
    5, -5, -10, 0, 0, -10, -5, 5,
    0, 0, 0, 20, 20, 0, 0, 0,
    5, 5, 10, 25, 25, 10, 5, 5,
    10, 10, 20, 30, 30, 20, 10, 10,
    50, 50, 50, 50, 50, 50, 50, 50,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const PAWN_SCORES_ENDGAME: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0,
    10, 10, 10, 10, 10, 10, 10, 10,
    20, 20, 20, 20, 20, 20, 20, 20,
    40, 40, 40, 40, 40, 40, 40, 40,
    100, 100, 100, 100, 100, 100, 100, 100,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const KNIGHT_SCORES: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20, 0, 0, 0, 0, -20, -40,
    -30, 0, 10, 15, 15, 10, 0, -30,
    -30, 5, 15, 20, 20, 15, 5, -30,
    -30, 0, 15, 20, 20, 15, 0, -30,
    -30, 5, 10, 15, 15, 10, 5, -30,
    -40, -20, 0, 5, 5, 0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_SCORES: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 5, 10, 10, 5, 0, -10,
    -10, 5, 5, 10, 10, 5, 5, -10,
    -10, 0, 10, 10, 10, 10, 0, -10,
    -10, 10, 10, 10, 10, 10, 10, -10,
    -10, 5, 0, 0, 0, 0, 5, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];
const KNIGHT_SCORES_ENDGAME: [i32; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50,
    -40, -20, 0, 0, 0, 0, -20, -40,
    -30, 0, 10, 15, 15, 10, 0, -30,
    -30, 5, 15, 20, 20, 15, 5, -30,
    -30, 0, 15, 20, 20, 15, 0, -30,
    -30, 5, 10, 15, 15, 10, 5, -30,
    -40, -20, 0, 5, 5, 0, -20, -40,
    -50, -40, -30, -30, -30, -30, -40, -50,
];

const BISHOP_SCORES_ENDGAME: [i32; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20,
    -10, 5, 0, 0, 0, 0, 5, -10,
    -10, 0, 5, 10, 10, 5, 0, -10,
    -10, 5, 5, 10, 10, 5, 5, -10,
    -10, 0, 10, 10, 10, 10, 0, -10,
    -10, 10, 10, 10, 10, 10, 10, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -20, -10, -10, -10, -10, -10, -10, -20,
];

const QUEEN_SCORES_ENDGAME: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5,
    0, 0, 5, 5, 5, 5, 0, -5,
    -10, 5, 5, 5, 5, 5, 0, -10,
    -10, 0, 5, 0, 0, 0, 0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20,
];


const QUEEN_SCORES: [i32; 64] = [
    -20, -10, -10, -5, -5, -10, 0, -20,
    -10, 0, 0, 0, 0, -5, 0, -10,
    -10, 0, 5, 5, 5, 5, 0, 0,
    0, 0, 5, 5, 5, 5, 0, -5,
    -5, 0, 5, 5, 5, 5, 0, -10,
    -10, 0, 5, 0, 0, 0, 0, -10,
    -10, 0, 0, 0, 0, 0, 0, -10,
    -20, -10, -10, -5, -5, -10, -10, -20,
];
const KING_SCORES: [i32; 64] = [
    20, 30, 10, 0, 0, 10, 30, 20,
    20, 20, -5, -5, -5, -5, 20, 20,
    -10, -20, -20, -20, -20, -20, -20, -10,
    -20, -30, -30, -40, -40, -30, -30, -20,
    -30, -40, -40, -50, -50, -40, -40, -30,
    -40, -50, -50, -60, -60, -50, -50, -40,
    -60, -60, -60, -60, -60, -60, -60, -60,
    -80, -70, -70, -70, -70, -70, -70, -80,
];

const KING_SCORES_ENDGAME: [i32; 64] = [
    -50, -40, -30, -20, -20, -30, -40, -50,
    -40, -30, -20, -10, -10, -20, -30, -40,
    -30, -20, -10, 20, 20, -10, -20, -30,
    -20, -10, 20, 30, 30, 20, -10, -20,
    -20, -10, 20, 30, 30, 20, -10, -20,
    -30, -20, -10, 20, 20, -10, -20, -30,
    -40, -30, -20, -10, -10, -20, -30, -40,
    -50, -40, -30, -20, -20, -30, -40, -50,
];

const ROOK_SCORES: [i32; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0,
    5, 0, 0, 0, 0, 0, 0, 5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 10, 10, 10, 10, 10, 10, -5,
    0, 0, 0, 0, 0, 0, 0, 0,
];

const ROOK_SCORES_ENDGAME: [i32; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0,
    5, 10, 10, 10, 10, 10, 10, 5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    -5, 0, 0, 0, 0, 0, 0, -5,
    0, 0, 0, 5, 5, 0, 0, 0,
];


impl Board {
    pub fn static_evaluation(&self) -> i32 {
        let mut score = 0;
        let opponent_occupancy = self.opponent_pieces.pawns | self.opponent_pieces.knight | self.opponent_pieces.bishop | self.opponent_pieces.rook | self.opponent_pieces.queen | self.opponent_pieces.king;
        let my_occupancy = self.my_pieces.pawns | self.my_pieces.knight | self.my_pieces.bishop | self.my_pieces.rook | self.my_pieces.queen | self.my_pieces.king;
        let all_occupancy = my_occupancy | opponent_occupancy;

        let count = pop_count(all_occupancy) as i32;
        let end_gameness = (32 - count) / 32;

        let is_white = match self.color_to_move {
            COLOR::WHITE => { true }
            COLOR::BLACK => { false }
        };
        score += self.evaluate_pos(self.my_pieces, end_gameness, is_white);
        score -= self.evaluate_pos(self.opponent_pieces, end_gameness, !is_white);
        score
    }


    fn evaluate_pos(&self, pbb: PieceBitBoards, end_gameness: i32, is_white: bool) -> i32 {
        let mut score = 0;

        score += self.ev_piece(pbb.pawns, end_gameness, is_white, 100, &PAWN_SCORES, &PAWN_SCORES_ENDGAME);
        score += self.ev_piece(pbb.bishop, end_gameness, is_white, 330, &BISHOP_SCORES, &BISHOP_SCORES_ENDGAME);
        score += self.ev_piece(pbb.knight, end_gameness, is_white, 300, &KNIGHT_SCORES, &KNIGHT_SCORES_ENDGAME);
        score += self.ev_piece(pbb.rook, end_gameness, is_white, 500, &ROOK_SCORES, &ROOK_SCORES_ENDGAME);
        score += self.ev_piece(pbb.queen, end_gameness, is_white, 900, &QUEEN_SCORES, &QUEEN_SCORES_ENDGAME);
        score += self.ev_piece(pbb.king, end_gameness, is_white, 2500, &KING_SCORES, &KING_SCORES_ENDGAME);

        score
    }
    fn ev_piece(&self, square_mask: u64, end_gameness: i32, is_white: bool, piece_val: i32, table: &[i32; 64], table_endgame: &[i32; 64]) -> i32 {
        let mut x = square_mask;
        let mut score = 0;

        while x != 0 {
            let sq = lsb(x);
            x = remove_lsb(x);

            // Determine the square index (0 to 63)
            let _sq_index = if is_white {
                sq
            } else {
                63 - sq
            };

            score += piece_val;
            score += table[sq as usize] * (1 - end_gameness) + table_endgame[sq as usize] * end_gameness;
            // Adjust the score based on endgame condition
        }

        score
    }
}
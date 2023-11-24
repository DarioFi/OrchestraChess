use crate::board::Board;
use crate::magic::int_to_coord;
use crate::utils::{COLOR, lsb, PieceType, remove_lsb};
use crate::utils::PieceType::Pawn;

const DIMS: usize = 2560;
const PSQ_BUCKETS: usize = 8;

const PAWN_IND: usize = 0;
const KNIGHT_IND: usize = 1;
const BISHOP_IND: usize = 2;
const ROOK_IND: usize = 3;
const QUEEN_IND: usize = 4;
const KING_IND: usize = 5;

pub fn make_index(piece_index: usize, is_mine: usize, mut piece_square: usize, king_square: usize) -> usize {
    /*
    p_idx = piece_type * 2 + piece_color
    halfkp_idx = piece_square + (p_idx + king_square * 11) * 64
    */
    let mut king_file = king_square % 8;
    let king_rank = king_square / 8;
    let mut piece_file = piece_square % 8;
    let piece_rank = piece_square / 8;
    if king_file < 4 {
        king_file ^= 7;
        piece_file ^= 7;
    }
    let p_idx = piece_index * 2 + is_mine;
    let new_piece_id = piece_rank * 8 + piece_file;
    let new_king_id = 31 - (king_rank * 4 + (king_file - 4));
    let halfkp_idx = new_piece_id + p_idx * 64 + new_king_id * 11 * 64;
    // println!("{}", halfkp_idx);
    return halfkp_idx;
}

fn change_perspective(sq: u8) -> u8 {
    let file = sq % 8;
    let rank = sq / 8;
    return (7 - rank) * 8 + file;
}

impl Board {
    pub fn refresh_accumulator(&mut self) {
        let mut my_acc: [i16; DIMS] = self.nnue.feature_transformer.get_bias();
        let mut opp_acc: [i16; DIMS] = self.nnue.feature_transformer.get_bias();

        let mut my_psq_acc = [0_i32; PSQ_BUCKETS];
        let mut opp_psq_acc = [0_i32; PSQ_BUCKETS];


        let mut my_king_square = lsb(self.my_pieces.king);
        let mut opp_king_square = lsb(self.opponent_pieces.king);
        match self.color_to_move {
            COLOR::WHITE => {
                opp_king_square = change_perspective(opp_king_square);
            }
            COLOR::BLACK => {
                my_king_square = change_perspective(my_king_square);
            }
        }

        // opp_king_square = get_king_index(opp_king_square);
        // my_king_square = get_king_index(my_king_square);

        let am_i_white = self.color_to_move == COLOR::WHITE;

        self.update_specific_piece(self.my_pieces.pawn, PAWN_IND, my_king_square, opp_king_square, am_i_white, true, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.my_pieces.knight, KNIGHT_IND, my_king_square, opp_king_square, am_i_white, true, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.my_pieces.bishop, BISHOP_IND, my_king_square, opp_king_square, am_i_white, true, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.my_pieces.rook, ROOK_IND, my_king_square, opp_king_square, am_i_white, true, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.my_pieces.queen, QUEEN_IND, my_king_square, opp_king_square, am_i_white, true, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);

        self.update_specific_piece(self.opponent_pieces.pawn, PAWN_IND, my_king_square, opp_king_square, am_i_white, false, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.opponent_pieces.knight, KNIGHT_IND, my_king_square, opp_king_square, am_i_white, false, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.opponent_pieces.bishop, BISHOP_IND, my_king_square, opp_king_square, am_i_white, false, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.opponent_pieces.rook, ROOK_IND, my_king_square, opp_king_square, am_i_white, false, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);
        self.update_specific_piece(self.opponent_pieces.queen, QUEEN_IND, my_king_square, opp_king_square, am_i_white, false, &mut my_acc, &mut opp_acc, &mut my_psq_acc, &mut opp_psq_acc);

        let ks = lsb(self.my_pieces.king);
        let sq_my_perspective;
        let sq_opp_perspective;
        match am_i_white {
            true => {
                sq_my_perspective = ks;
                sq_opp_perspective = change_perspective(ks);
            }
            false => {
                sq_my_perspective = change_perspective(ks);
                sq_opp_perspective = ks;
            }
        }

        let my_index = make_index(KING_IND, 0, sq_my_perspective as usize, my_king_square as usize);
        let opp_index = make_index(KING_IND, 0, sq_opp_perspective as usize, opp_king_square as usize);

        self.nnue.feature_transformer.add_to_accumulator(my_index, &mut my_acc);
        self.nnue.feature_transformer.add_to_accumulator(opp_index, &mut opp_acc);

        let ks = lsb(self.opponent_pieces.king);
        let sq_my_perspective;
        let sq_opp_perspective;
        match am_i_white {
            true => {
                sq_my_perspective = ks;
                sq_opp_perspective = change_perspective(ks);
            }
            false => {
                sq_my_perspective = change_perspective(ks);
                sq_opp_perspective = ks;
            }
        }

        let my_index = make_index(KING_IND, 0, sq_my_perspective as usize, my_king_square as usize);
        let opp_index = make_index(KING_IND, 0, sq_opp_perspective as usize, opp_king_square as usize);

        self.nnue.feature_transformer.add_to_accumulator(my_index, &mut my_acc);
        self.nnue.feature_transformer.add_to_accumulator(opp_index, &mut opp_acc);
        self.nnue.feature_transformer.add_to_accumulator_psq(my_index, &mut my_psq_acc);
        self.nnue.feature_transformer.add_to_accumulator_psq(opp_index, &mut opp_psq_acc);

        self.nnue.feature_transformer.my_acc_stack.push(my_acc);
        self.nnue.feature_transformer.opp_acc_stack.push(opp_acc);
        self.nnue.feature_transformer.my_psq_acc_stack.push(my_psq_acc);
        self.nnue.feature_transformer.opp_psq_acc_stack.push(opp_psq_acc);
    }

    fn update_specific_piece(&mut self, bitmap: u64, INDEX: usize, my_ks: u8, opp_ks: u8, am_i_white: bool, is_my_bitmap: bool, my_acc: &mut [i16; DIMS], opp_acc: &mut [i16; DIMS], my_psq: &mut [i32; 8], opp_psq: &mut [i32; 8]) {
        let mut pieces = bitmap;
        let mut sq = lsb(pieces);
        pieces = remove_lsb(pieces);
        while sq != 64 {
            let sq_my_perspective;
            let sq_opp_perspective;

            match am_i_white {
                true => {
                    sq_my_perspective = sq;
                    sq_opp_perspective = change_perspective(sq);
                }
                false => {
                    sq_my_perspective = change_perspective(sq);
                    sq_opp_perspective = sq;
                }
            }

            let my_index = make_index(INDEX, (!is_my_bitmap) as usize, sq_my_perspective as usize, my_ks as usize);
            let opp_index = make_index(INDEX, is_my_bitmap as usize, sq_opp_perspective as usize, opp_ks as usize);

            self.nnue.feature_transformer.add_to_accumulator(my_index, my_acc);
            self.nnue.feature_transformer.add_to_accumulator(opp_index, opp_acc);
            self.nnue.feature_transformer.add_to_accumulator_psq(my_index, my_psq);
            self.nnue.feature_transformer.add_to_accumulator_psq(opp_index, opp_psq);

            sq = lsb(pieces);
            pieces = remove_lsb(pieces);
        }
    }

    pub fn clean_accumulator(&mut self) {
        self.nnue.feature_transformer.my_acc_stack = vec![];
        self.nnue.feature_transformer.opp_acc_stack = vec![];
        self.nnue.feature_transformer.my_psq_acc_stack = vec![];
        self.nnue.feature_transformer.opp_psq_acc_stack = vec![];
    }
}

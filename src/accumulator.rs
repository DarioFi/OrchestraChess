use crate::board::Board;
use crate::magic::int_to_coord;
use crate::muve::Move;
use crate::utils::{COLOR, lsb, PieceType, remove_lsb};

const DIMS: usize = 2560;
const PSQ_BUCKETS: usize = 8;

const PAWN_IND: usize = 0;
const KNIGHT_IND: usize = 1;
const BISHOP_IND: usize = 2;
const ROOK_IND: usize = 3;
const QUEEN_IND: usize = 4;
const KING_IND: usize = 5;


// expects king and piece squares to be in pov but not yet reflected horizontally.
pub fn make_index(piece_index: usize, is_opp: usize, mut piece_square: usize, king_square: usize) -> usize {
    let mut king_file = king_square % 8;
    let king_rank = king_square / 8;
    let mut piece_file = piece_square % 8;
    let piece_rank = piece_square / 8;
    if king_file < 4 {
        king_file ^= 7;
        piece_file ^= 7;
    }
    let p_idx = piece_index * 2 + is_opp;
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
    // recomputes the accumulator and pushes to the stacks.
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
        self.nnue.feature_transformer.add_to_accumulator_psq(my_index, &mut my_psq_acc);
        self.nnue.feature_transformer.add_to_accumulator_psq(opp_index, &mut opp_psq_acc);

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

    // assume neither moved piece nor captured piece is a king.
    pub fn update_accumulator_on_simple_move(&mut self, mov: Move) {
        let from_sq = mov.start_square;
        let to_sq = mov.end_square;
        let piece_captured = mov.piece_captured; // PieceType::None if no piece captured
        let piece_moved_idx = mov.piece_moved as usize - 1;
        
        // get king squares, each from its own perspective
        let mut my_ks = lsb(self.my_pieces.king);
        let mut opp_ks = lsb(self.opponent_pieces.king);
        match self.color_to_move {
            COLOR::WHITE => {
                opp_ks = change_perspective(opp_ks);
            }
            COLOR::BLACK => {
                my_ks = change_perspective(my_ks);
            }
        }
        
        // we read the opponent's stack because who "me" is has changed since last move when it was pushed.
        let mut my_acc = self.nnue.feature_transformer.opp_acc_stack.last().unwrap().clone(); 
        let mut opp_acc = self.nnue.feature_transformer.my_acc_stack.last().unwrap().clone();
        let mut my_psq = self.nnue.feature_transformer.opp_psq_acc_stack.last().unwrap().clone();
        let mut opp_psq = self.nnue.feature_transformer.my_psq_acc_stack.last().unwrap().clone();
        
        // adjust perspectives
        let from_sq_my_perspective;
        let from_sq_opp_perspective;
        let to_sq_my_perspective;
        let to_sq_opp_perspective;
        match self.color_to_move {
            COLOR::WHITE => {
                from_sq_my_perspective = from_sq;
                to_sq_my_perspective = to_sq;
                from_sq_opp_perspective = change_perspective(from_sq);
                to_sq_opp_perspective = change_perspective(to_sq);
            },
            COLOR::BLACK => {
                from_sq_my_perspective = change_perspective(from_sq);
                to_sq_my_perspective = change_perspective(to_sq);
                from_sq_opp_perspective = from_sq;
                to_sq_opp_perspective = to_sq;
            },
        }

        // moved piece
        let my_from_index = make_index(piece_moved_idx, 1, from_sq_my_perspective as usize, my_ks as usize);
        let opp_from_index = make_index(piece_moved_idx, 0, from_sq_opp_perspective as usize, opp_ks as usize);
        let my_to_index = make_index(piece_moved_idx, 1, to_sq_my_perspective as usize, my_ks as usize);
        let opp_to_index = make_index(piece_moved_idx, 0, to_sq_opp_perspective as usize, opp_ks as usize);
        self.nnue.feature_transformer.subtract_from_accumulator(my_from_index, &mut my_acc);
        self.nnue.feature_transformer.subtract_from_accumulator(opp_from_index, &mut opp_acc);
        self.nnue.feature_transformer.add_to_accumulator(my_to_index, &mut my_acc);
        self.nnue.feature_transformer.add_to_accumulator(opp_to_index, &mut opp_acc);
        self.nnue.feature_transformer.subtract_from_accumulator_psq(my_from_index, &mut my_psq);
        self.nnue.feature_transformer.subtract_from_accumulator_psq(opp_from_index, &mut opp_psq);
        self.nnue.feature_transformer.add_to_accumulator_psq(my_to_index, &mut my_psq);
        self.nnue.feature_transformer.add_to_accumulator_psq(opp_to_index, &mut opp_psq);

        // captured piece
        if piece_captured != PieceType::Null {
            let piece_captured_idx = piece_captured as usize - 1;
            let my_captured_index = make_index(piece_captured_idx, 0, to_sq_my_perspective as usize, my_ks as usize);
            let opp_captured_index = make_index(piece_captured_idx, 1, to_sq_opp_perspective as usize, opp_ks as usize);
            self.nnue.feature_transformer.subtract_from_accumulator(my_captured_index, &mut my_acc);
            self.nnue.feature_transformer.subtract_from_accumulator(opp_captured_index, &mut opp_acc);
            self.nnue.feature_transformer.subtract_from_accumulator_psq(my_captured_index, &mut my_psq);
            self.nnue.feature_transformer.subtract_from_accumulator_psq(opp_captured_index, &mut opp_psq);
        }

        // push to stacks
        self.nnue.feature_transformer.my_acc_stack.push(my_acc);
        self.nnue.feature_transformer.opp_acc_stack.push(opp_acc);
        self.nnue.feature_transformer.my_psq_acc_stack.push(my_psq);
        self.nnue.feature_transformer.opp_psq_acc_stack.push(opp_psq);
        
    }

    // the swapping me and you logic is in the incremental update.
    pub fn update_accumulator_on_unmake(&mut self){
        self.nnue.feature_transformer.my_acc_stack.pop();
        self.nnue.feature_transformer.opp_acc_stack.pop();
        self.nnue.feature_transformer.my_psq_acc_stack.pop();
        self.nnue.feature_transformer.opp_psq_acc_stack.pop();
    }

    pub fn update_accumulator_on_make(&mut self, mov: Move){
        match mov.piece_moved {
            PieceType::King => self.refresh_accumulator(),
            PieceType::Pawn => {
                if mov.is_en_passant{
                    self.refresh_accumulator();
                } else if mov.promotion != PieceType::Null {
                    self.refresh_accumulator();
                } else {
                    self.update_accumulator_on_simple_move(mov);
                }
            },
            PieceType::Knight | PieceType::Bishop | PieceType::Rook | PieceType::Queen => self.update_accumulator_on_simple_move(mov),
            PieceType::Null => panic!("Mucho problemo"),

        }
    }
}

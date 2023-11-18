use crate::board::{empty_utility_bitboards, UtilityBitBoards};
use crate::constants::PieceType;
use crate::magic::square_num_to_bitboard;
use crate::r#move::Move;

fn move_score(m: &Move) -> i32 {
    let val;
    if m.is_check {
        val = 300;
    } else {
        val = 0;
    }
    match m.piece_captured {
        PieceType::Null => { 0 + val }
        PieceType::Pawn => { 100 + val }
        PieceType::Knight => { 300 + val }
        PieceType::Bishop => { 330 + val }
        PieceType::Rook => { 500 + val }
        PieceType::Queen => { 900 + val }
        PieceType::King => { 2500 + val }
    }
}

// todo: OBstack arena allocator
pub struct MoveManager {
    quiet_moves: Vec<Move>,
    capture_moves: Vec<Move>,
    priority_moves: Vec<Move>,
    checking_pawn: u64,
    checking_knight: u64,
    checking_bishop: u64,
    checking_rook: u64,
    checking_queen: u64,
    pub forcing: bool,
}

impl MoveManager {
    pub fn new() -> MoveManager {
        MoveManager { quiet_moves: vec![], capture_moves: vec![], priority_moves: vec![], checking_pawn: 0, checking_knight: 0, checking_bishop: 0, checking_rook: 0, checking_queen: 0, forcing: false }
    }

    pub fn add_move(&mut self, mut m: Move) {
        let check: bool;

        match m.piece_moved {
            PieceType::Pawn => { check = (square_num_to_bitboard(m.end_square) & self.checking_pawn) != 0 }
            PieceType::Knight => { check = (square_num_to_bitboard(m.end_square) & self.checking_knight) != 0 }
            PieceType::Bishop => { check = (square_num_to_bitboard(m.end_square) & self.checking_bishop) != 0 }
            PieceType::Rook => { check = (square_num_to_bitboard(m.end_square) & self.checking_rook) != 0 }
            PieceType::Queen => { check = (square_num_to_bitboard(m.end_square) & self.checking_queen) != 0 }
            PieceType::King => { check = false }
            _ => {
                debug_assert!(false);
                check = false
            }
        }

        m.is_check = check;

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

    pub fn update_checkers(&mut self, utility_bit_boards: &UtilityBitBoards) {
        self.checking_pawn = utility_bit_boards.checking_pawn;
        self.checking_knight = utility_bit_boards.checking_knight;
        self.checking_bishop = utility_bit_boards.checking_bishop;
        self.checking_rook = utility_bit_boards.checking_rook;
        self.checking_queen = utility_bit_boards.checking_queen;
    }

    pub fn sort(&mut self) {
        self.quiet_moves.sort_by_key(|a| -move_score(a));
        self.capture_moves.sort_by_key(|a| -move_score(a));
    }

    pub fn iter(&self) -> impl Iterator<Item=&Move> {
        return self.priority_moves.iter().chain(self.capture_moves.iter().chain(self.quiet_moves.iter()));
    }

    pub fn len(&self) -> usize {
        return self.quiet_moves.len() + self.capture_moves.len() + self.priority_moves.len();
    }
}


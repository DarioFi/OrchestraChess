use crate::constants::PieceType;
use crate::r#move::Move;

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

pub struct MoveManager {
    quite_moves: Vec<Move>,
    capture_moves: Vec<Move>,
}

impl MoveManager {
    pub fn new() -> MoveManager {
        MoveManager { quite_moves: vec![], capture_moves: vec![] }
    }

    pub fn add_move(&mut self, m: Move) {
        if m.piece_captured != PieceType::Null || m.is_en_passant || m.promotion != PieceType::Null {
            self.capture_moves.push(m);
        } else {
            self.quite_moves.push(m);
        }
    }

    pub fn sort(&mut self) {
        self.quite_moves.sort_by_key(|a| -move_score(a));
        self.capture_moves.sort_by_key(|a| -move_score(a));
    }

    pub fn iter(&self) -> impl Iterator<Item=&Move> {
        return self.capture_moves.iter().chain(self.quite_moves.iter());
    }

    pub fn len(&self) -> usize {
        return self.quite_moves.len() + self.capture_moves.len();
    }
}


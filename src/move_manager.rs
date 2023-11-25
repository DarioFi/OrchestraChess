use crate::utils::PieceType;
use crate::engine::MATING_SCORE;
use crate::muve::Move;


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

fn move_score_capture(m: &Move) -> i32 {
    piece_score(m.piece_captured) + piece_score(m.promotion) - piece_score(m.piece_moved)
}

pub struct MoveManager {
    quiet_moves: Vec<Move>,
    capture_moves: Vec<Move>,
    priority_moves: Vec<Move>,
}

impl MoveManager {
    pub fn new() -> MoveManager {
        MoveManager { quiet_moves: vec![], capture_moves: vec![], priority_moves: vec![] }
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

    pub fn sort(&mut self) {
        // self.quiet_moves.sort_by_key(|a| -move_score(a) );
        self.capture_moves.sort_by_key(|a| -move_score_capture(a));
    }

    pub fn iter(&self) -> impl Iterator<Item=&Move> {
        return self.priority_moves.iter().chain(self.capture_moves.iter().chain(self.quiet_moves.iter()));
    }

    pub fn len(&self) -> usize {
        return self.quiet_moves.len() + self.capture_moves.len() + self.priority_moves.len();
    }
}


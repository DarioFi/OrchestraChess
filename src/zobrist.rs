
pub struct ZobristHashHandler {
    pub table: [[u64; 12]; 64],
    pub black_to_move: u64,
    pub hash: u64,
}


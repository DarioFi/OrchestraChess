#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    Null = 0,
    Pawn = 1,
    Knight = 2,
    Bishop = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
}

impl PieceType {
    pub(crate) fn to_uci_string(&self) -> &str {
        match self {
            PieceType::Pawn => "p",
            PieceType::Knight => "n",
            PieceType::Bishop => "b",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
            _ => panic!("Invalid piece type"),
        }
    }
}


pub const MOVING_PIECES: [PieceType; 4] = [PieceType::Queen, PieceType::Knight, PieceType::Rook, PieceType::Bishop];

#[repr(i8)]
#[derive(Clone, Copy, PartialEq)]
pub enum COLOR {
    WHITE = 1,
    BLACK = -1,
}

impl COLOR {
    pub(crate) fn flip(&self) -> COLOR {
        match self {
            COLOR::WHITE => COLOR::BLACK,
            COLOR::BLACK => COLOR::WHITE,
        }
    }
}


pub const MASK_ONES: u64 = 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
pub const MASK_ZEROES: u64 = 0b0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_0000_;
use crate::utils::PieceType;


fn index_to_string(index: u8) -> String {
    let row = index / 8;
    let col = index % 8;
    let file = match col {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => panic!("Out of bounds"),
    };

    let rank = match row {
        0 => "1",
        1 => "2",
        2 => "3",
        3 => "4",
        4 => "5",
        5 => "6",
        6 => "7",
        7 => "8",
        _ => panic!("Out of bounds"),
    };

    let res = format!("{}{}", file, rank);
    res
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Move {
    pub(crate) start_square: u8,
    pub(crate) end_square: u8,
    pub(crate) piece_moved: PieceType,
    pub(crate) piece_captured: PieceType,
    pub(crate) promotion: PieceType,
    pub(crate) is_castling: bool,
    pub(crate) is_en_passant: bool,
}

impl Move {
    pub fn to_uci_string(&self) -> String {
        let a = index_to_string(self.start_square);
        let b = index_to_string(self.end_square);
        let c: String;
        if self.promotion != PieceType::Null {
            c = self.promotion.to_uci_string().parse().unwrap();
        } else {
            c = "".parse().unwrap();
        }
        return format!("{}{}{}", a, b, c);
    }
}

pub fn create_move(start_square: u8, end_square: u8, piece_moved: PieceType, piece_captured: PieceType, promotion: PieceType, is_castling: bool, is_en_passant: bool) -> Move {
    Move {
        start_square,
        end_square,
        piece_moved,
        piece_captured,
        promotion,
        is_castling,
        is_en_passant,
    }
}

pub fn null_move() -> Move {
    create_move(0, 0, PieceType::Null, PieceType::Null, PieceType::Null, false, false)
}

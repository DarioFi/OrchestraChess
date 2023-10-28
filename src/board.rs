use std::ops::{Index, IndexMut};
use crate::r#move::{create_move, Move};

use crate::constants::{COLOR, PieceType, MASK_ONES, MASK_ZEROES, MOVING_PIECES};
use crate::constants::COLOR::{WHITE, BLACK};
use crate::helpers::{pop_count, lsb, remove_lsb};
use crate::magic::{Magics, DIRECTIONS, DIAGONAL_DIRS, STRAIGHT_DIRS, square_num_to_bitboard, coord_to_int, coord_bit, new_magic, bitboard_to_square_num, int_to_coord};
use crate::magic::DIRECTIONS::{NE, NW, SE, SW};
use crate::zobrist::ZobristHashHandler;
use crate::zobrist_impl::init_zobrist;

#[derive(Clone, Copy, Debug)]
pub struct PieceBitBoards {
    pub(crate) pawns: u64,
    pub(crate) knight: u64,
    pub(crate) bishop: u64,
    pub(crate) rook: u64,
    pub(crate) queen: u64,
    pub(crate) king: u64,
}

fn empty_piece_bit_boards() -> PieceBitBoards {
    PieceBitBoards {
        pawns: 0,
        knight: 0,
        bishop: 0,
        rook: 0,
        queen: 0,
        king: 0,
    }
}

impl Index<PieceType> for PieceBitBoards {
    type Output = u64;

    fn index(&self, piece_type: PieceType) -> &Self::Output {
        match piece_type {
            PieceType::Pawn => &self.pawns,
            PieceType::Knight => &self.knight,
            PieceType::Bishop => &self.bishop,
            PieceType::Rook => &self.rook,
            PieceType::Queen => &self.queen,
            PieceType::King => &self.king,
            _ => panic!("Invalid piece type"),
        }
    }
}

impl IndexMut<PieceType> for PieceBitBoards {
    fn index_mut(&mut self, piece_type: PieceType) -> &mut Self::Output {
        match piece_type {
            PieceType::Pawn => &mut self.pawns,
            PieceType::Knight => &mut self.knight,
            PieceType::Bishop => &mut self.bishop,
            PieceType::Rook => &mut self.rook,
            PieceType::Queen => &mut self.queen,
            PieceType::King => &mut self.king,
            _ => panic!("Invalid piece type"),
        }
    }
}


struct UtilityBitBoards {
    my_occupancy: u64,
    opponent_occupancy: u64,
    all_occupancy: u64,

    checkers: u64,
    blocker_squares: u64,

    pinned: u64,
    pinned_ns: u64,
    pinned_we: u64,
    pinned_nwse: u64,
    pinned_swne: u64,

    my_attack: u64,
    opponent_attack: u64,

}

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
struct CastlingRights(u8);

const WK: u8 = 1 << 0;
const WQ: u8 = 1 << 1;
const BK: u8 = 1 << 2;
const BQ: u8 = 1 << 3;


const WK_STARTPOS: u64 = coord_bit(0, 4);

const BK_STARTPOS: u64 = coord_bit(7, 4);

const WRK_STARTPOS: u64 = coord_bit(0, 7);
const WRQ_STARTPOS: u64 = coord_bit(0, 0);
const BRK_STARTPOS: u64 = coord_bit(7, 7);
const BRQ_STARTPOS: u64 = coord_bit(7, 0);

const WK_EMPTY: u64 = coord_bit(0, 5) | coord_bit(0, 6);
const WQ_EMPTY: u64 = coord_bit(0, 1) | coord_bit(0, 2) | coord_bit(0, 3);

const WK_ATTACK: u64 = WK_EMPTY;
const WQ_ATTACK: u64 = coord_bit(0, 2) | coord_bit(0, 3);

const BK_EMPTY: u64 = coord_bit(7, 5) | coord_bit(7, 6);
const BQ_EMPTY: u64 = coord_bit(7, 1) | coord_bit(7, 2) | coord_bit(7, 3);

const BK_ATTACK: u64 = BK_EMPTY;
const BQ_ATTACK: u64 = coord_bit(7, 2) | coord_bit(7, 3);


impl CastlingRights {
    fn can_wq(&self) -> bool {
        (self.0 & WQ) != 0
    }

    fn can_wk(&self) -> bool {
        (self.0 & WK) != 0
    }

    fn can_bq(&self) -> bool {
        (self.0 & BQ) != 0
    }

    fn can_bk(&self) -> bool {
        (self.0 & BK) != 0
    }

    fn moved_white_king(&mut self) {
        self.0 &= !WK;
        self.0 &= !WQ;
    }

    fn moved_black_king(&mut self) {
        self.0 &= !BK;
        self.0 &= !BQ;
    }

    fn moved_rook(&mut self, square: u8) {
        match square {
            0 => { self.0 &= !WQ; }
            7 => { self.0 &= !WK; }
            56 => { self.0 &= !BQ; }
            63 => { self.0 &= !BK; }
            _ => {}
        }
    }
}


pub struct Board {
    // todo: for performance reason probably it makes sense to instantiate a single Vec<Move> and
    // then use that one for everything kind of in place, for now we don't do it

    pub(crate) my_pieces: PieceBitBoards,
    pub(crate) opponent_pieces: PieceBitBoards,

    utility: UtilityBitBoards,

    pub(crate) color_to_move: COLOR,
    // todo: check if this is smart, set to 0 if no en passant?? idk just something that cannot give true with random checks, maybe 64
    en_passant_square: u8,
    castling_rights: CastlingRights,

    moves_stack: Vec<Move>,
    zobrist_stack: Vec<u64>,
    en_passant_stack: Vec<u8>,
    castling_stack: Vec<CastlingRights>,

    magics: Magics,

    pub(crate) zobrist: ZobristHashHandler,
}

impl Board {
    pub(crate) fn is_check(&self) -> bool {
        // assume self.generate_moves was ran
        // => self.update_utilities() was ran
        return self.utility.checkers != 0
    }
}


fn square_string_to_int(s: &str) -> u8 {
    // trasform a1 into 0, h8 into 63
    let mut chars = s.chars();
    let file = char_to_int(chars.next().unwrap());
    let rank = chars.next().unwrap().to_digit(10).unwrap() as u8 - 1;
    rank * 8 + file
}

pub fn from_fen(fen: &str) -> Board {
    let mut board = empty_board();

    let parts: Vec<&str> = fen.split(" ").collect();
    let pieces_part = parts[0];
    let mut rank = 7;
    let mut file = 0;
    for p in pieces_part.chars() {
        match p {
            'K' => {
                board.my_pieces.king |= coord_bit(rank, file);
                file += 1;
            }
            'Q' => {
                board.my_pieces.queen |= coord_bit(rank, file);
                file += 1;
            }
            'R' => {
                board.my_pieces.rook |= coord_bit(rank, file);
                file += 1;
            }
            'B' => {
                board.my_pieces.bishop |= coord_bit(rank, file);
                file += 1;
            }
            'N' => {
                board.my_pieces.knight |= coord_bit(rank, file);
                file += 1;
            }
            'P' => {
                board.my_pieces.pawns |= coord_bit(rank, file);
                file += 1;
            }
            'k' => {
                board.opponent_pieces.king |= coord_bit(rank, file);
                file += 1;
            }
            'q' => {
                board.opponent_pieces.queen |= coord_bit(rank, file);
                file += 1;
            }
            'r' => {
                board.opponent_pieces.rook |= coord_bit(rank, file);
                file += 1;
            }
            'b' => {
                board.opponent_pieces.bishop |= coord_bit(rank, file);
                file += 1;
            }
            'n' => {
                board.opponent_pieces.knight |= coord_bit(rank, file);
                file += 1;
            }
            'p' => {
                board.opponent_pieces.pawns |= coord_bit(rank, file);
                file += 1;
            }
            '/' => {
                rank -= 1;
                file = 0;
            }
            _ => { file += p.to_digit(10).unwrap() as u8; }
        }
    }

    let color = parts[1];
    match color {
        "w" => {
            board.color_to_move = WHITE;
        }
        "b" => {
            board.color_to_move = BLACK;
            let mut tmp = board.my_pieces.clone();
            board.my_pieces = board.opponent_pieces.clone();
            board.opponent_pieces = tmp.clone();
        }
        _ => { panic!("Invalid color"); }
    }

    let castling = parts[2];
    for c in castling.chars() {
        match c {
            'K' => { board.castling_rights.0 |= WK; }
            'Q' => { board.castling_rights.0 |= WQ; }
            'k' => { board.castling_rights.0 |= BK; }
            'q' => { board.castling_rights.0 |= BQ; }
            '-' => { board.castling_rights.0 = 0; }
            _ => { panic!("Invalid castling rights"); }
        }
    }

    let en_passant = parts[3];
    if en_passant != "-" {
        board.en_passant_square = square_string_to_int(en_passant);
    } else {
        board.en_passant_square = 0;
    }

    let fifty_move_rule = parts[4];
    let half_move_clock = parts[5];

    board.init_hash();
    board
}


pub fn from_startpos() -> Board {
    from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w QKqk - 0 1")
}

pub fn empty_board() -> Board {
    Board {
        my_pieces: empty_piece_bit_boards(),
        opponent_pieces: empty_piece_bit_boards(),
        utility: empty_utility_bitboards(),
        color_to_move: WHITE,
        en_passant_square: 0,
        castling_rights: CastlingRights(0),
        moves_stack: vec![],
        zobrist_stack: vec![],
        en_passant_stack: vec![],
        castling_stack: vec![],
        magics: new_magic(),
        zobrist: init_zobrist(),
    }
}

fn empty_utility_bitboards() -> UtilityBitBoards {
    UtilityBitBoards {
        my_occupancy: 0,
        opponent_occupancy: 0,
        all_occupancy: 0,
        checkers: 0,
        blocker_squares: 0,
        pinned: 0,
        pinned_ns: 0,
        pinned_we: 0,
        pinned_nwse: 0,
        pinned_swne: 0,
        my_attack: 0,
        opponent_attack: 0,
    }
}

// region Utilities update
impl Board {
    fn update_utilities(&mut self) {
        self.utility.opponent_occupancy = self.opponent_pieces.pawns | self.opponent_pieces.knight | self.opponent_pieces.bishop | self.opponent_pieces.rook | self.opponent_pieces.queen | self.opponent_pieces.king;
        self.utility.my_occupancy = self.my_pieces.pawns | self.my_pieces.knight | self.my_pieces.bishop | self.my_pieces.rook | self.my_pieces.queen | self.my_pieces.king;
        self.utility.all_occupancy = self.utility.my_occupancy | self.utility.opponent_occupancy;

        self.update_pinned_checkers();
        self.update_attacked_squares();
    }

    fn update_pinned_checkers(&mut self) {
        self.utility.checkers = 0;
        self.utility.pinned = 0;

        self.utility.pinned_swne = 0;
        self.utility.pinned_nwse = 0;
        self.utility.pinned_ns = 0;
        self.utility.pinned_we = 0;

        self.utility.blocker_squares = 0;

        let king_square = lsb(self.my_pieces.king);
        // check attacks by knights and pawns

        // knights
        let potential_attackers = self.magics.get_knight_moves(king_square);
        self.utility.checkers |= potential_attackers & self.opponent_pieces.knight;  // if a knight is on an attacker square add it to checkers

        // pawns
        let potential_attackers = self.magics.get_pawn_captures(king_square, self.color_to_move);
        self.utility.checkers |= potential_attackers & self.opponent_pieces.pawns;  // idea is that it is symmetric

        self.utility.blocker_squares = self.utility.checkers;

        // rays
        for dir in STRAIGHT_DIRS {
            let xray_mask = self.magics.get_rays_moves(king_square, self.utility.opponent_occupancy, dir);
            let pierced_pieces = xray_mask & self.utility.my_occupancy;
            if pop_count(pierced_pieces) > 1 {
                continue;
            } else {
                let hitter = xray_mask & self.opponent_pieces.queen | xray_mask & self.opponent_pieces.rook;
                if pop_count(hitter) == 1 { // means we are attacked by something

                    if pop_count(pierced_pieces) == 1 {
                        self.utility.pinned |= pierced_pieces;
                        if dir == DIRECTIONS::N || dir == DIRECTIONS::S {
                            self.utility.pinned_ns |= pierced_pieces;
                        } else {
                            self.utility.pinned_we |= pierced_pieces;
                        }
                    } else {
                        self.utility.checkers |= hitter;
                        self.utility.blocker_squares |= xray_mask;
                        self.utility.blocker_squares |= hitter;
                    }
                }
            }
        }

        for dir in DIAGONAL_DIRS {
            let xray_mask = self.magics.get_rays_moves(king_square, self.utility.opponent_occupancy, dir);
            let pierced_pieces = xray_mask & self.utility.my_occupancy;
            if pop_count(pierced_pieces) > 1 {
                continue;
            } else {
                let hitter = xray_mask & self.opponent_pieces.queen | xray_mask & self.opponent_pieces.bishop;
                if pop_count(hitter) == 1 { // means we are attacked by something

                    if pop_count(pierced_pieces) == 1 {
                        self.utility.pinned |= pierced_pieces;
                        if dir == DIRECTIONS::NW || dir == DIRECTIONS::SE {
                            self.utility.pinned_nwse |= pierced_pieces;
                        } else {
                            self.utility.pinned_swne |= pierced_pieces;
                        }
                    } else {
                        self.utility.checkers |= hitter;
                        self.utility.blocker_squares |= xray_mask;
                        self.utility.blocker_squares |= hitter;
                    }
                }
            }
        }
    }

    fn update_attacked_squares(&mut self) {
        self.utility.opponent_attack = 0;

        // king
        let sq = lsb(self.opponent_pieces.king);
        self.utility.opponent_attack |= self.gen_king_moves_bitboard(sq);

        // pawns
        let mut pawns = self.opponent_pieces.pawns;
        let mut sq = lsb(pawns);
        pawns = remove_lsb(pawns);
        while sq != 64 {
            self.utility.opponent_attack |= self.magics.get_pawn_captures(sq, self.color_to_move.flip());
            sq = lsb(pawns);
            pawns = remove_lsb(pawns);
        }


        // knight
        let mut knigths = self.opponent_pieces.knight;
        let mut sq = lsb(knigths);
        knigths = remove_lsb(knigths);
        while sq != 64 {
            self.utility.opponent_attack |= self.gen_knight_moves_bitboard(sq);
            sq = lsb(knigths);
            knigths = remove_lsb(knigths);
        }

        // from now on we should use a version without the king because we need pierced attacks
        let blockers = self.utility.all_occupancy & !self.my_pieces.king;
        // rook
        let mut rook = self.opponent_pieces.rook;
        let mut sq = lsb(rook);
        rook = remove_lsb(rook);
        while sq != 64 {
            self.utility.opponent_attack |= self.magics.get_rook_moves(sq, blockers);
            sq = lsb(rook);
            rook = remove_lsb(rook);
        }

        // bishop
        let mut bishops = self.opponent_pieces.bishop;
        let mut sq = lsb(bishops);
        bishops = remove_lsb(bishops);
        while sq != 64 {
            self.utility.opponent_attack |= self.magics.get_bishop_moves(sq, blockers);
            sq = lsb(bishops);
            bishops = remove_lsb(bishops);
        }

        // queen
        let mut queen = self.opponent_pieces.queen;
        let mut sq = lsb(queen);
        queen = remove_lsb(queen);
        while sq != 64 {
            self.utility.opponent_attack |= self.magics.get_queen_moves(sq, blockers);
            sq = lsb(queen);
            queen = remove_lsb(queen);
        }
    }
}

// endregion

fn char_to_int(x: char) -> u8 {
    match x {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,

        _ => {
            panic!()
        }
    }
}

impl Board {
    pub fn move_from_str(&self, mov: &str) -> Move {
        let start_square = square_string_to_int(&mov[0..2]);
        let end_square = square_string_to_int(&mov[2..4]);
        let promotion = match mov.len() {
            5 => {
                match mov.chars().nth(4).unwrap() {
                    'q' => PieceType::Queen,
                    'r' => PieceType::Rook,
                    'b' => PieceType::Bishop,
                    'n' => PieceType::Knight,
                    _ => panic!("Invalid promotion"),
                }
            }
            _ => PieceType::Null,
        };
        let piece_moved = self.get_my_piece_on_square(start_square);
        let piece_captured = self.get_opponent_piece_on_square(end_square);
        let is_castling;
        if (piece_moved == PieceType::King) && (end_square as i32 - start_square as i32).abs() == 2 {
            is_castling = true;
        } else {
            is_castling = false
        }
        let is_enpassant;
        if piece_moved == PieceType::Pawn && end_square == self.en_passant_square {
            is_enpassant = true;
        } else {
            is_enpassant = false;
        }

        create_move(
            start_square,
            end_square,
            self.get_my_piece_on_square(start_square),
            self.get_opponent_piece_on_square(end_square),
            promotion,
            is_castling,
            is_enpassant,
        )
    }


    pub fn test(&self) {
        self.my_pieces[PieceType::Bishop];
    }

    pub fn generate_moves(&mut self, captures: bool) -> Vec<Move> {
        self.update_utilities();
        let mut moves: Vec<Move> = Vec::new();
        let ks = lsb(self.my_pieces.king);

        let cap_mask: u64;
        if captures {
            cap_mask = self.utility.opponent_occupancy;
        } else {
            cap_mask = MASK_ONES;
        }

        match pop_count(self.utility.checkers) {
            1 => {
                let land_mask = cap_mask & self.utility.blocker_squares; // move needs to end up on one of these so this will be our land_mask

                moves.extend(self.gen_king_moves(ks, captures));

                // pawns
                moves.extend(self.gen_pawns_legal(land_mask));

                moves.extend(self.gen_moves_land_mask_normal_pieces(land_mask));
            }
            2 => {
                return self.gen_king_moves(ks, captures);
            }
            _ => {
                moves.extend(self.gen_king_moves(ks, captures));
                moves.extend(self.gen_castle());

                // pawns
                moves.extend(self.gen_pawns_legal(cap_mask));

                // normal pieces
                moves.extend(self.gen_moves_land_mask_normal_pieces(cap_mask));
            }
        }

        moves
    }

    fn gen_moves_land_mask_normal_pieces(&mut self, land_mask: u64) -> Vec<Move> {
        // iterate over pieces then run gen_moves using land_mask
        // similarly to how we generate attacks in the update utilities
        let mut moves: Vec<Move> = Vec::new();
        // knights
        let mut knights = self.my_pieces.knight & !self.utility.pinned;
        let mut sq = lsb(knights);
        knights = remove_lsb(knights);
        while sq != 64 {

            // if pinned skip
            if (self.utility.pinned & square_num_to_bitboard(sq)) != 0 {
                sq = lsb(knights);
                knights = remove_lsb(knights);
                continue;
            }

            moves.extend(self.gen_knight_moves(sq, land_mask));
            sq = lsb(knights);
            knights = remove_lsb(knights);
        }

        // rook
        let mut rook = self.my_pieces.rook;
        let mut sq = lsb(rook);
        rook = remove_lsb(rook);
        while sq != 64 {
            moves.extend(self.gen_rook_moves(sq, land_mask));
            sq = lsb(rook);
            rook = remove_lsb(rook);
        }

        // bishop
        let mut bishops = self.my_pieces.bishop;
        let mut sq = lsb(bishops);
        bishops = remove_lsb(bishops);
        while sq != 64 {
            moves.extend(self.gen_bishop_moves(sq, land_mask));
            sq = lsb(bishops);
            bishops = remove_lsb(bishops);
        }

        // queen
        let mut queen = self.my_pieces.queen;
        let mut sq = lsb(queen);
        queen = remove_lsb(queen);
        while sq != 64 {
            moves.extend(self.gen_queen_moves(sq, land_mask));
            sq = lsb(queen);
            queen = remove_lsb(queen);
        }

        moves
    }
    fn get_opponent_piece_on_square(&self, index: u8) -> PieceType {
        let sqntb = square_num_to_bitboard(index);
        if (self.opponent_pieces.pawns & sqntb) != 0 {
            return PieceType::Pawn;
        }
        if (self.opponent_pieces.knight & sqntb) != 0 {
            return PieceType::Knight;
        }
        if (self.opponent_pieces.bishop & sqntb) != 0 {
            return PieceType::Bishop;
        }
        if (self.opponent_pieces.rook & sqntb) != 0 {
            return PieceType::Rook;
        }
        if (self.opponent_pieces.queen & sqntb) != 0 {
            return PieceType::Queen;
        }
        if (self.opponent_pieces.king & sqntb) != 0 {
            return PieceType::King;
        }
        return PieceType::Null;
    }


    pub(crate) fn get_my_piece_on_square(&self, index: u8) -> PieceType {
        let sqntb = square_num_to_bitboard(index);
        if (self.my_pieces.pawns & sqntb) != 0 {
            return PieceType::Pawn;
        }
        if (self.my_pieces.knight & sqntb) != 0 {
            return PieceType::Knight;
        }
        if (self.my_pieces.bishop & sqntb) != 0 {
            return PieceType::Bishop;
        }
        if (self.my_pieces.rook & sqntb) != 0 {
            return PieceType::Rook;
        }
        if (self.my_pieces.queen & sqntb) != 0 {
            return PieceType::Queen;
        }
        if (self.my_pieces.king & sqntb) != 0 {
            return PieceType::King;
        }
        return PieceType::Null;
    }

    pub(crate) fn get_piece_on_square(&self, index: u8) -> (PieceType, COLOR) {
        let x = self.get_my_piece_on_square(index);
        if x != PieceType::Null {
            return (x, self.color_to_move);
        }
        let x = self.get_opponent_piece_on_square(index);
        if x != PieceType::Null {
            return (x, self.color_to_move.flip());
        }

        (PieceType::Null, COLOR::WHITE)
    }

    fn gen_king_moves(&self, square: u8, captures: bool) -> Vec<Move> {
        let legal_moves = self.gen_king_moves_bitboard(square) & (!self.utility.opponent_attack);
        let fin: u64;
        if captures {
            fin = legal_moves & self.utility.opponent_occupancy;
        } else {
            fin = legal_moves;
        }
        self.loop_over_moves_mask(fin, PieceType::King, square)
    }

    fn gen_knight_moves(&self, square: u8, land_mask: u64) -> Vec<Move> {
        let legal_moves = self.gen_knight_moves_bitboard(square) & land_mask;
        return self.loop_over_moves_mask(legal_moves, PieceType::Knight, square);
    }
    fn gen_rook_moves(&self, square: u8, mut land_mask: u64) -> Vec<Move> {
        // compute land mask from pinned logic
        let sqntb = square_num_to_bitboard(square);
        let pinned = (self.utility.pinned & sqntb) != 0;
        if pinned {
            let diag_pin = (self.utility.pinned_nwse & sqntb) != 0;
            let diag2_pin = (self.utility.pinned_swne & sqntb) != 0;
            if diag2_pin || diag_pin {
                return Vec::new();
            }

            let pinned_ns = (self.utility.pinned_ns & sqntb) != 0;

            if pinned_ns {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::N][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::S][square as usize]);
            }
            let pinned_we = (self.utility.pinned_we & sqntb) != 0;
            if pinned_we {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::E][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::W][square as usize]);
            }
        }


        let legal_moves = self.gen_rook_moves_bitboard(square) & land_mask;
        return self.loop_over_moves_mask(legal_moves, PieceType::Rook, square);
    }
    fn gen_bishop_moves(&self, square: u8, mut land_mask: u64) -> Vec<Move> {
        let sqntb = square_num_to_bitboard(square);
        let pinned = (self.utility.pinned & sqntb) != 0;
        if pinned {
            let pinned_ns = (self.utility.pinned_ns & sqntb) != 0;

            if pinned_ns {
                return Vec::new();
            }
            let pinned_we = (self.utility.pinned_we & sqntb) != 0;
            if pinned_we {
                return Vec::new();
            }
            let pinned_nwse = (self.utility.pinned_nwse & sqntb) != 0;
            let pinned_swne = (self.utility.pinned_swne & sqntb) != 0;
            if pinned_nwse {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::NW][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::SE][square as usize]);
            } else if pinned_swne {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::SW][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::NE][square as usize]);
            }
        }

        let legal_moves = self.gen_bishop_moves_bitboard(square) & land_mask;
        return self.loop_over_moves_mask(legal_moves, PieceType::Bishop, square);
    }

    fn gen_queen_moves(&self, square: u8, mut land_mask: u64) -> Vec<Move> {
        let sqntb = square_num_to_bitboard(square);
        let pinned = (self.utility.pinned & sqntb) != 0;
        if pinned {
            let pinned_ns = (self.utility.pinned_ns & sqntb) != 0;
            if pinned_ns {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::N][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::S][square as usize]);
            }
            let pinned_we = (self.utility.pinned_we & sqntb) != 0;
            if pinned_we {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::E][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::W][square as usize]);
            }
            let pinned_nwse = (self.utility.pinned_nwse & sqntb) != 0;
            if pinned_nwse {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::NW][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::SE][square as usize]);
            }
            let pinned_swne = (self.utility.pinned_swne & sqntb) != 0;

            if pinned_swne {
                land_mask &= (self.magics.direction_full_masks[DIRECTIONS::SW][square as usize] |
                    self.magics.direction_full_masks[DIRECTIONS::NE][square as usize]);
            }
        }

        let legal_moves = self.gen_queen_moves_bitboard(square) & land_mask;
        return self.loop_over_moves_mask(legal_moves, PieceType::Queen, square);
    }
    fn loop_over_moves_mask(&self, mut mask: u64, piece_moved: PieceType, start_square: u8) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut end_square = lsb(mask);
        while (end_square != 64) {
            moves.push(Move {
                start_square,
                end_square,
                piece_moved,
                piece_captured: self.get_opponent_piece_on_square(end_square),
                promotion: PieceType::Null,
                is_castling: false,
                is_en_passant: false,
            });
            mask = remove_lsb(mask);
            end_square = lsb(mask);
        }
        moves
    }

    fn gen_queen_moves_bitboard(&self, square: u8) -> u64 {
        self.gen_bishop_moves_bitboard(square) | self.gen_rook_moves_bitboard(square)
    }
    fn gen_bishop_moves_bitboard(&self, square: u8) -> u64 {
        let potential = self.magics.get_bishop_moves(square, self.utility.all_occupancy);
        potential & (!self.utility.my_occupancy)
    }
    fn gen_rook_moves_bitboard(&self, square: u8) -> u64 {
        let potential: u64 = self.magics.get_rook_moves(square, self.utility.all_occupancy);
        potential & (!self.utility.my_occupancy)
    }

    fn gen_pawns_captures_bitboard(&self, square: u8, color: COLOR) -> u64 {
        let potential = self.magics.get_pawn_captures(square, color);
        potential & (!self.utility.all_occupancy)
    }
    fn gen_knight_moves_bitboard(&self, square: u8) -> u64 {
        let potential = self.magics.get_knight_moves(square);
        potential & (!self.utility.my_occupancy)
    }

    fn gen_king_moves_bitboard(&self, square: u8) -> u64 {
        let potential = self.magics.get_king_moves(square);
        potential & (!self.utility.my_occupancy)
    }


    fn gen_pawns_legal(&self, land_mask: u64) -> Vec<Move> {
        // pushes
        let mut moves: Vec<Move> = Vec::new();

        let back_rank: u64;
        let promotion_rank: u64;
        let direction: i32; // signed because it can be subtracted

        match self.color_to_move {
            WHITE => {
                back_rank = 0x000000000000FF00;
                promotion_rank = 0xFF00000000000000;
                direction = 8;
            }
            BLACK => {
                back_rank = 0x00FF000000000000;
                promotion_rank = 0x00000000000000FF;
                direction = -8;
            }
        }

        let mut pawns = self.my_pieces.pawns;
        let mut sq = lsb(pawns);
        pawns = remove_lsb(pawns);
        while sq != 64 {
            let sqntb = square_num_to_bitboard(sq);
            // check push
            let push = square_num_to_bitboard((sq as i32 + direction) as u8);
            let pinned = (self.utility.pinned & sqntb) != 0;
            let pinned_ns = (self.utility.pinned_ns & sqntb) != 0;
            if push != 0
            {
                let can_move_vert;
                if pinned {
                    can_move_vert = pinned_ns;
                } else {
                    can_move_vert = true;
                }


                if (self.utility.all_occupancy & push == 0) && can_move_vert {
                    if push & promotion_rank == 0 {
                        // no promotion
                        if push & land_mask != 0 {
                            moves.push(create_move(
                                sq,
                                (sq as i32 + direction) as u8,
                                PieceType::Pawn,
                                PieceType::Null,
                                PieceType::Null,
                                false,
                                false,
                            ));
                        }

                        if square_num_to_bitboard(sq) & back_rank != 0 {
                            // double push
                            let double_push = square_num_to_bitboard((sq as i32 + 2 * direction) as u8);
                            if (self.utility.all_occupancy & double_push == 0) && (double_push & land_mask != 0) {
                                moves.push(create_move(
                                    sq,
                                    (sq as i32 + 2 * direction) as u8,
                                    PieceType::Pawn,
                                    PieceType::Null,
                                    PieceType::Null,
                                    false,
                                    false,
                                ));
                            }
                        }
                    } else if push & land_mask != 0 {
                        // promotion
                        for piece_promotion in MOVING_PIECES {
                            moves.push(create_move(
                                sq,
                                (sq as i32 + direction) as u8,
                                PieceType::Pawn,
                                PieceType::Null,
                                piece_promotion,
                                false,
                                false,
                            ))
                        };
                    }
                }
            }

            // check capture
            if !pinned_ns {
                let capture_mask_pre_land_mask: u64 = self.magics.get_pawn_captures(sq, self.color_to_move);
                let capture_mask_total = capture_mask_pre_land_mask & land_mask;
                let mut en_passant_mask: u64;
                if self.en_passant_square != 0 {
                    // gotta shift the land mask
                    let enpmld;
                    if direction > 0 {
                        enpmld = land_mask << direction;
                    } else {
                        enpmld = land_mask >> (-direction); // todo verify this right direction
                    }
                    en_passant_mask = square_num_to_bitboard(self.en_passant_square) & capture_mask_pre_land_mask & enpmld;
                } else {
                    en_passant_mask = 0;
                }


                let mut capture_mask = capture_mask_total & self.utility.opponent_occupancy;
                let pinned_we = (self.utility.pinned_we & sqntb) != 0;
                if pinned_we {
                    sq = lsb(pawns);
                    pawns = remove_lsb(pawns);
                    continue;
                }
                let pinned_diag_swne = (self.utility.pinned_swne & sqntb) != 0;
                let pinned_diag_nwse = (self.utility.pinned_nwse & sqntb) != 0;
                if pinned_diag_nwse {
                    capture_mask &= self.magics.get_rays_moves(sq, 0, NW) |
                        self.magics.get_rays_moves(sq, 0, SE);
                }
                if pinned_diag_swne {
                    capture_mask &= self.magics.get_rays_moves(sq, 0, SW) |
                        self.magics.get_rays_moves(sq, 0, NE);
                }

                let mut capture_sq = lsb(capture_mask);
                capture_mask = remove_lsb(capture_mask);
                while capture_sq != 64 {

                    // check if it is pinned and can go diagonal



                    if square_num_to_bitboard(capture_sq) & promotion_rank == 0 {
                        // no promotion
                        moves.push(create_move(
                            sq,
                            capture_sq,
                            PieceType::Pawn,
                            self.get_opponent_piece_on_square(capture_sq),
                            PieceType::Null,
                            false,
                            false,
                        ));
                    } else {
                        // promotion
                        for piece_promotion in MOVING_PIECES {
                            moves.push(create_move(
                                sq,
                                capture_sq,
                                PieceType::Pawn,
                                self.get_opponent_piece_on_square(capture_sq),
                                piece_promotion,
                                false,
                                false,
                            ))
                        };
                    }
                    capture_sq = lsb(capture_mask);
                    capture_mask = remove_lsb(capture_mask);
                }

                if en_passant_mask != 0 {
                    let pinned_diag_swne = (self.utility.pinned_swne & sqntb) != 0;
                    let pinned_diag_nwse = (self.utility.pinned_nwse & sqntb) != 0;
                    if pinned_diag_nwse {
                        en_passant_mask &= self.magics.get_rays_moves(sq, 0, NW) |
                            self.magics.get_rays_moves(sq, 0, SE);
                    }
                    if pinned_diag_swne {
                        en_passant_mask &= self.magics.get_rays_moves(sq, 0, SW) |
                            self.magics.get_rays_moves(sq, 0, NE);
                    }
                    if en_passant_mask != 0 {
                        // means there is an en passant capture
                        let enpmov = create_move(
                            sq,
                            self.en_passant_square,
                            PieceType::Pawn,
                            PieceType::Null,
                            PieceType::Null,
                            false,
                            true,
                        );

                        if self.check_legal_en_passant(&enpmov) {
                            moves.push(enpmov);
                        }
                    }
                }
            }
            sq = lsb(pawns);
            pawns = remove_lsb(pawns);
        }


        moves
    }

    fn gen_castle(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        match self.color_to_move {
            COLOR::WHITE => {
                // if (self.my_pieces.king & WK_STARTPOS) != 0 {
                //     return moves;
                // }

                if self.castling_rights.can_wq() {
                    if self.my_pieces.rook & WRQ_STARTPOS != 0 {
                        if (self.utility.all_occupancy & WQ_EMPTY == 0) & (self.utility.opponent_attack & WQ_ATTACK == 0) {
                            moves.push(Move {
                                start_square: lsb(WK_STARTPOS),
                                end_square: coord_to_int(0, 2),
                                piece_moved: PieceType::King,
                                piece_captured: PieceType::Null,
                                promotion: PieceType::Null,
                                is_castling: true,
                                is_en_passant: false,
                            })
                        }
                    }
                }
                if self.castling_rights.can_wk() {
                    if self.my_pieces.rook & WRK_STARTPOS != 0 {
                        if (self.utility.all_occupancy & WK_EMPTY == 0) & (self.utility.opponent_attack & WK_ATTACK == 0) {
                            moves.push(Move {
                                start_square: lsb(WK_STARTPOS),
                                end_square: coord_to_int(0, 6),
                                piece_moved: PieceType::King,
                                piece_captured: PieceType::Null,
                                promotion: PieceType::Null,
                                is_castling: true,
                                is_en_passant: false,
                            })
                        }
                    }
                }
            }
            COLOR::BLACK => {
                if self.castling_rights.can_bq() {
                    if self.my_pieces.rook & BRQ_STARTPOS != 0 {
                        if (self.utility.all_occupancy & BQ_EMPTY == 0) & (self.utility.opponent_attack & BQ_ATTACK == 0) {
                            moves.push(Move {
                                start_square: lsb(BK_STARTPOS),
                                end_square: coord_to_int(7, 2),
                                piece_moved: PieceType::King,
                                piece_captured: PieceType::Null,
                                promotion: PieceType::Null,
                                is_castling: true,
                                is_en_passant: false,
                            })
                        }
                    }
                }

                if self.castling_rights.can_bk() {
                    if self.my_pieces.rook & BRK_STARTPOS != 0 {
                        if (self.utility.all_occupancy & BK_EMPTY == 0) & (self.utility.opponent_attack & BK_ATTACK == 0) {
                            moves.push(Move {
                                start_square: lsb(BK_STARTPOS),
                                end_square: coord_to_int(7, 6),
                                piece_moved: PieceType::King,
                                piece_captured: PieceType::Null,
                                promotion: PieceType::Null,
                                is_castling: true,
                                is_en_passant: false,
                            })
                        }
                    }
                }
            }
        }

        moves
    }

    fn check_legal_en_passant(&self, mov: &Move) -> bool {
        // todo: probably it still has a minor bug, to reproduce it see todo under main

        // (claim) only case where this should not pass the other checks is if there is a horizontal
        // rook and only two squares. Otherwise it would be pinned / land_mask would catch it
        // maybe treating the pawn as a rook, making a ray, checking if it ends up on a sequence
        // rook pawn king or king pawn rook. If so, it is illegal. Maybe even just looping over
        // the row instead of doing crazy bitboard kung fu

        // if i cast a direction::E ray from the

        // get king position (whether left or right to the pawn)
        // get king rank, if different than mov.start then true

        let ks = self.my_pieces.king;
        let ss_bit = square_num_to_bitboard(mov.start_square);
        // if !same_rank(ks, ss_bit) {
        //     return true;
        // }

        if !(bitboard_to_square_num(ks) / 8 == mov.start_square / 8) {
            return true;
        }

        // loop over line by adding to ks
        let is_left = ks < ss_bit;
        let mut king_file: i32;
        let king_rank: i32;
        let temp = int_to_coord(bitboard_to_square_num(ks));
        king_rank = temp.0 as i32;
        king_file = temp.1 as i32;

        let direction: i32;
        match is_left {
            true => {
                direction = 1;
            }
            false => {
                direction = -1
            }
        }

        let mut found_my_pawn = false;
        let mut found_another_pawn = false;
        king_file += direction;
        while 0 <= king_file && king_file < 8 {
            let x = square_num_to_bitboard(coord_to_int(king_rank as u8, king_file as u8));
            if self.utility.all_occupancy & x != 0 {
                if x == square_num_to_bitboard(mov.start_square) {
                    found_my_pawn = true;
                } else if self.opponent_pieces.pawns & x != 0 {
                    if found_another_pawn {
                        return true;
                    } else {
                        found_another_pawn = true;
                    }
                } else {
                    if (self.opponent_pieces.rook & x != 0) || (self.opponent_pieces.queen & x != 0) {
                        return false;
                    } else {
                        return true;
                    }
                }
            }
            king_file += direction;
        }

        return true;
    }

    pub(crate) fn is_3fold(&self) -> bool {
        let hash = self.zobrist.hash;
        self.zobrist_stack.iter().fold(0, |acc, x| if *x == hash { acc + 1 } else { acc }) >= 3
    }
}

impl Board {
    pub fn make_move(&mut self, mov: Move) {
        self.update_hash(mov);
        self.moves_stack.push(mov);
        self.castling_stack.push(self.castling_rights.clone());
        self.en_passant_stack.push(self.en_passant_square);
        self.zobrist_stack.push(self.zobrist.hash);
        self.en_passant_square = 0;

        match mov.piece_moved {
            PieceType::King => {
                match self.color_to_move {
                    WHITE => { self.castling_rights.moved_white_king() }
                    BLACK => { self.castling_rights.moved_black_king() }
                }

                // check if castling
                if mov.is_castling {
                    self.make_castling_move(mov);
                } else {
                    self.make_simple_move(mov);
                }
            }
            PieceType::Pawn => {
                if mov.is_en_passant {
                    match self.color_to_move {
                        WHITE => { self.opponent_pieces.pawns &= !square_num_to_bitboard(mov.end_square - 8) }
                        BLACK => { self.opponent_pieces.pawns &= !square_num_to_bitboard(mov.end_square + 8) }
                    }
                    self.my_pieces.pawns &= !square_num_to_bitboard(mov.start_square);
                    self.my_pieces.pawns |= square_num_to_bitboard(mov.end_square);
                } else if mov.promotion != PieceType::Null {
                    self.my_pieces.pawns &= !square_num_to_bitboard(mov.start_square);
                    self.my_pieces[mov.promotion] |= square_num_to_bitboard(mov.end_square);

                    if mov.piece_captured != PieceType::Null {
                        self.opponent_pieces[mov.piece_captured] &= !square_num_to_bitboard(mov.end_square);
                    }
                } else {
                    if (mov.end_square as i32 - mov.start_square as i32).abs() == 16 {
                        match self.color_to_move {
                            WHITE => {
                                self.en_passant_square = mov.start_square + 8;
                            }
                            BLACK => {
                                self.en_passant_square = mov.start_square - 8;
                            }
                        }
                    }
                    self.make_simple_move(mov);
                }
            }
            PieceType::Rook => {
                self.castling_rights.moved_rook(mov.start_square);
                self.make_simple_move(mov);
            }
            PieceType::Knight => {
                self.make_simple_move(mov);
            }
            PieceType::Bishop => {
                self.make_simple_move(mov);
            }
            PieceType::Queen => {
                self.make_simple_move(mov);
            }
            PieceType::Null => { panic!("No bueno") }
        }

        self.color_to_move = self.color_to_move.flip();


        let temp = self.my_pieces.clone();
        self.my_pieces = self.opponent_pieces.clone();
        self.opponent_pieces = temp;

        // todo: does this actually work + check which one is faster and decide if it is worth?
        // mem::swap(&mut self.my_pieces, &mut self.opponent_pieces);
    }


    fn make_simple_move(&mut self, mov: Move) {
        // a simple move is a move where piece a moves to square b and might capture something
        // or not. It is not a castling move or an en passant move

        // move my piece
        self.my_pieces[mov.piece_moved] &= !square_num_to_bitboard(mov.start_square);
        self.my_pieces[mov.piece_moved] |= square_num_to_bitboard(mov.end_square);

        // capture opponent piece
        if mov.piece_captured != PieceType::Null {
            self.opponent_pieces[mov.piece_captured] &= !square_num_to_bitboard(mov.end_square);
        }
    }
    fn make_castling_move(&mut self, mov: Move) {
        let rank: u8;
        match self.color_to_move {
            WHITE => { rank = 0 }
            BLACK => { rank = 7 }
        }

        if mov.end_square == coord_to_int(rank, 2) {
            // queen side
            self.my_pieces.rook &= !square_num_to_bitboard(coord_to_int(rank, 0));
            self.my_pieces.rook |= square_num_to_bitboard(coord_to_int(rank, 3));

            self.my_pieces.king &= !square_num_to_bitboard(coord_to_int(rank, 4));
            self.my_pieces.king |= square_num_to_bitboard(coord_to_int(rank, 2));
        } else {
            // king side
            self.my_pieces.rook &= !square_num_to_bitboard(coord_to_int(rank, 7));
            self.my_pieces.rook |= square_num_to_bitboard(coord_to_int(rank, 5));

            self.my_pieces.king &= !square_num_to_bitboard(coord_to_int(rank, 4));
            self.my_pieces.king |= square_num_to_bitboard(coord_to_int(rank, 6));
        }
    }

    pub fn unmake_move(&mut self) {
        let mov = self.moves_stack.pop().unwrap();


        self.castling_rights = self.castling_stack.pop().unwrap();
        self.en_passant_square = self.en_passant_stack.pop().unwrap();
        self.zobrist.hash = self.zobrist_stack.pop().unwrap();

        self.color_to_move = self.color_to_move.flip();

        let temp = self.my_pieces.clone();
        self.my_pieces = self.opponent_pieces.clone();
        self.opponent_pieces = temp;


        match mov.piece_moved {
            PieceType::King => {
                // check if castling
                if mov.is_castling {
                    self.unmake_castling_move(mov);
                } else {
                    self.unmake_simple_move(mov);
                }
            }
            PieceType::Pawn => {
                if mov.is_en_passant {
                    match self.color_to_move {
                        WHITE => { self.opponent_pieces.pawns |= square_num_to_bitboard(mov.end_square - 8) }
                        BLACK => { self.opponent_pieces.pawns |= square_num_to_bitboard(mov.end_square + 8) }
                    }
                    self.my_pieces.pawns |= square_num_to_bitboard(mov.start_square);
                    self.my_pieces.pawns &= !square_num_to_bitboard(mov.end_square);
                } else if mov.promotion != PieceType::Null {
                    self.my_pieces[mov.promotion] &= !square_num_to_bitboard(mov.end_square);
                    self.my_pieces.pawns |= square_num_to_bitboard(mov.start_square);

                    if mov.piece_captured != PieceType::Null {
                        self.opponent_pieces[mov.piece_captured] |= square_num_to_bitboard(mov.end_square);
                    }
                } else {
                    self.unmake_simple_move(mov);
                }
            }
            PieceType::Rook => {
                self.unmake_simple_move(mov);
            }
            PieceType::Knight => {
                self.unmake_simple_move(mov);
            }
            PieceType::Bishop => {
                self.unmake_simple_move(mov);
            }
            PieceType::Queen => {
                self.unmake_simple_move(mov);
            }
            _ => {}
        }
    }
    fn unmake_castling_move(&mut self, mov: Move) {
        let rank: u8;
        match self.color_to_move {
            WHITE => { rank = 0 }
            BLACK => { rank = 7 }
        }

        if mov.end_square == coord_to_int(rank, 2) {
            // queen side
            self.my_pieces.rook &= !square_num_to_bitboard(coord_to_int(rank, 3));
            self.my_pieces.rook |= square_num_to_bitboard(coord_to_int(rank, 0));

            self.my_pieces.king &= !square_num_to_bitboard(coord_to_int(rank, 2));
            self.my_pieces.king |= square_num_to_bitboard(coord_to_int(rank, 4));
        } else {
            // king side
            self.my_pieces.rook &= !square_num_to_bitboard(coord_to_int(rank, 5));
            self.my_pieces.rook |= square_num_to_bitboard(coord_to_int(rank, 7));

            self.my_pieces.king &= !square_num_to_bitboard(coord_to_int(rank, 6));
            self.my_pieces.king |= square_num_to_bitboard(coord_to_int(rank, 4));
        }
    }
    fn unmake_simple_move(&mut self, mov: Move) {
        // a simple move is a move where piece a moves to square b and might capture something
        // or not. It is not a castling move or an en passant move

        // move my piece
        self.my_pieces[mov.piece_moved] &= !square_num_to_bitboard(mov.end_square);
        self.my_pieces[mov.piece_moved] |= square_num_to_bitboard(mov.start_square);

        // capture opponent piece
        if mov.piece_captured != PieceType::Null {
            self.opponent_pieces[mov.piece_captured] |= square_num_to_bitboard(mov.end_square);
        }
    }
}

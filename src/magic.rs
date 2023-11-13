use std::ops::{Index, IndexMut};
use crate::constants::{COLOR};
use crate::helpers::{get_msb_masked, lsb, pop_count, remove_msb};


pub const fn coord_to_int(rank: u8, file: u8) -> u8 {
    rank * 8 + file
}

pub const fn coord_bit(x: u8, y: u8) -> u64 {
    square_num_to_bitboard(coord_to_int(x, y))
}
#[inline]
pub const fn square_num_to_bitboard(square: u8) -> u64 {
    1 << square
}

pub fn bitboard_to_square_num(x: u64) -> u8 {
    lsb(x)
}


pub fn int_to_coord(square: u8) -> (u8, u8) {
    (square / 8, square % 8)
}


fn signed_valid_coord(rank: i32, file: i32) -> bool {
    file >= 0 && file < 8 && rank >= 0 && rank < 8
}

fn signed_interior_coord(file: i32, rank: i32) -> bool {
    file > 0 && file < 7 && rank > 0 && rank < 7
}

pub fn hash_on_mask(key: u64, mask: u64) -> u64 {
    // gives the inverse order but looks quite bijective to me

    let mut result: u64 = 0;
    let mut _mask = mask;
    let mut sq;
    while _mask != 0 {
        sq = get_msb_masked(_mask);
        let res = ((sq & key) != 0) as u64;

        result <<= 1;
        result |= res;

        _mask = remove_msb(_mask);
    }

    result
}

pub fn inverse_hash_on_mask(result: u64, mask: u64) -> u64 {
    let mut original_key: u64 = 0;
    let mut _result = result;
    let mut _mask = mask;
    let mut bit: u64 = 1;

    while _mask != 0 {
        if (_mask & 1) == 1 {
            original_key |= (_result & 1) << bit.trailing_zeros();
            _result >>= 1;
        }
        _mask >>= 1;
        bit <<= 1;
    }

    original_key
}

// derive clone and copy

fn new_direction_magic() -> DirectionMagic {
    DirectionMagic {
        north: vec![],
        south: vec![],
        east: vec![],
        west: vec![],
        north_east: vec![],
        north_west: vec![],
        south_east: vec![],
        south_west: vec![],
    }
}

#[derive(Clone, Debug)]
struct DirectionMagic {
    north: Vec<u64>,
    south: Vec<u64>,
    east: Vec<u64>,
    west: Vec<u64>,
    north_east: Vec<u64>,
    north_west: Vec<u64>,
    south_east: Vec<u64>,
    south_west: Vec<u64>,
}

impl Index<DIRECTIONS> for DirectionMagic {
    type Output = Vec<u64>;

    fn index(&self, index: DIRECTIONS) -> &Self::Output {
        match index {
            DIRECTIONS::N => &self.north,
            DIRECTIONS::E => &self.east,
            DIRECTIONS::S => &self.south,
            DIRECTIONS::W => &self.west,
            DIRECTIONS::NE => &self.north_east,
            DIRECTIONS::NW => &self.north_west,
            DIRECTIONS::SE => &self.south_east,
            DIRECTIONS::SW => &self.south_west,
        }
    }
}

impl IndexMut<DIRECTIONS> for DirectionMagic {
    fn index_mut(&mut self, index: DIRECTIONS) -> &mut Self::Output {
        match index {
            DIRECTIONS::N => &mut self.north,
            DIRECTIONS::E => &mut self.east,
            DIRECTIONS::S => &mut self.south,
            DIRECTIONS::W => &mut self.west,
            DIRECTIONS::NE => &mut self.north_east,
            DIRECTIONS::NW => &mut self.north_west,
            DIRECTIONS::SE => &mut self.south_east,
            DIRECTIONS::SW => &mut self.south_west,
        }
    }
}

pub struct DirectionMasks {
    north: [u64; 64],
    south: [u64; 64],
    east: [u64; 64],
    west: [u64; 64],
    north_east: [u64; 64],
    north_west: [u64; 64],
    south_east: [u64; 64],
    south_west: [u64; 64],
}

impl Index<DIRECTIONS> for DirectionMasks {
    type Output = [u64; 64];

    fn index(&self, index: DIRECTIONS) -> &Self::Output {
        match index {
            DIRECTIONS::N => &self.north,
            DIRECTIONS::E => &self.east,
            DIRECTIONS::S => &self.south,
            DIRECTIONS::W => &self.west,
            DIRECTIONS::NE => &self.north_east,
            DIRECTIONS::NW => &self.north_west,
            DIRECTIONS::SE => &self.south_east,
            DIRECTIONS::SW => &self.south_west,
        }
    }
}

impl IndexMut<DIRECTIONS> for DirectionMasks {
    fn index_mut(&mut self, index: DIRECTIONS) -> &mut Self::Output {
        match index {
            DIRECTIONS::N => &mut self.north,
            DIRECTIONS::E => &mut self.east,
            DIRECTIONS::S => &mut self.south,
            DIRECTIONS::W => &mut self.west,
            DIRECTIONS::NE => &mut self.north_east,
            DIRECTIONS::NW => &mut self.north_west,
            DIRECTIONS::SE => &mut self.south_east,
            DIRECTIONS::SW => &mut self.south_west,
        }
    }
}

pub struct Magics {
    king_magic: [u64; 64],
    knight_magic: [u64; 64],

    pawn_captures_white: [u64; 64],
    pawn_captures_black: [u64; 64],

    rook_magic: [Vec<u64>; 64],
    bishop_magic: [Vec<u64>; 64],

    direction_magic: [DirectionMagic; 64],

    rook_full_masks: [u64; 64],
    rook_partial_masks: [u64; 64],
    bishop_full_masks: [u64; 64],
    bishop_partial_masks: [u64; 64],

    pub direction_full_masks: DirectionMasks,
    pub direction_partial_masks: DirectionMasks,
}

fn new_direction_mask() -> DirectionMasks {
    DirectionMasks {
        north: [0; 64],
        south: [0; 64],
        east: [0; 64],
        west: [0; 64],
        north_east: [0; 64],
        north_west: [0; 64],
        south_east: [0; 64],
        south_west: [0; 64],
    }
}

fn array() -> [u64; 64] {
    let mut arr = [0; 64];
    for i in 0..64 {
        arr[i] = 0;
    }
    arr
}

fn array_vec() -> [Vec<u64>; 64] {
    // initialize empty array of vecs
    let arr: [Vec<u64>; 64] = [vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![], ];
    arr
}

fn array_vec_dm() -> [DirectionMagic; 64] {
    // initialize empty array of new_direction_magic
    let arr: [DirectionMagic; 64] = [new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic(), new_direction_magic()];
    arr
}


pub fn new_magic() -> Magics {
    let mut mag = Magics {
        king_magic: array(),
        knight_magic: array(),
        rook_magic: array_vec(),
        bishop_magic: array_vec(),

        direction_magic: array_vec_dm(),

        rook_full_masks: array(),
        rook_partial_masks: array(),
        bishop_full_masks: array(),
        bishop_partial_masks: array(),
        direction_full_masks: new_direction_mask(),
        pawn_captures_white: array(),
        pawn_captures_black: array(),
        direction_partial_masks: new_direction_mask(),
    };
    mag.init();

    mag
}

#[derive(Clone, Copy, PartialEq)]
pub enum DIRECTIONS {
    N,
    E,
    S,
    W,
    NE,
    SE,
    SW,
    NW,
}

pub const DIAGONAL_DIRS: [DIRECTIONS; 4] = [
    DIRECTIONS::NW,
    DIRECTIONS::NE,
    DIRECTIONS::SE,
    DIRECTIONS::SW,
];

pub const STRAIGHT_DIRS: [DIRECTIONS; 4] = [
    DIRECTIONS::N,
    DIRECTIONS::E,
    DIRECTIONS::S,
    DIRECTIONS::W,
];

impl DIRECTIONS {
    fn to_pair(&self) -> (i32, i32) {
        match self {
            DIRECTIONS::N => { (1, 0) }
            DIRECTIONS::E => { (0, 1) }
            DIRECTIONS::S => { (-1, 0) }
            DIRECTIONS::W => { (0, -1) }
            DIRECTIONS::NE => { (1, 1) }
            DIRECTIONS::SE => { (-1, 1) }
            DIRECTIONS::SW => { (-1, -1) }
            DIRECTIONS::NW => { (1, -1) }
        }
    }
}


impl Magics {
    pub fn init(&mut self) {
        self.init_masks();

        self.init_king(); // looks good
        self.init_knight(); // looks good
        self.init_pawn(); // looks good

        // println!("{:064b}", self.pawn_captures_white[8]);
        // println!("{:064b}", self.pawn_captures_white[12]);
        // println!("{:064b}", self.pawn_captures_black[12]);


        self.init_rook(); // looks good


        self.init_bishop(); // looks good

        // usef(self.rook_magic[0][0]);
        // print!("2  ");
        // usef(self.bishop_magic[2][2 as usize]);
        // print!("3  ");
        // usef(self.bishop_magic[2][3 as usize]);
        // print!("4  ");
        // usef(self.bishop_magic[2][4 as usize]);
        // print!("8  ");
        // usef(self.bishop_magic[2][8 as usize]);


        self.init_directions(); // looks good
    }

    fn init_king(&mut self) {
        const KING_PATTERNS: [(i32, i32); 8] = [
            (1, 1), (1, 0), (1, -1), (0, 1), (0, -1), (-1, 1), (-1, 0), (-1, -1)
        ];

        for starting_square in 0..64
        {
            let (rank, file) = int_to_coord(starting_square);
            for (a, b) in KING_PATTERNS.iter() {
                let a = *a;
                let b = *b;
                let (rank, file) = (rank as i32 + a, file as i32 + b);
                if signed_valid_coord(rank, file) {
                    let ending_square = coord_to_int(rank as u8, file as u8);
                    self.king_magic[starting_square as usize] |= 1 << ending_square;
                }
            }
        }
    }

    fn init_knight(&mut self) {
        const KNIGHT_PATTERNS: [(i32, i32); 8] = [
            (2, 1), (1, 2), (2, -1), (-1, 2), (-2, 1), (1, -2), (-1, -2), (-2, -1)
        ];

        for starting_square in 0..64
        {
            let (rank, file) = int_to_coord(starting_square);
            for (a, b) in KNIGHT_PATTERNS.iter() {
                let a = *a;
                let b = *b;
                let (rank, file) = (rank as i32 + a, file as i32 + b);
                if signed_valid_coord(rank, file) {
                    let ending_square = coord_to_int(rank as u8, file as u8);
                    self.knight_magic[starting_square as usize] |= 1 << ending_square;
                }
            }
        }
    }
    fn init_rook(&mut self) {
        for ss in 0..64 {
            let mask = self.rook_full_masks[ss];
            let size = pop_count(mask);

            for hash in 0..2_u64.pow(size) {
                let blockers = inverse_hash_on_mask(hash, mask);

                let mut res: u64 = 0;
                for dir in STRAIGHT_DIRS.iter() {
                    res |= self.compute_moves(ss as u8, blockers, *dir);
                }

                self.rook_magic[ss].push(res);
            }
        }
    }
    fn init_bishop(&mut self) {
        for ss in 0..64 {
            let mask = self.bishop_full_masks[ss];
            let size = pop_count(mask);

            for hash in 0..2_u64.pow(size) {
                let blockers = inverse_hash_on_mask(hash, mask);

                let mut res: u64 = 0;
                for dir in DIAGONAL_DIRS.iter() {
                    res |= self.compute_moves(ss as u8, blockers, *dir);
                }

                self.bishop_magic[ss].push(res);
            }
        }
    }
    fn init_pawn(&mut self) {
        for dir in [1, -1].iter() {
            let dir = *dir;
            for ss in 0_u8..64 {
                let mut res: u64 = 0;
                let (rank, file) = int_to_coord(ss);
                let (rank, file) = (rank as i32 + dir, file as i32);
                if signed_valid_coord(rank, file) {
                    if signed_valid_coord(rank, file - 1) {
                        let ending_square = coord_to_int(rank as u8, (file - 1) as u8);
                        res |= 1 << ending_square;
                    }
                    if signed_valid_coord(rank, file + 1) {
                        let ending_square = coord_to_int(rank as u8, (file + 1) as u8);
                        res |= 1 << ending_square;
                    }
                }
                if dir == 1 {
                    self.pawn_captures_white[ss as usize] |= res;
                } else {
                    self.pawn_captures_black[ss as usize] |= res;
                }
            }
        }
    }

    fn init_masks(&mut self) {

        // output looks good

        self.init_direction_masks();
        self.init_rook_masks();
        self.init_bishop_masks();
    }
    fn init_rook_masks(&mut self) {
        for ss in 0..64 {
            self.rook_full_masks[ss] = self.direction_full_masks[DIRECTIONS::N][ss] | self.direction_full_masks[DIRECTIONS::E][ss] | self.direction_full_masks[DIRECTIONS::S][ss] | self.direction_full_masks[DIRECTIONS::W][ss];
            self.rook_partial_masks[ss] = self.direction_partial_masks[DIRECTIONS::N][ss] | self.direction_partial_masks[DIRECTIONS::E][ss] | self.direction_partial_masks[DIRECTIONS::S][ss] | self.direction_partial_masks[DIRECTIONS::W][ss];
        }
    }

    fn init_bishop_masks(&mut self) {
        for ss in 0..64 {
            self.bishop_full_masks[ss] = self.direction_full_masks[DIRECTIONS::NE][ss] | self.direction_full_masks[DIRECTIONS::NW][ss] | self.direction_full_masks[DIRECTIONS::SE][ss] | self.direction_full_masks[DIRECTIONS::SW][ss];
            self.bishop_partial_masks[ss] = self.direction_partial_masks[DIRECTIONS::NE][ss] | self.direction_partial_masks[DIRECTIONS::NW][ss] | self.direction_partial_masks[DIRECTIONS::SE][ss] | self.direction_partial_masks[DIRECTIONS::SW][ss];
        }
    }

    fn init_direction_masks(&mut self) {
        for sq in 0..64 {
            self.update_direction_mask(sq as u8, DIRECTIONS::N);
            self.update_direction_mask(sq as u8, DIRECTIONS::E);
            self.update_direction_mask(sq as u8, DIRECTIONS::S);
            self.update_direction_mask(sq as u8, DIRECTIONS::W);
            self.update_direction_mask(sq as u8, DIRECTIONS::NE);
            self.update_direction_mask(sq as u8, DIRECTIONS::NW);
            self.update_direction_mask(sq as u8, DIRECTIONS::SE);
            self.update_direction_mask(sq as u8, DIRECTIONS::SW);
        }
    }

    fn update_direction_mask(&mut self, sq: u8, dir: DIRECTIONS) {
        let drank: i32;
        let dfile: i32;
        drank = dir.to_pair().0;
        dfile = dir.to_pair().1;

        let mut crank = sq as i32 / 8;
        let mut cfile = sq as i32 % 8;

        crank += drank;
        cfile += dfile;
        while signed_interior_coord(crank, cfile) {
            let csq = coord_to_int(crank as u8, cfile as u8);
            self.direction_partial_masks[dir][sq as usize] |= square_num_to_bitboard(csq);
            crank += drank;
            cfile += dfile;
        }

        crank = sq as i32 / 8;
        cfile = sq as i32 % 8;

        crank += drank;
        cfile += dfile;
        while signed_valid_coord(crank, cfile) {
            let csq = coord_to_int(crank as u8, cfile as u8);
            self.direction_full_masks[dir][sq as usize] |= square_num_to_bitboard(csq);
            crank += drank;
            cfile += dfile;
        }
    }

    pub fn get_king_moves(&self, square: u8) -> u64 {
        self.king_magic[square as usize]
    }

    pub fn get_knight_moves(&self, square: u8) -> u64 {
        self.knight_magic[square as usize]
    }

    pub fn get_rook_moves(&self, square: u8, blockers: u64) -> u64 {
        let mask = self.rook_full_masks[square as usize];
        let hash = hash_on_mask(blockers, mask);
        self.rook_magic[square as usize][hash as usize]
    }

    pub fn get_bishop_moves(&self, square: u8, blockers: u64) -> u64 {
        let mask = self.bishop_full_masks[square as usize];
        let hash = hash_on_mask(blockers, mask);
        self.bishop_magic[square as usize][hash as usize]
    }

    pub fn get_queen_moves(&self, square: u8, blockers: u64) -> u64 {
        self.get_rook_moves(square, blockers) | self.get_bishop_moves(square, blockers)
    }

    pub fn get_rays_moves(&self, square: u8, blockers: u64, direction: DIRECTIONS) -> u64 {
        let mask = self.direction_full_masks[direction][square as usize];
        let hash = hash_on_mask(blockers, mask);
        self.direction_magic[square as usize][direction][hash as usize]
    }

    pub fn get_pawn_captures(&self, square: u8, color: COLOR) -> u64 {
        match color {
            COLOR::WHITE => self.pawn_captures_white[square as usize],
            COLOR::BLACK => self.pawn_captures_black[square as usize],
        }
    }
    fn init_directions(&mut self) {
        for sq in 0..64 {
            self.update_direction(sq as u8, DIRECTIONS::N);
            self.update_direction(sq as u8, DIRECTIONS::E);
            self.update_direction(sq as u8, DIRECTIONS::S);
            self.update_direction(sq as u8, DIRECTIONS::W);
            self.update_direction(sq as u8, DIRECTIONS::NE);
            self.update_direction(sq as u8, DIRECTIONS::NW);
            self.update_direction(sq as u8, DIRECTIONS::SE);
            self.update_direction(sq as u8, DIRECTIONS::SW);
        }
    }

    fn compute_moves(&self, square: u8, blockers: u64, direction: DIRECTIONS) -> u64 {
        let mut reachable: u64 = 0;

        let drank: i32;
        let dfile: i32;
        drank = direction.to_pair().0;
        dfile = direction.to_pair().1;

        let mut crank = square as i32 / 8;
        let mut cfile = square as i32 % 8;

        crank += drank;
        cfile += dfile;
        while signed_valid_coord(crank, cfile) {
            let csq = coord_to_int(crank as u8, cfile as u8);
            reachable |= square_num_to_bitboard(csq);

            if (square_num_to_bitboard(csq) & blockers) != 0 {
                break;
            }

            crank += drank;
            cfile += dfile;
        }

        reachable
    }
    fn update_direction(&mut self, square: u8, direction: DIRECTIONS) {
        let mask = self.direction_full_masks[direction][square as usize];
        let size = pop_count(mask);
        for hash in 0..2_u64.pow(size) {
            let blockers = inverse_hash_on_mask(hash, mask);

            let res = self.compute_moves(square as u8, blockers, direction);

            self.direction_magic[square as usize][direction].push(res);
        }
    }
}
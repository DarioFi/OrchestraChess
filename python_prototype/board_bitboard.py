from typing import List

from bitboard import BitBoard
from attack_bitboard import MovePatterns
from constants import COLOR, PieceType
from move import Move


class BitBoardManager:
    def __init__(self, color: COLOR):
        self.color = color
        self.pawn_moves = MovePatterns.pawn_moves[color]
        self.pawn_attacks = MovePatterns.pawn_attacks[color]
        
        self.bishop_bitboard = BitBoard()
        self.rook_bitboard = BitBoard()
        self.queen_bitboard = BitBoard()
        self.knight_bitboard = BitBoard()
        self.pawn_bitboard = BitBoard()
        self.king_bitboard = BitBoard()

    
    def get_occupied_squares(self):
        return self.bishop_bitboard | self.rook_bitboard | self.queen_bitboard | self.knight_bitboard | self.pawn_bitboard | self.king_bitboard


class UtilityBitboard:
    def __init__(self):
        self.pinned_squares = BitBoard()
        self.checkers = BitBoard()
        self.check_rays = BitBoard()


class Board:

    def __init__(self):
        self.white_bitboards = BitBoardManager(COLOR.WHITE)
        self.black_bitboards = BitBoardManager(COLOR.BLACK)
        self.white_piece_to_square = [[] for _ in PieceType]
        self.black_piece_to_square = [[] for _ in PieceType]
        self.my_bitboards = None
        self.opponent_bitboards = None

        self.castling_rights = CastlingRights()
        self.en_passant = None
        self.color_to_move = None
        self.zobrist = None

        self.utility_bitboard = UtilityBitboard()

        self.move_history = []
        self.castling_rights_history = []
        self.en_passant_history = []
        self.zobrist_hash_history = []

    @classmethod
    def from_fen(cls, fen: str):
        raise NotImplementedError

    @classmethod
    def from_startpos(cls):
        return cls.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    
    def generate_moves(self) -> List[Move]:
        raise NotImplementedError

    def update_utility_bitboard(self):
        checkers = BitBoard()
        check_rays = BitBoard()
        king_square = self.my_bitboards.king_bitboard.lsb()
        occupied_squares = self.my_bitboards.get_occupied_squares() | self.opponent_bitboards.get_occupied_squares()

        knight_attacks = MovePatterns.knight_moves[king_square] & self.opponent_bitboards.knight_bitboard
        checkers |= knight_attacks
        check_rays |= knight_attacks
        pawn_attacks = self.my_bitboards.pawn_attacks[king_square] & self.opponent_bitboards.pawn_bitboard
        checkers |= pawn_attacks
        check_rays |= pawn_attacks
        bishop_attacks = magic_bishop(king_square, occupied_squares) & (self.opponent_bitboards.bishop_bitboard | self.opponent_bitboards.queen_bitboard)
        checkers |= bishop_attacks
        check_rays |= compute_bishop_rays(king_square, bishop_attacks)
        rook_attacks = magic_rook(king_square, occupied_squares) & (self.opponent_bitboards.rook_bitboard | self.opponent_bitboards.queen_bitboard)
        checkers |= rook_attacks
        check_rays |= compute_rook_rays(king_square, rook_attacks)

        self.utility_bitboard.checkers = checkers
        self.utility_bitboard.check_rays = check_rays

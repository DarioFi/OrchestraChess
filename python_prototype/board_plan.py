from bitboard import BitBoard


diagonal_pattern = [(1, 1), (-1, 1), (1, -1), (-1, -1)]
straight_pattern = [(1, 0), (-1, 0), (0, 1), (0, -1)]
knight_pattern = [(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)]
king_pattern = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)]


class Bitboards:
    """
    Mantain one bitboard per piece type.
    """
    def __init__(self):
        self.bishop_bitboard = BitBoard()
        self.rook_bitboard = BitBoard()
        self.queen_bitboard = BitBoard()
        self.knight_bitboard = BitBoard()
        self.pawn_bitboard = BitBoard()
        self.king_bitboard = BitBoard()

    def occupied_squares(self):
        return self.bishop_bitboard | self.rook_bitboard | self.queen_bitboard | self.knight_bitboard | self.pawn_bitboard | self.king_bitboard


class MovePatterns:
    """
    Handle generation of move patterns based on piece type, starting position, and occupancy.
    """
    def __init__(self):
        MovePatterns.knight_moves = adjacent(knight_pattern)
        MovePatterns.king_moves = adjacent(king_pattern)
        MovePatterns.rook_moves = sliding(straight_pattern)
        MovePatterns.bishop_moves = sliding(diagonal_pattern)
        MovePatterns.queen_moves = [x | y for x, y in zip(MovePatterns.rook_moves, MovePatterns.bishop_moves)]


    
    @staticmethod
    def adjacent(patterns):
        res = []
        for starting_square in range(64):
            file, rank = starting_square % 8, starting_square // 8
            new = BitBoard()
            for pattern in patterns:
                file_a = file + pattern[0]
                rank_a = rank + pattern[1]
                if 0 <= file_a <= 7 and 0 <= rank_a <= 7:
                    new.set_squares(file_a + rank_a * 8)
            res.append(new)
        return res

    @staticmethod
    def sliding(patterns):
        res = []
        for starting_square in range(64):
            starting_square: int
            file, rank = starting_square % 8, starting_square // 8
            new = BitBoard()
            for pattern in patterns:
                file_a = file + pattern[0]
                rank_a = rank + pattern[1]
                while 0 <= file_a <= 7 and 0 <= rank_a <= 7:
                    new.set_squares(file_a + rank_a * 8)
                    file_a += pattern[0]
                    rank_a += pattern[1]
            res.append(new)
        return res


class Board:
    self.white_bitboards = Bitboards()
    self.black_bitboards = Bitboards()
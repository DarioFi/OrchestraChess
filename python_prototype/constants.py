import dataclasses
from enum import Enum
from typing import Tuple


class COLOR(Enum):
    WHITE = 1
    BLACK = -1

    def flip(self):
        return COLOR.WHITE if self == COLOR.BLACK else COLOR.BLACK

    def is_max(self):
        return self == COLOR.WHITE


class PieceType(Enum):
    PAWN = 0
    KNIGHT = 1
    BISHOP = 2
    ROOK = 3
    QUEEN = 4
    KING = 5

    @classmethod
    def from_char(cls, param):
        match param:
            case "p":
                return cls.PAWN
            case "n":
                return cls.KNIGHT
            case "b":
                return cls.BISHOP
            case "r":
                return cls.ROOK
            case "q":
                return cls.QUEEN
            case "k":
                return cls.KING
            case _:
                raise ValueError(param)

    def to_char(self):  # this acts on the instance instead of the class, which makes sense, but it's different from
        # from_char
        match self:
            case self.PAWN:
                return "p"
            case self.KNIGHT:
                return "n"
            case self.BISHOP:
                return "b"
            case self.ROOK:
                return "r"
            case self.QUEEN:
                return "q"
            case self.KING:
                return "k"


@dataclasses.dataclass(frozen=True)
class Square:  # todo: be very careful of potential bugs due to copies by reference instead of by value
    file: int
    rank: int

    @classmethod
    def from_string_algebraic(cls, s):
        return cls(ord(s[0]) - ord("a"), int(s[1]) - 1)

    def to_string_algebraic(self):
        return chr(self.file + ord("a")) + str(self.rank + 1)

    def __add__(self, other: Tuple[int, int]):
        return Square(
            self.file + other[0],
            self.rank + other[1]
        )

    def is_valid(self):
        return 0 <= self.file < 8 and 0 <= self.rank < 8

    def clone(self):
        return Square(self.file, self.rank)


pawn_table = [
    [0, 0, 0, 0, 0, 0, 0, 0, ],
    [50, 50, 50, 50, 50, 50, 50, 50, ],
    [10, 10, 20, 30, 30, 20, 10, 10, ],
    [5, 5, 10, 25, 25, 10, 5, 5, ],
    [0, 0, 0, 20, 20, 0, 0, 0, ],
    [5, -5, -10, 0, 0, -10, -5, 5, ],
    [5, 10, 10, -20, -20, 10, 10, 5, ],
    [0, 0, 0, 0, 0, 0, 0, 0]
]

pawn_table_endgame = [
    [0, 0, 0, 0, 0, 0, 0, 0, ],
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 5, 5, 5, 5, 5, 5, 5, ],
    [10, 10, 10, 10, 10, 10, 10, 10, ],
    [20, 20, 20, 20, 20, 20, 20, 20, ],
    [30, 30, 30, 30, 30, 30, 30, 30, ],
    [50, 50, 50, 50, 50, 50, 50, 50, ],
    [0, 0, 0, 0, 0, 0, 0, 0]
]

rook_table = [
    [0, 0, 0, 0, 0, 0, 0, 0],
    [5, 10, 10, 10, 10, 10, 10, 5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [-5, 0, 0, 0, 0, 0, 0, -5],
    [0, 0, 0, 5, 5, 0, 0, 0]
]

bishop_table = [
    [-20, -10, -10, -10, -10, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 10, 10, 5, 0, -10],
    [-10, 5, 5, 10, 10, 5, 5, -10],
    [-10, 0, 10, 10, 10, 10, 0, -10],
    [-10, 10, 10, 10, 10, 10, 10, -10],
    [-10, 5, 0, 0, 0, 0, 5, -10],
    [-20, -10, -10, -10, -10, -10, -10, -20],
]

queen_table = [
    [-20, -10, -10, -5, -5, -10, -10, -20],
    [-10, 0, 0, 0, 0, 0, 0, -10],
    [-10, 0, 5, 5, 5, 5, 0, -10],
    [-5, 0, 5, 5, 5, 5, 0, -5],
    [0, 0, 5, 5, 5, 5, 0, -5],
    [-10, 5, 5, 5, 5, 5, 0, -10],
    [-10, 0, 5, 0, 0, 0, 0, -10],
    [-20, -10, -10, -5, -5, -10, -10, -20]
]
knight_table = [
    [-50, -40, -30, -30, -30, -30, -40, -50],
    [-40, -20, 0, 0, 0, 0, -20, -40],
    [-30, 0, 10, 15, 15, 10, 0, -30],
    [-30, 5, 15, 20, 20, 15, 5, -30],
    [-30, 0, 15, 20, 20, 15, 0, -30],
    [-30, 5, 10, 15, 15, 10, 5, -30],
    [-40, -20, 0, 5, 5, 0, -20, -40],
    [-50, -40, -30, -30, -30, -30, -40, -50],
]

king_table = [
    [-80, -70, -70, -70, -70, -70, -70, -80],
    [-60, -60, -60, -60, -60, -60, -60, -60],
    [-40, -50, -50, -60, -60, -50, -50, -40],
    [-30, -40, -40, -50, -50, -40, -40, -30],
    [-20, -30, -30, -40, -40, -30, -30, -20],
    [-10, -20, -20, -20, -20, -20, -20, -10],
    [20, 20, -5, -5, -5, -5, 20, 20],
    [20, 30, 10, 0, 0, 10, 30, 20],
]

king_table_endgame = [
    [-50, -40, -30, -20, -20, -30, -40, -50],
    [-40, -30, -20, -10, -10, -20, -30, -40],
    [-30, -20, -10, 20, 20, -10, -20, -30],
    [-20, -10, 20, 30, 30, 20, -10, -20],
    [-20, -10, 20, 30, 30, 20, -10, -20],
    [-30, -20, -10, 20, 20, -10, -20, -30],
    [-40, -30, -20, -10, -10, -20, -30, -40],
    [-50, -40, -30, -20, -20, -30, -40, -50],

]

complete_table = {
    PieceType.PAWN: pawn_table,
    PieceType.ROOK: rook_table,
    PieceType.BISHOP: bishop_table,
    PieceType.KNIGHT: knight_table,
    PieceType.KING: king_table,
    PieceType.QUEEN: queen_table,
}

complete_table_endgame = {
    PieceType.PAWN: pawn_table_endgame,
    PieceType.ROOK: rook_table,
    PieceType.BISHOP: bishop_table,
    PieceType.KNIGHT: knight_table,
    PieceType.KING: king_table_endgame,
    PieceType.QUEEN: queen_table,
}

values = {
    PieceType.PAWN: 100,
    PieceType.KNIGHT: 320,
    PieceType.BISHOP: 330,
    PieceType.ROOK: 500,
    PieceType.QUEEN: 900,
    PieceType.KING: 20000
}

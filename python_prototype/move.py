from dataclasses import dataclass
from typing import Optional

from constants import Square, PieceType


@dataclass
class Move:  # en passant is codified as a capture of a pawn on the square behind the pawn that moved
    piece_moved: PieceType
    piece_captured: Optional[PieceType]
    from_square: Square
    to_square: Square
    promotion: Optional[PieceType]
    is_check: Optional[bool] = None

    @classmethod
    def from_string(cls, s: str, board):
        """
        instantiate a Move object from a string in algebraic notation (e.g. "e2e4" or "e7e8q").
        :param s: string in algebraic notation.
        :param board: Board object.
        :return: Move object.
        """
        start = Square.from_string_algebraic(s[0:2])
        end = Square.from_string_algebraic(s[2:4])
        promotion = None if len(s) == 4 else PieceType.from_char(s[4])

        return cls(
            piece_moved=board.bitboard[start.rank][start.file][0],
            piece_captured=board.bitboard[end.rank][end.file][0],
            from_square=start,
            to_square=end,
            promotion=promotion,
        )

    def to_string(self):
        """
        create a string in algebraic notation from a Move object.
        :return: string in algebraic notation.
        """
        start = Square.to_string_algebraic(self.from_square)
        end = Square.to_string_algebraic(self.to_square)
        promotion = "" if self.promotion is None else self.promotion.to_char()

        return start + end + promotion

    def __repr__(self):
        return str(self.piece_moved) + "  " + self.to_string()

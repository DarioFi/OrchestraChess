import pytest
from board import Board
from constants import COLOR


def increase_by_one(x):
    return x + 1


@pytest.mark.parametrize("input, expected", [(1, 2), (2, 3), (3, 4)])
def test_increase_by_one(input, expected):
    assert increase_by_one(input) == expected


def test_board_initialization():
    board = Board.from_startpos()
    assert board.color_to_move == COLOR.WHITE
    assert board.castling_rights.white_king_side
    assert board.castling_rights.white_queen_side
    assert board.castling_rights.black_king_side
    assert board.castling_rights.black_queen_side
    assert board.en_passant is None

from chess import pgn, Board
import io

path_to_pgn_file = "/Users/mattia/Desktop/github/OrchestraChess/lichess_pgn_2023.11.26_OrchestraBot_vs_ChessChildren.ekWp5G2c.pgn"


def pgn_to_uci(pgn_moves):
    board = Board()
    uci_moves = []

    for move in pgn_moves:
        uci_move = board.uci(move)
        board.push(move)
        uci_moves.append(uci_move)

    return uci_moves

with open(path_to_pgn_file) as f:
    game = pgn.read_game(f)

# Extract relevant information
headers = game.headers
moves = [move for move in game.mainline_moves()]
algebraic_moves = pgn_to_uci(moves)
print(" ".join(algebraic_moves))

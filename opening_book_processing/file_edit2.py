from chess import pgn, Board


def pgn_to_uci(pgn_moves):
    board = Board()
    uci_moves = []

    for move in pgn_moves:
        uci_move = board.uci(move)
        board.push(move)
        uci_moves.append(uci_move)

    return uci_moves


GAME_LIMIT = 10000000


def process_pgn(pgn_file):
    with open(pgn_file) as f:
        with open('uci_moves.uci', 'w') as o:
            game_count = 0
            elo_white = []
            elo_black = []

            time_controls = []

            while True:
                # print(game_count)
                if game_count > GAME_LIMIT:
                    break
                game = pgn.read_game(f)
                if game is None:
                    break

                # Extract relevant information
                headers = game.headers

                white_elo = int(headers["WhiteElo"])
                black_elo = int(headers["BlackElo"])
                moves = [move for move in game.mainline_moves()]
                algebraic_moves = pgn_to_uci(moves)
                o.write(" ".join(algebraic_moves) + "\n")
                game_count += 1


if __name__ == "__main__":
    pgn_file = "output_high_elo.pgn"  # Replace with the path to your PGN file
    process_pgn(pgn_file)

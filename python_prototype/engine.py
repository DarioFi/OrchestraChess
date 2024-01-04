import copy
import time

from board import Board
from move import Move
from constants import COLOR, PieceType, values
from typing import List, Optional

MATING_SCORE = 250000


def score_move(move: Move):
    if move.piece_captured is not None:
        return values[move.piece_captured]
    elif move.promotion == PieceType.QUEEN:
        return values[PieceType.QUEEN] + 100
    else:
        return 0


# TODO: handle threads.
# Note: with alpha-beta pruning enabled, the search is not exhaustive. We must be carefult when we
# integrate a lookup table, since computed scores may only be upper or lower bounds to the actual score.
class Engine:
    def __init__(self, board: Board):
        self.board = board
        self.current_best_move = None
        self.trasposition_table = {}  # hash: str -> (depth: int, score: float, move: Optional[Move])
        self.node_count = 0

        self.query_hits = 0

    def reorder_moves(self, moves: List[Move]) -> List[Move]:
        """
        Use static heuristics and/or results of previous computations to order moves
        based on how promising they are. Important for pruning.
        :param moves: list of legal moves to reorder
        :return: reordered list of moves
        """
        moves.sort(key=score_move, reverse=True)
        return moves

    def update_transposition_table(self, depth: int, score: float, move: Move, is_exact: bool = True):
        """
        Update transposition table with the result of a computation.
        :param depth: depth of the computation
        :param score: score of the position
        :param move: best move
        """
        hash = self.board.zobrist.get_hash()
        if hash in self.trasposition_table:
            old_depth, old_score, old_move, old_is_exact = self.trasposition_table[hash]
            if old_depth >= depth:
                return
        self.trasposition_table[hash] = (depth, score, move, is_exact)

    def query_trasposition_table(self):
        """
        Check if the current position is already in the transposition table.
        :return: depth, score, move if the position is in the table, 3 times None otherwise
        """
        hash = self.board.zobrist.get_hash()
        if hash in self.trasposition_table:
            self.query_hits += 1
            return self.trasposition_table[hash]

        return None, None, None, None

    def vanilla_minimax(self, depth: int, color: COLOR):
        self.node_count += 1
        if depth == 0:
            return self.board.static_evaluation(), None
        legal_moves = self.board.generate_moves()
        if len(legal_moves) == 0:
            king_square = self.board.piece_to_squares[(PieceType.KING, self.board.color_to_move)][0]
            if self.board.is_attacked(king_square):
                return - color.value * MATING_SCORE, None
            else:
                return 0.0, None

        legal_moves = self.reorder_moves(legal_moves)
        best_score, best_move = - color.value * MATING_SCORE, legal_moves[0]
        for move in legal_moves:
            self.board.make_move(move)
            score = self.vanilla_minimax(depth - 1, color.flip())[0]
            self.board.unmake_move()
            if color.is_max():
                if score > best_score:
                    best_score, best_move = score, move
            else:
                if score < best_score:
                    best_score, best_move = score, move
        return best_score, best_move

    def vanilla_negamax(self, depth, color):
        self.node_count += 1
        old_depth, old_score, old_move, is_exact = self.query_trasposition_table()
        if is_exact == True and old_depth >= depth:
            return old_score, old_move

        if depth == 0:
            return self.board.static_evaluation() * color.value, None
        legal_moves = self.board.generate_moves()
        if len(legal_moves) == 0:
            if self.board.is_attacked(self.board.piece_to_squares[(PieceType.KING, self.board.color_to_move)][0]):
                return -MATING_SCORE, None
            else:
                return 0, None

        legal_moves = self.reorder_moves(legal_moves)
        best_score, best_move = -MATING_SCORE, legal_moves[0]
        for move in legal_moves:
            self.board.make_move(move)
            score = - self.vanilla_negamax(depth - 1, color.flip())[0]
            self.board.unmake_move()
            if score > best_score:
                best_score, best_move = score, move
        self.update_transposition_table(depth, best_score, best_move)
        return best_score, best_move

    def minimax(self, depth: int, alpha: float, beta: float, color: COLOR):
        if depth == 0:
            return self.board.static_evaluation(), None
        legal_moves = self.board.generate_moves()
        if len(legal_moves) == 0:
            king_square = self.board.piece_to_squares[(PieceType.KING, self.board.color_to_move)][0]
            if self.board.is_attacked(king_square):
                return - color.value * MATING_SCORE, None
            else:
                return 0.0, None

        legal_moves = self.reorder_moves(legal_moves)
        if color.is_max():
            best_score, best_move = - MATING_SCORE, legal_moves[0]
            for move in legal_moves:
                self.board.make_move(move)
                score = self.minimax(depth - 1, color.flip())[0]
                self.board.unmake_move()
                if score > best_score:
                    best_score, best_move = score, move
                if score >= beta:
                    break
                alpha = max(alpha, score)
        else:
            best_score, best_move = MATING_SCORE, legal_moves[0]
            for move in legal_moves:
                self.board.make_move(move)
                score = self.minimax(depth - 1, color.flip())[0]
                self.board.unmake_move()
                if score < best_score:
                    best_score, best_move = score, move
                if score <= alpha:
                    break
                beta = min(beta, score)
        return best_score, best_move  # best_score is an upper bound to the actual best score if color.is_max(), and a lower bound otherwise

    def negamax(self, depth, alpha, beta, color) -> (int, Optional[Move]):
        self.node_count += 1

        if self.board.is_3fold():
            return 0

        # access transposition table and check if we can return early
        old_depth, old_score, old_move, old_is_exact = self.query_trasposition_table()
        if old_depth is not None and old_depth >= depth:
            if old_is_exact == True or old_score >= beta:
                return old_score, old_move

        # check if exploration is over and return static evaluation
        if depth == 0:
            return self.board.static_evaluation() * color.value, None

        # generate legal moves and check if mate/stalemate has been reached
        legal_moves = self.board.generate_moves()
        if len(legal_moves) == 0:
            if self.board.is_attacked(self.board.piece_to_squares[(PieceType.KING, self.board.color_to_move)][0]):
                return -MATING_SCORE, None
            else:
                return 0, None

        # explore the tree one level deeper
        legal_moves = self.reorder_moves(legal_moves)
        best_score, best_move, is_exact = -MATING_SCORE, legal_moves[0], True
        for move in legal_moves:
            self.board.make_move(move)
            score = -self.negamax(depth - 1, -beta, -alpha, color.flip())[0]
            self.board.unmake_move()
            if score > best_score:
                if score > MATING_SCORE - 100:
                    best_score, best_move = score - 1, move
                else:
                    best_score, best_move = score, move

            alpha = max(alpha, score)
            if alpha >= beta:
                is_exact = False
                break

        # update transposition table and return. in general, best_score is a lower bound to the actual best score
        self.update_transposition_table(depth, best_score, best_move, is_exact)
        return best_score, best_move

    def stop_search(self):
        """
        Stop the search and return the best move found so far.
        :return: best move
        """
        # Note: this way we discard information from the current unfinished search.
        # TODO: use it somehow.
        return self.current_best_move

    def search(self, max_depth: int):
        """
        Search the best move for the current position.
        :param max_depth: maximum depth to search
        :return: best move
        """
        self.node_count = 0
        for depth in range(1, max_depth + 1):
            score, best_move = self.negamax(depth, -MATING_SCORE, MATING_SCORE, self.board.color_to_move)
            self.current_best_move = best_move
            print(f"info depth {depth} score cp {score} pv {best_move.to_string()} nodes {self.node_count}")

        return score, self.current_best_move


from test_mov_gen import data

if __name__ == '__main__':
    # board = Board.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    # eng = Engine(board)
    # depth = 10

    depth = 6

    # board = Board.from_startpos()
    # board = Board.from_fen("8/8/8/8/8/4K3/8/5Q1k b - - 24 13")
    # eng = Engine(board)
    # eng.vanilla_minimax(depth, COLOR.WHITE)
    # print(eng.node_count)

    board = Board.from_fen(data.various_fen[1])
    eng = Engine(board)
    # res = eng.vanilla_negamax(depth, COLOR.WHITE)

    res = eng.search(6)
    print(eng.node_count)
    print(eng.query_hits)

    print(res)

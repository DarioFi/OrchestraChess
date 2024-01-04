import random

from engine import Engine
from move import Move
from board import Board
from timer import Timer
from typing import Optional, List
import helpers

DEBUG = False


# static class
class OrchestraDirector:
    board: Optional[Board] = None
    timer: Optional[Timer] = None

    @classmethod
    def init_startpos(cls):
        if DEBUG:
            print("startpos")
        cls.board = Board.from_startpos()

    @classmethod
    def handle_command(cls, command, options):
        match command:
            case "uci":
                cls.uci_handle_uci()
            case "isready":
                cls.uci_handle_isready()
            case "ucinewgame":
                pass
            case "position":
                cls.uci_handle_position(options)
            case "go":
                cls.uci_handle_go(options)
            case "stop":
                cls.uci_handle_stop()
            case "quit":
                cls.uci_handle_quit()
            case "setoption":
                pass
            case _:
                if DEBUG:
                    raise NotImplementedError(command, options)

    @classmethod
    def uci_handle_uci(cls):
        print("id name Orchestra")
        print("id author Dario & Mattia")
        print("uciok")

    @classmethod
    def uci_handle_position(cls, options):
        if options[0:8] == "startpos":
            cls.init_startpos()
            w = options.split("moves")
            if len(w) > 1:
                cls.execute_moves(w[1])
        else:
            fen, moves = helpers.split_fen_moves(options)  # not sure it goes there
            cls.init_from_fen(fen)
            cls.execute_moves(moves)

    @classmethod
    def execute_moves(cls, param):
        moves: List[str] = param.split()
        for mov_str in moves:
            mov: Move = Move.from_string(mov_str, cls.board)
            cls.board.make_move(mov)

    @classmethod
    def init_from_fen(cls, fen):
        # todo: review this because the string editing is done in two different places
        # fen = options[options.find("[") + 1:options.find("]")]
        cls.board = Board.from_fen(fen)

    @classmethod
    def uci_handle_isready(cls):
        print("readyok")

    @classmethod
    def uci_handle_go(cls, options):
        op_list = options.split()
        i = 0
        while i < len(op_list):
            match op_list[i]:
                case "wtime":
                    i += 2
                case "btime":
                    i += 2
                case "winc":
                    i += 2
                case "binc":
                    i += 2
                case "depth":
                    i += 2
                case "nodes":
                    i += 2
                case "infinite":
                    i += 1
                case _:
                    i += 1

        legal_moves = cls.board.generate_moves()

        eng = Engine(cls.board)
        mov = eng.search(6)[1]

        print("bestmove " + mov.to_string())
        return mov

    @classmethod
    def uci_handle_stop(cls):
        raise NotImplementedError

    @classmethod
    def uci_handle_quit(cls):
        exit(0)


if __name__ == '__main__':
    OrchestraDirector.handle_command("position",  "fen 8/8/8/K7/8/8/7Q/1k6 b - - 0 1 moves b1c1")
    # OrchestraDirector.handle_command("position", "startpos moves e2e4")
    OrchestraDirector.handle_command("go", "movetime 1000")

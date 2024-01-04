import copy
from typing import List, Tuple, Optional, Set, Dict, Iterable

import constants
from move import Move
from constants import PieceType, COLOR, Square

import random

sliding_pieces = [PieceType.BISHOP, PieceType.ROOK, PieceType.QUEEN]
sliding_diagonal = [PieceType.BISHOP, PieceType.QUEEN]
sliding_straight = [PieceType.ROOK, PieceType.QUEEN]

diagonal_pattern = [(1, 1), (-1, 1), (1, -1), (-1, -1)]
straight_pattern = [(1, 0), (-1, 0), (0, 1), (0, -1)]
knight_pattern = [(1, 2), (2, 1), (-1, 2), (2, -1), (1, -2), (-2, 1), (-1, -2), (-2, -1)]
king_pattern = [(1, 0), (0, 1), (-1, 0), (0, -1), (1, 1), (-1, 1), (1, -1), (-1, -1)]


# note: we ignore three-fold repetition for now.
class Board:
    def __init__(self):
        self.piece_to_squares: Dict[Tuple[PieceType, COLOR]: Square] = {
            (x, y): [] for x in PieceType for y in COLOR
        }
        self.bitboard: List[List[Tuple[Optional[PieceType], Optional[COLOR]]]] = [
            [(None, None) for _ in range(8)] for _ in range(8)
        ]

        self.zobrist = ZobristHashHandler()

        self.move_history_stack = []
        self.castling_rights_stack = []
        self.en_passant_stack = []
        self.move_50_rule_stack = []
        self.zobrist_history = []

        self.move_50_rule = 0  # half-moves since last irreversible move
        self.castling_rights = CastlingRights()
        self.en_passant: Optional[Square] = None  # square where capturable pawn is
        self.color_to_move = COLOR.WHITE

    @classmethod
    def from_fen(cls, fen):
        """
        Parse input string in fen format and use its info to initialize a Board object.
        """
        board = cls()
        current_rank = 7
        current_file = 0
        fen_parts = fen.split(" ")  # [pieces, color, castling rights, en passant, 50 move rule, total half moves]

        for c in fen_parts[0]:
            if c == "/":
                current_rank -= 1
                current_file = 0
            elif c.isdigit():
                current_file += int(c)
            else:
                if c.upper() == c:
                    color = COLOR.WHITE
                else:
                    color = COLOR.BLACK

                piece_type = PieceType.from_char(c.lower())
                board.bitboard[current_rank][current_file] = (
                    piece_type, color)
                board.piece_to_squares[(piece_type, color)].append(Square(current_file, current_rank))
                current_file += 1

        board.color_to_move = COLOR.WHITE if fen_parts[1] == "w" else COLOR.BLACK
        board.castling_rights = CastlingRights.from_string(fen_parts[2])
        en_passant = fen_parts[3]
        if en_passant == "-":
            board.en_passant = None
        else:
            board.en_passant = Square.from_string_algebraic(en_passant)
        board.move_50_rule = int(fen_parts[4])

        board.zobrist.initialize_hash(board)

        return board

    @classmethod
    def from_startpos(cls):
        return cls.from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")

    def make_move(self, move: Move):
        """
        Executes a move on the board and flips the color to move.
        :param move: move to make
        :return:
        """
        self.move_history_stack.append(move)
        self.en_passant_stack.append(self.en_passant)
        self.castling_rights_stack.append(self.castling_rights.clone())
        self.zobrist.update_hash(move, self.color_to_move)
        self.zobrist_history.append(self.zobrist.hash)
        self.move_50_rule_stack.append(self.move_50_rule)
        self.move_50_rule += 1
        self.en_passant = None
        match move.piece_moved:
            case PieceType.KING:
                self.castling_rights.moved_king(self.color_to_move)
                if abs(move.from_square.file - move.to_square.file) > 1:
                    self.make_castle(move)
                else:
                    self.make_simple_move(move)

            case PieceType.PAWN:
                self.move_50_rule = 0
                # case capture or en passant
                if move.promotion is not None:
                    self.make_promotion(move)
                elif move.from_square.file != move.to_square.file:
                    if self.bitboard[move.to_square.rank][move.to_square.file] == (None, None):
                        self.make_en_passant(move)
                    elif move.promotion is None:
                        self.make_simple_move(move)


                else:
                    if abs(move.to_square.rank - move.from_square.rank) == 2:
                        tg = 1 if self.color_to_move == COLOR.WHITE else -1
                        self.en_passant = Square(move.to_square.file, move.to_square.rank - tg)
                    self.make_simple_move(move)
            case PieceType.ROOK:
                self.castling_rights.moved_rook(self.color_to_move, move.from_square)
                self.make_simple_move(move)
            case _:
                self.make_simple_move(move)

        self.color_to_move = self.color_to_move.flip()

    def is_3fold(self):
        return self.zobrist_history.count(self.zobrist.hash) > 2

    def make_simple_move(self, move: Move):
        """
        Makes a simple move that does not involve anything but moving a piece and possibly capturing what's on the landing square
        :param move:
        :return:
        """
        p, c = self.bitboard[move.to_square.rank][move.to_square.file]

        if p is not None:  # remove opponent piece
            self.move_50_rule = 0
            self.piece_to_squares[(p, c)].remove(move.to_square)

        self.piece_to_squares[(move.piece_moved, self.color_to_move)].remove(move.from_square)
        self.piece_to_squares[(move.piece_moved, self.color_to_move)].append(move.to_square)

        # set to_square to piece_moved and erase from_square
        self.bitboard[move.from_square.rank][move.from_square.file] = (None, None)
        self.bitboard[move.to_square.rank][move.to_square.file] = (move.piece_moved, self.color_to_move)

    def make_castle(self, move):
        """
        Execute a castle move.
        :param move:
        :return:
        """
        if move.to_square.file == 2:
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].remove(Square(0, move.from_square.rank))
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].append(Square(3, move.from_square.rank))
            self.bitboard[move.from_square.rank][3] = self.bitboard[move.from_square.rank][0]
            self.bitboard[move.from_square.rank][0] = (None, None)

            self.piece_to_squares[(PieceType.KING, self.color_to_move)] = [move.to_square]
            self.bitboard[move.to_square.rank][move.to_square.file] = (PieceType.KING, self.color_to_move)
            self.bitboard[move.from_square.rank][move.from_square.file] = (None, None)

        else:
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].remove(Square(7, move.from_square.rank))
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].append(Square(5, move.from_square.rank))
            self.bitboard[move.from_square.rank][5] = self.bitboard[move.from_square.rank][7]
            self.bitboard[move.from_square.rank][7] = (None, None)

            self.piece_to_squares[(PieceType.KING, self.color_to_move)] = [move.to_square]
            self.bitboard[move.to_square.rank][move.to_square.file] = (PieceType.KING, self.color_to_move)
            self.bitboard[move.from_square.rank][move.from_square.file] = (None, None)

    def unmake_move(self):
        """
        Unmake a move and revert the state of the board to the previous one
        :return:
        """
        self.move_50_rule = self.move_50_rule_stack.pop()
        self.en_passant = self.en_passant_stack.pop()
        self.castling_rights = self.castling_rights_stack.pop()
        self.color_to_move = self.color_to_move.flip()
        self.zobrist_history.pop()

        move = self.move_history_stack.pop()
        self.zobrist.update_hash(move, self.color_to_move)

        match move.piece_moved:
            case PieceType.KING:
                if abs(move.from_square.file - move.to_square.file) > 1:
                    self.unmake_castle(move)
                else:
                    self.unmake_classic_move(move)
            case PieceType.PAWN:
                # case capture or en passant
                if move.from_square.file != move.to_square.file:
                    if move.piece_captured is None:
                        self.unmake_en_passant(move)
                    else:
                        self.unmake_classic_move(move)
                elif move.promotion is not None:
                    self.unmake_promotion(move)
                else:
                    self.unmake_classic_move(move)
            case _:
                self.unmake_classic_move(move)

        # special cases: en passant, castling, promotion

    def generate_moves(self) -> List[Move]:
        """
        Generate all legal moves in the position
        :return:
        """
        pinned_pieces: Set[Square] = self.generate_pinned()
        king = self.piece_to_squares[(PieceType.KING, self.color_to_move)][0]
        attacked_bitboard = self.generate_attacked_squares(self.color_to_move.flip())
        attacking_king = attacked_bitboard[king.rank][king.file]
        is_check = len(attacking_king) > 0

        moves: List[Move] = []

        # king
        king = self.piece_to_squares[(PieceType.KING, self.color_to_move)][0]
        pseudo_legal = self.generate_adjacent_moves_pseudo_legal(king, king_pattern)
        for mov in pseudo_legal:
            if not attacked_bitboard[mov.to_square.rank][mov.to_square.file]:
                moves.append(mov)
        if not is_check:
            castles = self.generate_castling_legal(king, attacked_bitboard)
            moves.extend(castles)

        if len(attacking_king) > 1:
            # if there are more than one attacking king, we can only move the king
            return moves

        # bishop
        bishops = copy.copy(self.piece_to_squares[(PieceType.BISHOP, self.color_to_move)])
        for square in bishops:
            pseudo_legal = self.generate_sliding_moves_pseudo_legal(square, diagonal_pattern)
            if square not in pinned_pieces and not is_check:
                moves.extend(pseudo_legal)
            else:
                moves.extend(self.check_pseudo_legal_moves(pseudo_legal))

        # rook
        rooks = copy.copy(self.piece_to_squares[(PieceType.ROOK, self.color_to_move)])
        for square in rooks:
            pseudo_legal = self.generate_sliding_moves_pseudo_legal(square, straight_pattern)
            if square not in pinned_pieces and not is_check:
                moves.extend(pseudo_legal)
            else:
                moves.extend(self.check_pseudo_legal_moves(pseudo_legal))

        # queen
        queens = copy.copy(self.piece_to_squares[(PieceType.QUEEN, self.color_to_move)])
        for square in queens:
            pseudo_legal = self.generate_sliding_moves_pseudo_legal(square, straight_pattern + diagonal_pattern)
            if square not in pinned_pieces and not is_check:
                moves.extend(pseudo_legal)
            else:
                moves.extend(self.check_pseudo_legal_moves(pseudo_legal))

        # knight
        knights = copy.copy(self.piece_to_squares[(PieceType.KNIGHT, self.color_to_move)])
        for square in knights:
            if square in pinned_pieces:
                continue
            elif not is_check:
                moves.extend(self.generate_adjacent_moves_pseudo_legal(square, knight_pattern))
            else:
                mv = self.generate_adjacent_moves_pseudo_legal(square, knight_pattern)
                moves.extend(self.check_pseudo_legal_moves(mv))

        # pawn
        pawns = copy.copy(self.piece_to_squares[(PieceType.PAWN, self.color_to_move)])
        # ordering of this list, which breaks things

        for square in pawns:
            pseudo_legal = self.generate_pawns_pushes_captures_promotions_pseudo_legal(square)
            if square not in pinned_pieces and not is_check:
                moves.extend(pseudo_legal)
            else:
                moves.extend(self.check_pseudo_legal_moves(pseudo_legal))

            en_pass = self.generate_en_passant_pseudo_legal(square)
            if self.check_legality_en_passant(en_pass):
                moves.extend(en_pass)
            for m in moves:
                if m.from_square.rank == m.to_square.rank and m.piece_moved == PieceType.PAWN:
                    debug = 1
                    print("pretty bad if you ask me")
        return moves

    def generate_pinned(self):
        """
        Assume that the king is not in check. Sliding pieces go through the king for move generation purposes.
        :return:
        """
        pinned_squares = set()

        pinned_squares.update(
            self.gen_pinned_sliding(sliding_diagonal, diagonal_pattern)
        )
        pinned_squares.update(
            self.gen_pinned_sliding(sliding_straight, straight_pattern)
        )
        return pinned_squares

    def generate_adjacent_moves_pseudo_legal(self, square: Square, pattern):
        moves = []
        piece_moved = self.bitboard[square.rank][square.file][0]

        for direction in pattern:
            w = square + direction
            if not w.is_valid():
                continue

            piece_captured, color_captured = self.bitboard[w.rank][w.file]
            if color_captured == self.color_to_move:
                continue
            move = Move(
                from_square=square,
                to_square=w,
                piece_moved=piece_moved,
                piece_captured=piece_captured,
                promotion=None
            )

            moves.append(move)
        return moves

    def generate_sliding_moves_pseudo_legal(self, square: Square, list_directions):
        piece = self.bitboard[square.rank][square.file][0]
        moves = []
        for direction in list_directions:
            new_square = square.clone() + direction
            while new_square.is_valid():
                piece_f, color_f = self.bitboard[new_square.rank][new_square.file]
                if color_f != self.color_to_move:
                    moves.append(Move(
                        piece_moved=piece,
                        piece_captured=piece_f,
                        from_square=square,
                        to_square=new_square,
                        promotion=None
                    ))
                if piece_f is not None:
                    break

                new_square = new_square + direction

        return moves

    def generate_castling_pseudo_legal(self):
        raise NotImplementedError

    def generate_pawns_pushes_capassptures_promotions_pseudo_legal(self, square: Square):

        pawn_direction = 1 if self.color_to_move == COLOR.WHITE else -1  # direction of the pawn
        back_rank = 1 if self.color_to_move == COLOR.WHITE else 6  # rank of the back rank
        promotion_rank = 6 if self.color_to_move == COLOR.WHITE else 1  # rank of the promotion rank
        opponent_color = self.color_to_move.flip()
        moves = []

        # single push
        if square.rank != promotion_rank:
            if self.bitboard[square.rank + pawn_direction][square.file][0] is None:
                moves.append(Move(
                    piece_moved=PieceType.PAWN,
                    piece_captured=None,
                    from_square=square,
                    to_square=Square(square.file, square.rank + pawn_direction),
                    promotion=None
                ))
                # double push
                if square.rank == back_rank and self.bitboard[square.rank + 2 * pawn_direction][square.file][0] is None:
                    moves.append(Move(
                        piece_moved=PieceType.PAWN,
                        piece_captured=None,
                        from_square=square,
                        to_square=Square(square.file, square.rank + 2 * pawn_direction),
                        promotion=None
                    ))
        else:  # promotion straight
            if square.rank == promotion_rank:
                if self.bitboard[square.rank + pawn_direction][square.file][0] is None:
                    for piece in (PieceType.KNIGHT, PieceType.BISHOP, PieceType.ROOK, PieceType.QUEEN):
                        moves.append(Move(
                            piece_moved=PieceType.PAWN,
                            piece_captured=None,
                            from_square=square,
                            to_square=Square(square.file, square.rank + pawn_direction),
                            promotion=piece
                        ))

        # captures
        if square.file != 0:
            piece_captured, color_captured = self.bitboard[square.rank + pawn_direction][square.file - 1]
            if color_captured == opponent_color:
                assert piece_captured is not None
                assert self.bitboard[square.rank + pawn_direction][square.file - 1][0] is not None

                if square.rank == promotion_rank:
                    for piece in (PieceType.KNIGHT, PieceType.BISHOP, PieceType.ROOK, PieceType.QUEEN):
                        moves.append(Move(
                            piece_moved=PieceType.PAWN,
                            piece_captured=piece_captured,
                            from_square=square,
                            to_square=Square(square.file - 1, square.rank + pawn_direction),
                            promotion=piece
                        ))
                else:
                    moves.append(Move(
                        piece_moved=PieceType.PAWN,
                        piece_captured=piece_captured,
                        from_square=square,
                        to_square=Square(square.file - 1, square.rank + pawn_direction),
                        promotion=None
                    ))
        if square.file != 7:
            piece_captured, color_captured = self.bitboard[square.rank + pawn_direction][square.file + 1]
            if color_captured == opponent_color:
                if square.rank == promotion_rank:
                    for piece in (PieceType.KNIGHT, PieceType.BISHOP, PieceType.ROOK, PieceType.QUEEN):
                        moves.append(Move(
                            piece_moved=PieceType.PAWN,
                            piece_captured=piece_captured,
                            from_square=square,
                            to_square=Square(square.file + 1, square.rank + pawn_direction),
                            promotion=piece
                        ))
                else:
                    moves.append(Move(
                        piece_moved=PieceType.PAWN,
                        piece_captured=piece_captured,
                        from_square=square,
                        to_square=Square(square.file + 1, square.rank + pawn_direction),
                        promotion=None
                    ))

        for m in moves:
            if m.from_square.rank == m.to_square.rank:
                debug = 1
                print("pretty bad if you ask me")
        return moves

    def generate_en_passant_pseudo_legal(self, square: Square):  # decide if this returns a list or a single object
        # en passant
        direction = 1 if self.color_to_move == COLOR.WHITE else -1
        if self.en_passant is not None:
            if self.en_passant.rank == square.rank + direction and abs(self.en_passant.file - square.file) == 1:
                return [Move(
                    piece_moved=PieceType.PAWN,
                    piece_captured=None,
                    from_square=square,
                    to_square=self.en_passant,
                    promotion=None
                )]
        return []

    def generate_attacked_squares(self, attacker_color: COLOR):
        """
        Generate all the squares attacked by the attacker_color.
        :return:
        """
        bitboard: List[List[List[PieceType]]] = [[[] for _ in range(8)] for _ in range(8)]

        # pawns
        if attacker_color == COLOR.WHITE:
            tg = 1
        else:
            tg = -1
        # rank goes up
        for square in self.piece_to_squares[(PieceType.PAWN, attacker_color)]:
            if square.file != 0:
                bitboard[square.rank + tg][square.file - 1].append(PieceType.PAWN)
            if square.file != 7:
                bitboard[square.rank + tg][square.file + 1].append(PieceType.PAWN)

        # knights
        for square in self.piece_to_squares[(PieceType.KNIGHT, attacker_color)]:
            for direction in knight_pattern:
                attack_square = square + direction
                if attack_square.is_valid():
                    bitboard[attack_square.rank][attack_square.file].append(PieceType.KNIGHT)

        # king
        for square in self.piece_to_squares[(PieceType.KING, attacker_color)]:
            for direction in king_pattern:
                attack_square = square + direction
                if attack_square.is_valid():
                    bitboard[attack_square.rank][attack_square.file].append(PieceType.KING)

        # sliding
        for square in self.piece_to_squares[(PieceType.BISHOP, attacker_color)]:
            for direction in diagonal_pattern:
                self.fill_attack_bitboard(square, direction, bitboard, attacker_color)

        for square in self.piece_to_squares[(PieceType.ROOK, attacker_color)]:
            for direction in straight_pattern:
                self.fill_attack_bitboard(square, direction, bitboard, attacker_color)

        for square in self.piece_to_squares[(PieceType.QUEEN, attacker_color)]:
            for direction in straight_pattern + diagonal_pattern:
                self.fill_attack_bitboard(square, direction, bitboard, attacker_color)

        return bitboard

    def fill_attack_bitboard(self, square: Square, direction, bitboard, attacker_color: COLOR):
        """ updates bitboard in place """
        piece_att = self.bitboard[square.rank][square.file][0]
        new_square = square.clone() + direction
        while new_square.is_valid():

            bitboard[new_square.rank][new_square.file].append(piece_att)

            piece, color = self.bitboard[new_square.rank][new_square.file]
            if piece is not None:
                if piece == PieceType.KING and color != attacker_color:
                    pass
                else:
                    break

            new_square = new_square + direction

    def gen_pinned_sliding(self, list_sliding_pieces, list_sliding_directions):
        """
        Generate all the pieces pinned by attacks in direction list_directions (pieces with these patterns are in
        list_pieces_types)
        :param list_sliding_pieces:
        :param list_sliding_directions:
        :return:
        """
        pinned_squares = set()
        king_square: Square = self.piece_to_squares[(PieceType.KING, self.color_to_move)][0]

        for direction in list_sliding_directions:
            encountered = None
            new_square = king_square.clone()

            while True:
                new_square = new_square + direction
                if not new_square.is_valid():
                    break

                piece, color = self.bitboard[new_square.rank][new_square.file]
                if piece is None: continue

                if color == self.color_to_move:
                    if encountered is not None: break
                    encountered = new_square
                else:
                    if piece in list_sliding_pieces:
                        pinned_squares.add(encountered)
                    else:
                        break

        return pinned_squares

    def is_attacked(self, square: Square):
        """
        Check if a square is attacked by the opponent. This is slow and should only be used once if multiple attacks
        need to be checked, it is better to generate the full bitboard
        # todo: instead of using self.color_to_move use a custom color as argument
        :param square:
        :return:
        """
        # check sliding, pawns, knights, kings
        opponent_color = self.color_to_move.flip()

        # pawns
        if self.color_to_move == COLOR.WHITE and square.rank != 7:
            # rank goes up
            if square.file != 0:
                if self.bitboard[square.rank + 1][square.file - 1] == (PieceType.PAWN, COLOR.BLACK):
                    return True
            if square.file != 7:
                if self.bitboard[square.rank + 1][square.file + 1] == (PieceType.PAWN, COLOR.BLACK):
                    return True
        elif square.rank != 0:
            if square.file != 0:
                if self.bitboard[square.rank - 1][square.file - 1] == (PieceType.PAWN, COLOR.WHITE):
                    return True
            if square.file != 7:
                if self.bitboard[square.rank - 1][square.file + 1] == (PieceType.PAWN, COLOR.WHITE):
                    return True

        # knights
        for direction in knight_pattern:
            attack_square = square + direction
            if attack_square.is_valid():
                if self.bitboard[attack_square.rank][attack_square.file] == (PieceType.KNIGHT, opponent_color):
                    return True

        # king
        other_king_square: Square = self.piece_to_squares[(PieceType.KING, opponent_color)][0]
        distance_rank = abs(square.rank - other_king_square.rank)  # maybe functions to do this?
        distance_file = abs(square.file - other_king_square.file)
        if max(distance_rank, distance_file) < 2:  # could be compressed but why?
            return True

        # sliding
        if self.is_attacked_sliding(square, sliding_diagonal, diagonal_pattern):
            return True

        if self.is_attacked_sliding(square, sliding_straight, straight_pattern):
            return True

        return False

    def is_attacked_sliding(self, square: Square, list_sliding_pieces, list_sliding_directions):
        opponent_color = self.color_to_move.flip()

        for direction in list_sliding_directions:
            new_square = square.clone()
            while True:
                new_square = new_square + direction
                if not new_square.is_valid():
                    break

                piece, color = self.bitboard[new_square.rank][new_square.file]
                if piece is None:
                    continue

                if color == opponent_color and piece in list_sliding_pieces:
                    return True
                else:
                    break
        return False

    def static_evaluation(self):
        values = constants.values
        score = 0
        n_pieces = 0
        for l in self.piece_to_squares.values():
            n_pieces += len(l)

        end_game_mul = 1 - n_pieces / 32

        for piece, color in self.piece_to_squares:
            for square in self.piece_to_squares[(piece, color)]:
                n_pieces += 1
                score += values[piece] * color.value

                file = square.file
                rank = square.rank if color == COLOR.BLACK else 7 - square.rank

                score += constants.complete_table[piece][rank][file] * color.value * (1 - end_game_mul)
                score += constants.complete_table_endgame[piece][rank][file] * color.value * end_game_mul

        return score

    def pawn_structure_eval(self):
        pawns_w = self.piece_to_squares[(PieceType.PAWN, COLOR.WHITE)]
        pawns_b = self.piece_to_squares[(PieceType.PAWN, COLOR.BLACK)]

        doubles = 0
        passers = 0

    def check_pseudo_legal_moves(self, moves: List[Move]) -> Iterable[Move]:
        """
        This function checks which moves in the list moves are legal. Assume all the moves here come from the same piece
        so that some speedup can be implemented in the future. Basic implementation might be just bruteforcing all the
        moves with is_attacked().
        :param moves:
        :return:
        """

        # todo: this implementation is idiotic we need to keep track of pinned pieces and the direction they are pinned
        # but for now we do this
        valid = []
        for move in moves:
            self.make_move(move)
            self.color_to_move = self.color_to_move.flip()
            if not self.is_attacked(self.piece_to_squares[(PieceType.KING, self.color_to_move)][0]):
                valid.append(move)
            self.color_to_move = self.color_to_move.flip()

            self.unmake_move()
        return valid

    def check_legality_en_passant(self, en_pass):
        return self.check_pseudo_legal_moves(en_pass) != []

    def generate_castling_legal(self, square: Square, attacked_bitboard):
        """
        Generate castling moves that are legal.
        todo: completely generated by the copilot, needs to be checked but looks good.
        :param square:
        :param attacked_bitboard:
        :return:
        """
        moves = []
        rank = square.rank
        if self.color_to_move == COLOR.WHITE:
            if self.castling_rights.white_king_side and self.bitboard[0][7] == (PieceType.ROOK, COLOR.WHITE):
                if not attacked_bitboard[0][5] and not attacked_bitboard[0][6] and self.bitboard[0][5][0] is None and \
                        self.bitboard[0][6][0] is None:
                    moves.append(Move(
                        piece_moved=PieceType.KING,
                        piece_captured=None,
                        from_square=square,
                        to_square=Square(6, 0),
                        promotion=None
                    ))
            if self.castling_rights.white_queen_side and self.bitboard[0][0] == (PieceType.ROOK, COLOR.WHITE):
                if not attacked_bitboard[0][2] and not attacked_bitboard[0][3] and self.bitboard[0][1][0] is None and \
                        self.bitboard[0][2][0] is None and self.bitboard[0][3][0] is None:
                    moves.append(Move(
                        piece_moved=PieceType.KING,
                        piece_captured=None,
                        from_square=square,
                        to_square=Square(2, 0),
                        promotion=None
                    ))
        else:
            if self.castling_rights.black_king_side and self.bitboard[7][7] == (PieceType.ROOK, COLOR.BLACK):
                if not attacked_bitboard[7][5] and not attacked_bitboard[7][6] and self.bitboard[7][5][0] is None and \
                        self.bitboard[7][6][0] is None:
                    moves.append(Move(
                        piece_moved=PieceType.KING,
                        piece_captured=None,
                        from_square=square,
                        to_square=Square(6, 7),
                        promotion=None
                    ))
            if self.castling_rights.black_queen_side and self.bitboard[7][0] == (PieceType.ROOK, COLOR.BLACK):
                if not attacked_bitboard[7][2] and not attacked_bitboard[7][3] and self.bitboard[7][1][0] is None and \
                        self.bitboard[7][2][0] is None and self.bitboard[7][3][0] is None:
                    moves.append(Move(
                        piece_moved=PieceType.KING,
                        piece_captured=None,
                        from_square=square,
                        to_square=Square(2, 7),
                        promotion=None
                    ))
        return moves

    def make_en_passant(self, move):
        direction = 1 if self.color_to_move == COLOR.WHITE else -1

        self.bitboard[move.to_square.rank][move.to_square.file] = (PieceType.PAWN, self.color_to_move)
        self.bitboard[move.from_square.rank][move.from_square.file] = (None, None)
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move)].remove(move.from_square)
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move)].append(move.to_square)

        # remove old pawn
        self.bitboard[move.to_square.rank - direction][move.to_square.file] = (None, None)
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move.flip())].remove(
            Square(move.to_square.file, move.to_square.rank - direction))

    def make_promotion(self, move):
        self.bitboard[move.to_square.rank][move.to_square.file] = (move.promotion, self.color_to_move)
        self.piece_to_squares[(move.promotion, self.color_to_move)].append(move.to_square)

        self.bitboard[move.from_square.rank][move.from_square.file] = (None, None)
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move)].remove(move.from_square)

        if move.piece_captured is not None:
            self.piece_to_squares[(move.piece_captured, self.color_to_move.flip())].remove(move.to_square)

    def unmake_classic_move(self, move):
        """assumes the color that needs to play is the one that played the move"""
        # reset to_square to original state
        opponent_color = self.color_to_move.flip()
        if move.piece_captured is not None:
            self.bitboard[move.to_square.rank][move.to_square.file] = (move.piece_captured, self.color_to_move.flip())
            self.piece_to_squares[(move.piece_captured, opponent_color)].append(move.to_square)
        else:
            self.bitboard[move.to_square.rank][move.to_square.file] = (None, None)

        # reset from_square to original state
        self.bitboard[move.from_square.rank][move.from_square.file] = (move.piece_moved, self.color_to_move)

        if move.promotion is not None:
            self.piece_to_squares[(move.promotion, self.color_to_move)].remove(move.to_square)
        else:
            self.piece_to_squares[(move.piece_moved, self.color_to_move)].remove(move.to_square)
        self.piece_to_squares[(move.piece_moved, self.color_to_move)].append(move.from_square)

    def unmake_en_passant(self, move):
        self.bitboard[move.to_square.rank][move.to_square.file] = (None, None)
        self.bitboard[move.from_square.rank][move.from_square.file] = (move.piece_moved, self.color_to_move)
        self.piece_to_squares[(move.piece_moved, self.color_to_move)].remove(move.to_square)
        self.piece_to_squares[(move.piece_moved, self.color_to_move)].append(move.from_square)

        # re add missing pawn
        direction = 1 if self.color_to_move == COLOR.WHITE else -1
        self.bitboard[move.to_square.rank - direction][move.to_square.file] = (
            PieceType.PAWN, self.color_to_move.flip())
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move.flip())].append(
            Square(move.to_square.file, move.to_square.rank - direction))

    def unmake_promotion(self, move):
        # remove the new promoted piece from the board
        self.bitboard[move.to_square.rank][move.to_square.file] = (None, None)
        self.piece_to_squares[(move.promotion, self.color_to_move)].remove(move.to_square)

        # add the pawn back
        self.bitboard[move.from_square.rank][move.from_square.file] = (PieceType.PAWN, self.color_to_move)
        self.piece_to_squares[(PieceType.PAWN, self.color_to_move)].append(move.from_square)

    def unmake_castle(self, move):
        if move.to_square.file == 2:

            # handle rook
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].remove(Square(3, move.from_square.rank))
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].append(Square(0, move.from_square.rank))

            self.bitboard[move.from_square.rank][0] = (PieceType.ROOK, self.color_to_move)
            self.bitboard[move.from_square.rank][3] = (None, None)

            # handle king
            self.piece_to_squares[(PieceType.KING, self.color_to_move)] = [move.from_square]
            self.bitboard[move.from_square.rank][move.from_square.file] = (PieceType.KING, self.color_to_move)
            self.bitboard[move.to_square.rank][move.to_square.file] = (None, None)

        else:
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].remove(Square(5, move.from_square.rank))
            self.piece_to_squares[(PieceType.ROOK, self.color_to_move)].append(Square(7, move.from_square.rank))

            self.bitboard[move.from_square.rank][7] = (PieceType.ROOK, self.color_to_move)
            self.bitboard[move.from_square.rank][5] = (None, None)

            self.piece_to_squares[(PieceType.KING, self.color_to_move)] = [move.from_square]
            self.bitboard[move.from_square.rank][move.from_square.file] = (PieceType.KING, self.color_to_move)
            self.bitboard[move.to_square.rank][move.to_square.file] = (None, None)


class CastlingRights:
    def __init__(self, wks=False, wqs=False, bks=False, bqs=False):
        self.white_king_side = wks
        self.white_queen_side = wqs
        self.black_king_side = bks
        self.black_queen_side = bqs

    @classmethod
    def from_string(cls, s):
        castling_rights = cls()
        for c in s:
            if c == "K":
                castling_rights.white_king_side = True
            elif c == "Q":
                castling_rights.white_queen_side = True
            elif c == "k":
                castling_rights.black_king_side = True
            elif c == "q":
                castling_rights.black_queen_side = True
            elif c == "-":
                pass
            else:
                raise ValueError
        return castling_rights

    def update_rights(self, move: Move):
        raise NotImplementedError

    def moved_king(self, color):
        if color == COLOR.WHITE:
            self.white_king_side = False
            self.white_queen_side = False
        else:
            self.black_king_side = False
            self.black_queen_side = False

    def clone(self):
        return CastlingRights(
            wks=self.white_king_side,
            wqs=self.white_queen_side,
            bks=self.black_king_side,
            bqs=self.black_queen_side
        )

    def moved_rook(self, color, square):
        if color == COLOR.WHITE:
            if square == Square(0, 0):
                self.white_queen_side = False
            elif square == Square(7, 0):
                self.white_king_side = False
        else:
            if square == Square(0, 7):
                self.black_queen_side = False
            elif square == Square(7, 7):
                self.black_king_side = False


class ZobristHashHandler:
    def __init__(self, n_bits: int = 64, seed: int = 0) -> None:
        self.table, self.black_to_move = self.generate_zobrist_table(n_bits=n_bits, seed=seed)

    @staticmethod
    def generate_zobrist_table(n_bits, seed):
        random.seed(seed)
        table = [[random.getrandbits(n_bits) for _ in range(12)] for _ in range(64)]
        black_to_move = random.getrandbits(n_bits)
        return table, black_to_move

    @staticmethod
    def get_table_idxs(piece: PieceType, color: COLOR, square: Square):
        return square.rank * 8 + square.file, piece.value + 3 * (color.value + 1)

    def initialize_hash(self, board: Board) -> None:
        h = 0
        if board.color_to_move == COLOR.BLACK:
            h ^= self.black_to_move
        for rank in range(8):
            for file in range(8):
                if board.bitboard[rank][file][0] is None:
                    continue
                piece, color = board.bitboard[rank][file]
                square = Square(file, rank)
                i, j = self.get_table_idxs(piece, color, square)
                h ^= self.table[i][j]
        self.hash = h

    def get_hash(self):
        return self.hash

    def update_hash(self, move: Move, color_to_move: COLOR):
        self.hash ^= self.black_to_move
        if move.piece_captured is not None:
            i, j = self.get_table_idxs(move.piece_captured, color_to_move.flip(), move.to_square)
            self.hash ^= self.table[i][j]
        i, j = self.get_table_idxs(move.piece_moved, color_to_move, move.from_square)
        self.hash ^= self.table[i][j]
        i, j = self.get_table_idxs(move.piece_moved, color_to_move, move.to_square)
        self.hash ^= self.table[i][j]


if __name__ == "__main__":
    zobrist = ZobristHashHandler()
    fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQ - 0 1"
    board = Board.from_fen(fen)
    move = Move.from_string("e2e4", board)

    print(board.zobrist.get_hash())
    board.make_move(move)
    print(board.zobrist.get_hash())
    board.unmake_move()
    print(board.zobrist.get_hash())

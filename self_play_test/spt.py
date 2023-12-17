import random
import subprocess
import time
from typing import List

move_time = 1000
MAX_MOVES = 150 * 2
CUTOFF = 1800


def send_command(command, engine):
    # Send a UCI command to the Rust executable
    engine.stdin.write(command + '\n')
    engine.stdin.flush()


def read_response(engine):
    x = ""
    info_str = None
    while not x.startswith("bestmove"):
        if x.startswith("info"):
            info_str = x
        if "panic" in x:
            raise Exception("Panic in engine")
        x = engine.stdout.readline().strip()
    return x, info_str


def ask_move(engine):
    send_command("go movetime " + str(move_time), engine)
    return read_response(engine)


def sim_game(e1, e2, n1, n2):
    moves: List[str] = []
    white_last_score = None
    black_last_score = None

    zrs_since = 0

    send_command("position startpos" + " ".join(moves), e1)
    bm, info = ask_move(e1)
    moves.append(bm.split(" ")[1])
    print(info, bm)

    while True:
        if white_last_score is not None: zrs_since += 1
        send_command("position startpos moves " + " ".join(moves), e2)
        bm, info = ask_move(e2)
        moves.append(bm.split(" ")[1])
        if info: print("Black: ", n2, ": ", info)

        if info is not None and "cp" in info:
            sc = int(info.split("score cp ")[1].split(" ")[0])
            if abs(sc) > 20:
                zrs_since = 0
            if black_last_score is None and abs(sc) > 200:
                print(" ".join(moves))
                return None  # means opening was too decisive
            black_last_score = sc

        if info is not None and "mate" in info:  # mate from e2 perspective
            sc = int(info.split("score mate ")[1].split(" ")[0])
            if sc > 0:
                return -1
            else:
                return 1

        send_command("position startpos moves " + " ".join(moves), e1)
        bm, info = ask_move(e1)
        moves.append(bm.split(" ")[1])
        if info: print("White: ", n1, ": ", info)
        if info is not None and "cp" in info:
            sc = int(info.split("score cp ")[1].split(" ")[0])
            if abs(sc) > 20:
                zrs_since = 0
            if white_last_score is None and abs(sc) > 200:
                print(" ".join(moves))
                return None  # means opening was too decisive
            white_last_score = sc

        print(f"{white_last_score=} {black_last_score=}", "Moves: ", len(moves))

        if white_last_score is not None and black_last_score is not None:
            if white_last_score > CUTOFF and black_last_score < -CUTOFF:
                return 1
            if black_last_score > CUTOFF and white_last_score < -CUTOFF:
                return -1

        if zrs_since > 10:
            return 0
        if len(moves) > MAX_MOVES:
            return 0


origin_execut = "/home/dario/Programming/OrchestraChess/target/release/rust-chess-bot"
move_heur_exec = "/home/dario/Programming/rust-chess-bot/target/release/rust-chess-bot"

# Start the Rust executable
vanilla_version = subprocess.Popen(
    [origin_execut],
    cwd="/home/dario/Programming/OrchestraChess",
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    universal_newlines=True  # Enables text mode
)

move_heur_version = subprocess.Popen(
    [move_heur_exec],
    cwd="/home/dario/Programming/rust-chess-bot",
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    universal_newlines=True  # Enables text mode
)

unplayed = 0
van_wins = 0
heu_wins = 0
draws = 0
n_van = "Vanilla"
n_heu = "MovHeur"
for game in range(100):
    if random.random() < 0.5:
        x = sim_game(vanilla_version, move_heur_version, n_van, n_heu)
    else:
        x = sim_game(move_heur_version, vanilla_version, n_heu, n_van)
        if x is not None: x = -x

    match x:
        case 1:
            van_wins += 1
        case -1:
            heu_wins += 1
        case 0:
            draws += 1
        case None:
            unplayed += 1

    print(f"{game=} {van_wins=} {heu_wins=} {draws=} {unplayed=} result={x}")

vanilla_version.terminate()
move_heur_version.terminate()

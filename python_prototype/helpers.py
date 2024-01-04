LOG_PATH = './pandora_box/logs.txt'


def log_to_file(message):
    with open(LOG_PATH, 'a') as f:
        f.write(message + '\n')


def respond_to_uci(message):
    print(message)
    log_to_file(message)


def split_fen_moves(s):
    if "moves" not in s:
        return s, ""
    s = s.split("moves")

    return s[0].replace('fen ', ''), s[1]

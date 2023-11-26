from tqdm import tqdm

ELO_THRESHOLD = 2300
TIME_CONTROL_THRESHOLD = 180

INPUT = "output.pgn"
OUTPUT = "output_high_elo.pgn"


with open(INPUT, 'r') as input_file:
    with open(OUTPUT, 'w') as output_file:
        low = True
        buffer = ""
        has_moves = False  # Flag to check if the game has moves
        for line in tqdm(input_file):
            if line.startswith('1.'):
                has_moves = True  # Set the flag if moves are encountered

            if "WhiteElo" in line:
                if has_moves:  # Only process games with moves
                    output_file.write(buffer)
                    has_moves = False
                try:
                    if int(line.split('"')[1]) > ELO_THRESHOLD:
                        low = False
                        buffer += line
                    else:
                        low = True
                        buffer = ""
                        has_moves = False  # Reset the flag for the next game
                except ValueError:
                    print("ValueError")
                    continue
            elif "BlackElo" in line:
                try:
                    if int(line.split('"')[1]) > ELO_THRESHOLD and low is False:
                        low = False
                        buffer += line
                    else:
                        low = True
                        buffer = ""
                        has_moves = False
                except ValueError:
                    print("ValueError")
                    continue
            elif "TimeControl" in line:
                if low is False:
                    try:
                        tc = line.split('"')[1]
                        tc = tc.split('+')[0]
                        tc = int(tc)
                        if tc >= TIME_CONTROL_THRESHOLD:
                            buffer += line
                        else:
                            low = True
                            buffer = ""
                            has_moves = False
                    except:
                        low = True
                        buffer = ""
                        has_moves = False
            elif low is False:
                buffer += line

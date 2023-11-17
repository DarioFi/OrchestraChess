from tqdm import tqdm

with open('output.pgn', 'r') as input_file:
    with open('output_high_elo.pgn', 'w') as output_file:
        low = True
        buffer = ""
        has_moves = False  # Flag to check if the game has moves
        for line in tqdm(input_file.readlines()):
            if line.startswith('1.'):
                has_moves = True  # Set the flag if moves are encountered

            if "WhiteElo" in line:
                if has_moves:  # Only process games with moves
                    output_file.write(buffer)
                    has_moves = False
                if int(line.split('"')[1]) > 2000:
                    low = False
                    buffer += line
                else:
                    low = True
                    buffer = ""
                    has_moves = False  # Reset the flag for the next game
            elif "BlackElo" in line:
                if int(line.split('"')[1]) > 2000 and low is False:
                    low = False
                    buffer += line
                else:
                    low = True
                    buffer = ""
                    has_moves = False
            elif "TimeControl" in line:
                if low is False:
                    try:
                        tc = line.split('"')[1]
                        tc = tc.split('+')[0]
                        tc = int(tc)
                        if tc > 180:
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

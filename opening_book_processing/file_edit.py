from tqdm import tqdm
# todo: pipeline this into a series of files

with open('output.pgn', 'w') as output_file:
    with open('lichess_db_standard_rated_2016-06.pgn', 'r') as file:
        for line in tqdm(file.readlines()):
            if "[" not in line:
                output_file.write(line)
            else:
                if "Elo" in line:
                    output_file.write(line)
                if "TimeControl" in line:
                    output_file.write(line)

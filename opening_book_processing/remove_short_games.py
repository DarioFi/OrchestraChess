import tqdm

with open('uci_moves.uci', 'r') as f:
    with open('short_uci.uci', 'w') as o:
        for line in tqdm.tqdm(f):
            x = line.split()
            if len(x) > 40:
                buf = " ".join(x[:30]) + "\n"
                o.write(buf)

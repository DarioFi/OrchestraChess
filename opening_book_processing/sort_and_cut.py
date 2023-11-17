data = {}

with open("short_uci.uci") as f:
    with open("sorted_uci.uci", "w") as o:
        lines = f.readlines()
        lines.sort()
        for line in lines:
            o.write(line)
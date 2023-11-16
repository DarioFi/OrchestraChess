with open("short_uci.uci") as f:
    with open("sorted_uci.uci", "w") as o:
        lines = f.readlines()
        lines = list(set(lines))
        lines.sort()
        for line in lines:
            o.write(line)
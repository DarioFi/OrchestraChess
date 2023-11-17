from dataclasses import dataclass
from typing import List, Optional
import json


@dataclass
class Node:
    Mov: Optional[str]
    Score: int
    Children: List['Node']


def build_tree(games: List[List[str]], move: str) -> Node:
    root = Node(move, len(games), [])
    if not games or len(games[0]) == 0:
        return root

    unique_first_moves = set(game[0] for game in games)
    for move in unique_first_moves:
        child_games = [game[1:] for game in games if game[0] == move]
        child_node = build_tree(child_games, move)
        root.Children.append(child_node)
    
    return root


def save_tree_to_json(tree: Node, file_path: str):
    with open(file_path, 'w') as json_file:
        json.dump(tree.__dict__, json_file, default=lambda o: o.__dict__)


# Load games from the file
file_name = "good_games.uci"
games = []
with open(file_name, "r") as file:
    for line in file:
        moves = line.strip().split()
        games.append(moves)
        
# Build the tree
root = build_tree(games, "a1a1")

save_path = "tree.json"
save_tree_to_json(root, save_path)

print("Done!")

# Orchestra Chess

Orchestra Chess is a chess engine written from scratch in Rust, that reached 2250+ Elo rating on Lichess in blitz (98th percentile of weekly active human players). It was developed by Mattia Scardecchia and Dario Filatrella as a project for a Software Engineering course at Bocconi University, in Fall 2023.

## Engine

Here we describe the main components of the engine. Code can be found in the `src` directory.

### Board representation

We adopt a piece centric representation through a set-wise bitboard approach: for every color, and every piece type, we maintain a 64-bit integer that, read in binary, has ones in correspondence of all squares occupied by a piece of that type and color and zeros elsewhere. 
This choice makes move generation very efficient, because it allows to exploit bitwise operations that are extremely efficient on modern hardware.

### Principal Variation Search  --> mention quiescence search

We employ a Principal Variation Search algorithm with alpha-beta pruning, embedded within an iterative deepening scheme. To maximize pruning frequency, at each node of the game tree we use various heuristics to explore the most promising moves first. Furthermore, once the desired depth is reached, we perform a quiescence search, which consists in exploring a few more steps restricting attention to forcing moves only. This makes the propagation of the static evaluation through min-maxing far more reliable, mitigating the horizon effect.

### Efficiently Updatable Neural Network

To provide a static evaluation of a position, we employ a neural network architecture called NNUE (Efficiently Updatable Neural Network) with HalfKAv2_hm feature set. The input is a sparse binary vector where each component is associated to a quadruple (piece type, piece color, piece square, own king square). This gets embedded to a latent space through a linear layer, and a shallow feed-forward network provides an evaluation (actually, we just described HalfKA. The input encoding of HalfKAv2_hm is slightly more involved, but the idea is the same).
By design, the embedded input can be maintained and updated incrementally during the tree traversal, which is very efficient since making/unmaking a move can flip at most three input bits.
Because of lack of resources, we used pretrained weights from the [Stockfish](https://github.com/official-stockfish/Stockfish) open-source project.

### Transposition Table and Zobrist hash

We use a hash map to store the evaluation of already visited positions during the search. This way, when a transposition happens, which is frequent at high depths, we can query the hash map instead of repeating the same computations.
This requires being able to efficiently hash the board state. We employ a 64-bit Zobrist hash function to achieve that: we associate to each triple (piece type, piece color, piece square) a random 64-bit integer, and we hash a position XOR-ing the corresponding integers, plus an additional random integer if it's black to move. This way, the hash can be efficiently updated incrementally as we traverse the game tree.

### Opening Book

To save time during the opening phase, we downloaded a database of 44M games from Lichess, filtered them based on players' ratings and time control, and built a tree rooted in the starting position in which a node's children are all the positions that have been reached after that node in the filtered database. The end result is a 213Mb tree that can be efficiently queried for the most played continuation up to move 15 (although move quality obviously deteriorates with depth). For simplicity, we ignored transpositions here.
The python scripts used to create the opening book can be found in the `opening_book_processing` directory.

## UCI Protocol

We implemented the common UCI protocol to be able to communicate with existing GUIs and with Lichess. Time management is achieved spawning a separate thread that updates a mutex when it thinks it's time to stop the search. The decison is made based on the remaining time and the duration of the search at the previous depth of the iterative deepening scheme.

## Sources --> chess programming wiki, sebastian league, lichess repo, stockfish, nnue paper

* [Chess Programming Wiki](https://www.chessprogramming.org/Main_Page)
* [Lichess](https://lichess.org)
* [lichess-bot repo](https://github.com/lichess-bot-devs/lichess-bot)
* [NNUE repo](https://github.com/official-stockfish/nnue-pytorch)
* [Stockfish repo](https://github.com/official-stockfish/Stockfish)
* [Sebastian League's Chess Coding Adventure](https://github.com/SebLague/Chess-Coding-Adventure)

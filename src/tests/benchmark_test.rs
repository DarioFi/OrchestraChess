use std::time::Instant;
use crate::board::Board;
use crate::orchestradirector;

#[cfg(test)]
mod perft_tests {
    use super::*;

    /// Start from the standard chess opening position.
    fn init_board() -> Board {
        let mut board = Board::empty_board();
        board.from_startpos();
        board
    }

    /// Just prints how many nodes `perft(6)` visits and how long it took.
    /// Marked #[ignore] so it won't slow down your normal `cargo test` runs.
    #[test]
    #[ignore]
    fn test_perft_speed() {
        let mut board = init_board();
        let now = Instant::now();
        let nodes = board.perft(6, 6, true);
        let elapsed_ms = now.elapsed().as_millis();
        println!("perft(6) -> {} nodes in {} ms", nodes, elapsed_ms);
    }

    /// Run a quick perft(5) on a handful of FENs, printing each count.
    #[test]
    fn test_per_fen_nodes() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r2qk2r/1p1nbppp/p2pbn2/3Np3/4P3/1N2BP2/PPPQ2PP/R3KB1R b KQkq - 4 10",
            "3r2k1/1p3ppp/p2P1b2/5R2/1PN1r3/6P1/P4B1P/6K1 w - - 1 30",
            "3r2k1/4Rppp/pp6/8/PP6/6P1/7P/6K1 w - - 1 37",
        ];

        let mut board = init_board();
        let mut orchestra = orchestradirector::new_orchestra_director();
        for fen in &fens {
            orchestra.handle_command("position", &format!("fen {}", fen));
            orchestra.eng.search(5, 0);
            println!("FEN: {}\n → {} nodes\n", fen, orchestra.eng.node_count);
        }
    }

    /// Run through the same FENs, summing up total node count and printing it.
    #[test]
    fn test_total_node_count() {
        let fens = [
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            "r2qk2r/1p1nbppp/p2pbn2/3Np3/4P3/1N2BP2/PPPQ2PP/R3KB1R b KQkq - 4 10",
            "3r2k1/1p3ppp/p2P1b2/5R2/1PN1r3/6P1/P4B1P/6K1 w - - 1 30",
            "3r2k1/4Rppp/pp6/8/PP6/6P1/7P/6K1 w - - 1 37",
        ];

        let mut orchestra = orchestradirector::new_orchestra_director();
        let mut total = 0u64;
        for fen in &fens {
            orchestra.handle_command("position", &format!("fen {}", fen));
            orchestra.eng.search(5, 0);
            total += orchestra.eng.node_count;
        }
        println!("Total nodes across all positions: {}", total);
    }
}

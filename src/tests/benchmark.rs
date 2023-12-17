use std::fmt::format;
use std::process::exit;
use crate::board::Board;
use crate::orchestradirector;

#[allow(dead_code)]
pub fn test_perft_speed() {
    println!("Starting perft speed test");
    let mut board = Board::empty_board();
    board.from_startpos();

    let now = std::time::Instant::now();
    let res = board.perft(6, 6, true);
    let elapsed = now.elapsed();
    let elapsed_ms = elapsed.as_millis();
    println!("{} nodes in {} ms", res, elapsed_ms);
}

#[allow(dead_code)]
pub fn test_n_nodes() {
    let fens = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r2qk2r/1p1nbppp/p2pbn2/3Np3/4P3/1N2BP2/PPPQ2PP/R3KB1R b KQkq - 4 10",
        "3r2k1/1p3ppp/p2P1b2/5R2/1PN1r3/6P1/P4B1P/6K1 w - - 1 30",
        "3r2k1/4Rppp/pp6/8/PP6/6P1/7P/6K1 w - - 1 37"
    ];
    let mut tot_nodes = 0;
    let mut orchestra_director = orchestradirector::new_orchestra_director();
    let mut result: String = "".to_owned();
    for fen in fens.iter() {
        orchestra_director.handle_command("position", &*("fen ".to_string() + fen));
        orchestra_director.eng.search(5, 0);
        result.push_str(&*format!("fen {}, nodes {}\n", fen, orchestra_director.eng.node_count));
        tot_nodes += orchestra_director.eng.node_count;
    }

    println!("{}", result);
    println!("Total nodes: {}", tot_nodes);
}

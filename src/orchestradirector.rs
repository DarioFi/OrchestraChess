use std::option::Option;
#[macro_export]
macro_rules! debug {
    () => {
        false
    };
}


use crate::board::{Board, empty_board, from_fen, from_startpos};


#[path = "helpers.rs"]
mod helpers;

#[path = "move.rs"]
mod r#move;

use crate::engine::{Engine, new_engine};

pub struct OrchestraDirector {
    pub eng: Engine,
}

pub fn new_orchestra_director() -> OrchestraDirector {
    OrchestraDirector {
        eng: Engine { board: empty_board() },
    }
}

impl OrchestraDirector {
    pub(crate) fn init_startpos(&mut self) {
        if debug!() {
            println!("startpos");
        }
        self.eng.board = from_startpos();
    }

    pub fn handle_command(&mut self, command: &str, options: &str) {
        match command {
            "uci" => self.uci_handle_uci(),
            "isready" => self.uci_handle_isready(),
            "ucinewgame" => {} // Do nothing for "ucinewgame"
            "position" => self.uci_handle_position(options),
            "go" => self.uci_handle_go(options),
            "stop" => self.uci_handle_stop(),
            "quit" => self.uci_handle_quit(),
            "setoption" => {} // Do nothing for "setoption"
            _ => {
                if debug!() {
                    panic!("NotImplementedError: {} {}", command, options);
                }
            }
        }
    }

    fn uci_handle_uci(&self) {
        println!("id name Orchestra");
        println!("id author Dario & Mattia");
        println!("uciok");
    }

    fn uci_handle_position(&mut self, options: &str) {
        if options.starts_with("startpos") {
            self.init_startpos();
            let w = options.split("moves").collect::<Vec<_>>();
            if w.len() > 1 {
                self.execute_moves(w[1]);
            }
        } else {
            let (fen, moves) = helpers::split_fen_moves(options); // Assuming helpers module is available
            self.init_from_fen(&fen);
            self.execute_moves(&moves);
        }
    }

    fn execute_moves(&mut self, param: &str) {
        let moves: Vec<&str> = param.split_whitespace().collect();
        for mov_str in moves {
            let mov = self.eng.board.move_from_str(mov_str);
            self.eng.board.make_move(mov);
        }
    }

    fn init_from_fen(&mut self, fen: &str) {
        // todo: review this because the string editing is done in two different places
        // fen = options[options.find("[") + 1:options.find("]")]
        self.eng.board = from_fen(fen);
    }

    fn uci_handle_isready(&self) {
        println!("readyok");
    }

    fn uci_handle_go(&mut self, options: &str) {
        let op_list: Vec<&str> = options.split_whitespace().collect();
        let mut i = 0;
        while i < op_list.len() {
            match op_list[i] {
                "wtime" | "btime" | "winc" | "binc" | "depth" | "nodes" => {
                    i += 2;
                }
                "infinite" => {
                    i += 1;
                }
                _ => {
                    i += 1;
                }
            }
        }


        let mov = self.eng.search(6);

        println!("bestmove {}", mov.to_string());
    }

    fn uci_handle_stop(&self) {
        panic!("NotImplementedError");
    }

    fn uci_handle_quit(&self) {
        std::process::exit(0);
    }
}

use std::sync::{Arc, Mutex};
use crate::helpers::respond_to_uci;
#[macro_export]
macro_rules! debug {
    () => {
        false
    };
}

use crate::timer::{start_timer, Timer};

use crate::board::Board;
use crate::book::OpeningBook;
use crate::constants::COLOR::{BLACK, WHITE};

#[path = "helpers.rs"]
mod helpers;

#[path = "move.rs"]
mod r#move;

use crate::engine::{new_engine, Engine};

pub struct OrchestraDirector {
    pub eng: Engine,
    timer: Timer,

}

pub fn new_orchestra_director() -> OrchestraDirector {
    OrchestraDirector {
        eng: new_engine(Board::empty_board()),
        timer: Timer::new_timer(),

    }
}

impl OrchestraDirector {
    pub(crate) fn init_startpos(&mut self) {
        self.eng.board = Board::from_startpos();
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
        respond_to_uci("id name Orchestra");
        respond_to_uci("id author Dario & Mattia");
        respond_to_uci("uciok");
    }

    fn uci_handle_position(&mut self, options: &str) {
        if options.starts_with("startpos") { // todo: review this because the string editing is done in two different places
            self.init_startpos();
            let w = options.split("moves").collect::<Vec<_>>();
            if w.len() > 1 {
                self.execute_moves(w[1]);
                self.eng.moves_loaded = w[1].to_string();
            }
            self.eng.position_loaded = "startpos".to_string();
        } else {
            let (fen, moves) = helpers::split_fen_moves(options);
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
        self.eng.board = Board::from_fen(fen);
    }

    fn uci_handle_isready(&self) {
        respond_to_uci("readyok");
    }

    fn uci_handle_go(&mut self, options: &str) {
        println!("{}", options);
        let hook = Arc::new(Mutex::new(false));

        let op_list: Vec<&str> = options.split_whitespace().collect();
        let mut i = 0;
        let mut movetime: u64 = 0;
        let mut depth: u64 = 0;
        while i < op_list.len() {
            match op_list[i] {
                "wtime" => {
                    if self.eng.board.color_to_move == WHITE {
                        self.timer.msec_left = op_list[i + 1].parse().unwrap();
                    }
                    i += 2;
                }
                "btime" => {
                    if self.eng.board.color_to_move == BLACK {
                        self.timer.msec_left = op_list[i + 1].parse().unwrap();
                    }
                    i += 2;
                }

                "winc" => {
                    if self.eng.board.color_to_move == WHITE {
                        self.timer.msec_inc = op_list[i + 1].parse().unwrap();
                    }
                    i += 2;
                }
                "binc" => {
                    if self.eng.board.color_to_move == BLACK {
                        self.timer.msec_inc = op_list[i + 1].parse().unwrap();
                    }
                    i += 2;
                }
                "depth" => {
                    depth = op_list[i + 1].parse().unwrap();
                    depth /= 2;
                    i += 2;
                }
                "nodes"
                => {
                    i += 2;
                }
                "infinite" => {
                    i += 1;
                }
                "movetime" => {
                    movetime = op_list[i + 1].parse().unwrap();
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            }
        }

        self.timer.move_time = movetime;

        if depth == 0 {
            start_timer(self.timer.clone(), hook.clone());
            depth = 20;
        };
        let res = self.eng.search(depth, hook.clone());
        let mov = res.1;
        let _score = res.0;

        println!("bestmove {}", mov.to_uci_string());
    }

    fn uci_handle_stop(&self) {
        panic!("NotImplementedError");
    }

    fn uci_handle_quit(&self) {
        std::process::exit(0);
    }
}

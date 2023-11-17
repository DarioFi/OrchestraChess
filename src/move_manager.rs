use crate::r#move::Move;

pub struct MoveManager{

}

impl MoveManager{

    pub fn new() -> MoveManager{
        MoveManager{}
    }

    pub fn get_move(&self, move_str: &str) -> Move{
        let mut move_str = move_str.to_string();
        move_str.make_ascii_lowercase();
        match move_str.as_str(){
            "rock" => Move::Rock,
            "paper" => Move::Paper,
            "scissors" => Move::Scissors,
            "lizard" => Move::Lizard,
            "spock" => Move::Spock,
            _ => Move::Invalid
        }
    }

}
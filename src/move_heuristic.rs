use crate::muve::Move;

pub struct MovesHeuristic {
    killers: Vec<(Option<Move>, Option<Move>)>,
}

const INIT_LEN: usize = 100;

impl MovesHeuristic {
    pub fn new() -> MovesHeuristic {
        MovesHeuristic { killers: Vec::with_capacity(INIT_LEN) }
    }

    pub(crate) fn get_killers(&self, depth: usize) -> Vec<Move> {
        if self.killers.len() > depth {
            let (k1, k2) = self.killers[depth];
            let mut res = vec![];
            if k1.is_some() {
                res.push(k1.unwrap());
            }
            if k2.is_some() {
                res.push(k2.unwrap());
            }
            res
        } else {
            vec![]
        }
    }

    pub(crate) fn failed_high(&mut self, depth: usize, m: Move, prev_m: Option<&Move>) {
        if self.killers.len() <= depth {
            self.killers.resize(depth + 1, (None, None));
        }
        let (k1, k2) = self.killers[depth];
        self.killers[depth] = (Some(m), k1);
    }
}


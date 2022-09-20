use crate::{prob_dist::ProbDist, RollOut, Rollable};

#[derive(Default, Clone, Debug)]
pub struct Nothing {}

impl Nothing {
    pub fn new() -> Self {
        Self {}
    }
}

impl Rollable for Nothing {
    fn roll(&self) -> RollOut {
        RollOut::default()
    }

    fn roll_quiet(&self) -> isize {
        0
    }

    fn dist(&self) -> ProbDist {
        ProbDist::default()
    }
}

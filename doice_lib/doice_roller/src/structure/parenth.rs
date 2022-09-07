use std::str::FromStr;

use crate::{prob_dist::ProbDist, DiceError, Expression, RollOut, Rollable};

use super::lin_comb::LinComb;

#[derive(Clone)]
pub struct Parenth {
    expr: Expression,
}

impl FromStr for Parenth {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let contents = &src[1..(src.len() - 1)];
        Ok(Parenth {
            expr: LinComb::from_str(contents)?.into(),
        })
    }
}

impl Rollable for Parenth {
    fn roll(&self) -> RollOut {
        let mut out = self.expr.roll();
        out.txt.append(")");
        out.txt.append_front("(");
        out
    }

    fn roll_quiet(&self) -> isize {
        self.expr.roll_quiet()
    }

    fn dist(&self) -> ProbDist {
        self.expr.dist()
    }
}

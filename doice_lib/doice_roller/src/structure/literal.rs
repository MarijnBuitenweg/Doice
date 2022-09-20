use std::{collections::BTreeMap, str::FromStr};

use crate::{layouter::Layouter, prob_dist::ProbDist, DiceError, RollOut, Rollable};

#[derive(Default, Clone, Debug)]
pub struct Literal {
    value: isize,
}

impl From<isize> for Literal {
    fn from(value: isize) -> Self {
        Self { value }
    }
}

impl Rollable for Literal {
    fn roll(&self) -> RollOut {
        let mut out = Layouter::default();
        out.append(&self.value.to_string());
        RollOut {
            value: self.value,
            txt: out,
        }
    }

    fn roll_quiet(&self) -> isize {
        self.value
    }

    fn dist(&self) -> ProbDist {
        ProbDist::try_from(BTreeMap::from([(self.value, 1.0f64)])).expect("Literally Terrible!")
    }
}

impl FromStr for Literal {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Literal {
            value: src.parse().or(Err("Bad Literal"))?,
        })
    }
}

#![feature(map_first_last)]

use std::{fmt::Debug, ops::Add, str::FromStr};

use dyn_clone::DynClone;

use self::structure::{lin_comb::LinComb, nop::Nothing};

mod dice_adapters;
mod functions;
mod prob_dist;
mod sample_dist;
mod structure;
pub use prob_dist::ProbDist;
pub use sample_dist::SampleDist;
use structure::expression::Expression;

mod layouter;
pub use layouter::Layouter;
mod bruteforce;
use bruteforce::BruteForceProbDist;
mod dice_roller;
pub use dice_roller::DiceRoller;
mod error;
pub use error::DiceError;
pub mod legacy;

pub type Value = isize;

#[derive(Default, Clone)]
pub struct RollOut {
    pub value: Value,
    pub txt: Layouter,
}

impl Add<RollOut> for RollOut {
    type Output = Self;

    fn add(mut self, rhs: RollOut) -> Self::Output {
        self.txt.append(" ");
        self.txt = self.txt + rhs.txt;
        self.value += rhs.value;
        self
    }
}

pub trait Rollable: DynClone + Send + Sync + Debug {
    fn roll(&self) -> RollOut;
    fn dist(&self) -> ProbDist;

    fn roll_quiet(&self) -> isize {
        self.roll().value
    }
}

/// Wrapper around Expression that exposes the public API
#[derive(Clone, Debug)]
pub struct Roll {
    root: Expression,
}

impl Roll {
    pub fn from_expr(expr: Expression) -> Self {
        Roll { root: expr }
    }
}

impl Add<isize> for Roll {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let expr = self.root;
        let mut new_root = LinComb::from(expr);
        new_root.add_term(rhs.into());
        Roll {
            root: new_root.into(),
        }
    }
}

impl FromStr for Roll {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Roll {
            root: LinComb::from_str(src)?.into(),
        })
    }
}

impl TryFrom<&str> for Roll {
    type Error = DiceError;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Self::from_str(src)
    }
}

impl Rollable for Roll {
    fn roll(&self) -> RollOut {
        self.root.roll()
    }

    fn dist(&self) -> ProbDist {
        let mut dist = self.root.dist();
        dist.remove_null();
        dist
    }
}

impl Default for Roll {
    fn default() -> Self {
        Roll {
            root: Nothing::new().into(),
        }
    }
}

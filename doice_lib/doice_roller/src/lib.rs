#![feature(map_first_last)]
#![feature(iter_intersperse)]

use std::{convert::Infallible, error::Error, fmt::Display, ops::Add, str::FromStr};

use dyn_clone::DynClone;

use self::structure::{lin_comb::LinComb, nop::Nothing};

mod dice_adapters;
mod functions;
mod prob_dist;
mod sample_dist;
mod structure;
pub use prob_dist::ProbDist;
pub use sample_dist::SampleDist;

mod layouter;
pub use layouter::Layouter;
mod bruteforce;
pub use bruteforce::BruteForceProbDist;
mod dice_roller;
pub use dice_roller::DiceRoller;
pub mod legacy;

type Expression = Box<dyn Rollable + Send + Sync>;

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

pub trait Rollable: DynClone + Send {
    fn roll(&self) -> RollOut;
    fn dist(&self) -> ProbDist;

    fn roll_quiet(&self) -> isize {
        self.roll().value
    }
}

#[derive(Clone)]
pub struct Roll {
    src_text: String,
    root: Expression,
}

impl Roll {
    pub fn from_expr(expr: Expression, display_txt: String) -> Self {
        Roll {
            src_text: display_txt,
            root: expr,
        }
    }

    pub fn src(&self) -> &str {
        &self.src_text
    }
}

impl Add<isize> for Roll {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let (expr, src_text) = (self.root, self.src_text);
        let mut new_root = LinComb::from(expr);
        new_root.add_term(rhs.into());
        Roll {
            src_text,
            root: new_root.into(),
        }
    }
}

impl FromStr for Roll {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Roll {
            src_text: String::from(src),
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
            src_text: String::new(),
            root: Nothing::new().into(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DiceError {
    desc: String,
}

impl From<String> for DiceError {
    fn from(str: String) -> Self {
        DiceError { desc: str }
    }
}

impl From<DiceError> for String {
    fn from(err: DiceError) -> Self {
        err.desc
    }
}

impl From<&str> for DiceError {
    fn from(input: &str) -> Self {
        input.to_string().into()
    }
}

impl Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl Error for DiceError {}

impl From<Infallible> for DiceError {
    fn from(_: Infallible) -> Self {
        Default::default()
    }
}

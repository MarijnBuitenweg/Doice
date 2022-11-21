use dyn_clone::DynClone;
use std::{fmt::Debug, ops::Add, str::FromStr};

/// Contains the logic for functional expressions, and the definitions of all functions with corresponding docs
mod functions;
pub use functions::FUNCTION_DOCS;
/// Defines the `ProbDist` type
mod prob_dist;
pub use prob_dist::ProbDist;
/// Defines the `SampleDist` type
mod sample_dist;
pub use sample_dist::SampleDist;
/// Defines types representing structures present in expressions, such as linear combinations or parentheses
mod structure;
use structure::{expression::Expression, lin_comb::LinComb, nop::Nothing};
/// Defines the `Layouter` type
mod layouter;
pub use layouter::Layouter;
/// Contains the logic enabling the bruteforcing of probability distributions of rollable things
mod bruteforce;
use bruteforce::BruteForceProbDist;
/// Contains the logic for rolling dice
mod dice_roller;
pub use dice_roller::DiceRoller;
/// Defines the `DiceError` type
mod dice_error;
pub use dice_error::DiceError;
/// Contains and exposes old stuff, may be removed later
pub mod legacy;
mod utils;

#[cfg(test)]
mod test;

/// The type that is to be used as the numeric result of a roll
pub type Value = isize;

/// Struct containing the output of a roll, in both text and numeric formats
#[derive(Default, Clone)]
pub struct RollOut {
    pub value: Value,
    pub txt: Layouter,
}

impl Add<RollOut> for RollOut {
    type Output = Self;

    /// Adds the numeric, and appends the text fields
    fn add(mut self, rhs: RollOut) -> Self::Output {
        self.txt.append(" ");
        self.txt += rhs.txt;
        self.value += rhs.value;
        self
    }
}

/// Trait generalizing over anything that can be rolled
pub trait Rollable: DynClone + Send + Sync + Debug {
    /// Roll once, producing full output
    fn roll(&self) -> RollOut;
    /// Calculate the probability distribution of the rollable
    fn dist(&self) -> ProbDist;

    /// Roll once, producing only numeric output.
    /// Calls `Rollable::roll` by default, optimizing this implementation is optional but recommended
    fn roll_quiet(&self) -> Value {
        self.roll().value
    }
}

/// Wrapper around Expression that exposes the public API
#[derive(Clone, Debug)]
pub struct Roll {
    /// The base of the tree
    root: Expression,
}

impl Roll {
    /// Instantiates a roll wrapping the provided expression
    #[must_use]
    pub fn from_expr(expr: Expression) -> Self {
        Roll { root: expr }
    }
}

impl Add<isize> for Roll {
    type Output = Self;

    /// Adds a simple integer to the roll
    /// This operation is not nearly as efficient as most other additions, but o well
    fn add(self, rhs: Value) -> Self::Output {
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

    /// Attempts to parse the str into an expression
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        Ok(Roll {
            root: dbg!(LinComb::from_str(src)?.into()),
        })
    }
}

impl TryFrom<&str> for Roll {
    type Error = DiceError;

    /// Delegates to `FromStr::from_str`
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        Self::from_str(src)
    }
}

impl Rollable for Roll {
    /// Rolls the expression
    fn roll(&self) -> RollOut {
        self.root.roll()
    }

    /// Obtains and sanitizes the probability distribution of the expression
    fn dist(&self) -> ProbDist {
        let mut dist = self.root.dist();
        dist.remove_null();
        dist
    }

    /// Rolls the expression, quietly
    fn roll_quiet(&self) -> isize {
        self.root.roll_quiet()
    }
}

impl Default for Roll {
    /// Instantiates an empty roll that always returns 0
    fn default() -> Self {
        Roll {
            root: Nothing::new().into(),
        }
    }
}

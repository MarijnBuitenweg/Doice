use std::str::FromStr;

use crate::{
    bruteforce::BruteForceProbDist,
    structure::{expression::Expression, lin_comb::LinComb},
    utils::split_once_parenth,
    DiceError, Layouter, ProbDist, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Clone, Debug)]
pub struct AddNonZero {
    base: LinComb,
    added: LinComb,
}

impl FunctionInit for AddNonZero {
    const DOC: &'static str = "Adds the second expression to all non-zero outcomes of the first.\nUsage: addnz(base expr, added expr)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (base_src, added_src) =
            split_once_parenth(input, ',').ok_or("addnz: invalid arguments")?;
        Ok(Self {
            base: LinComb::from_str(base_src)?,
            added: LinComb::from_str(added_src)?,
        }
        .into())
    }
}

impl Rollable for AddNonZero {
    fn roll(&self) -> RollOut {
        let mut base_roll = self.base.roll();
        base_roll.txt = Layouter::from("[") + base_roll.txt;
        if base_roll.value != 0 {
            base_roll.txt += " +";
            base_roll += self.added.roll();
        }
        base_roll.txt += "]";
        base_roll
    }

    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}

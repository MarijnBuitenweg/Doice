use crate::{
    BruteForceProbDist, DiceError, DiceRoller, Expression, Layouter, ProbDist, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Default, Clone, Debug)]
pub struct Crit {
    roller: DiceRoller,
    avg_roll: f64,
}

impl FunctionInit for Crit {
    const DOC: &'static str = "Rolls the provided dmg dice like a critted attack roll, choosing to eiter double the initial roll or roll twice the amount of dice.\nUsage: crit(roll)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let roll: DiceRoller = input.parse()?;
        let avg_roll = roll.dist().expectation();
        Ok(Crit {
            roller: roll,
            avg_roll,
        }
        .into())
    }
}

/// Implements the `Rollable` trait for the `Crit` struct.
impl Rollable for Crit {
    /// Rolls the dice and returns the result as a `RollOut` struct.
    fn roll(&self) -> RollOut {
        let mut txt_out = Layouter::new();
        txt_out.append("[");
        let init_roll = self.roller.roll();
        txt_out += init_roll.txt;
        let val_out = if init_roll.value > self.avg_roll as isize {
            txt_out.append("*2]");
            init_roll.value * 2
        } else {
            let second_roll = self.roller.roll();
            txt_out += second_roll.txt;
            init_roll.value + second_roll.value
        };
        txt_out.append("]");

        RollOut {
            value: val_out,
            txt: txt_out,
        }
    }

    /// Calculates the probability distribution of the `Crit` struct.
    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}

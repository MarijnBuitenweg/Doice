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

impl Rollable for Crit {
    fn roll(&self) -> RollOut {
        let mut txt_out = Layouter::new();
        txt_out.append("[");
        let init_roll = self.roller.roll();
        txt_out = txt_out + init_roll.txt;
        let val_out = if init_roll.value > self.avg_roll as isize {
            txt_out.append("*2]");
            init_roll.value * 2
        } else {
            let second_roll = self.roller.roll();
            txt_out = txt_out + second_roll.txt;
            init_roll.value + second_roll.value
        };
        txt_out.append("]");

        RollOut {
            value: val_out,
            txt: txt_out,
        }
    }

    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}

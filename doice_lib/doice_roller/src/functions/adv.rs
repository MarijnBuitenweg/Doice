use crate::{DiceError, Expression, ProbDist, RollOut, Rollable};

use super::FunctionInit;

#[derive(Clone, Default, Debug)]
pub struct Adv {
    contents: Expression,
}

impl FunctionInit for Adv {
    fn generate(input: &str) -> Result<Expression, DiceError> {
        let contents = input.parse()?;
        Ok(Adv { contents }.into())
    }
}

impl Rollable for Adv {
    fn roll(&self) -> RollOut {
        let rolls = [self.contents.roll(), self.contents.roll()];
        let mut out = RollOut {
            value: rolls[0].value.max(rolls[1].value),
            ..Default::default()
        };

        out.txt.append("[");
        if out.value == rolls[0].value {
            out.txt.append(&format!("{} ", rolls[0].value));
            out.txt.append_strikethrough(&rolls[1].value.to_string());
        } else {
            out.txt.append_strikethrough(&rolls[0].value.to_string());
            out.txt.append(&format!(" {}", rolls[1].value));
        }
        out.txt.append("]");

        out
    }

    fn dist(&self) -> ProbDist {
        let mut dist = self.contents.dist();
        dist.apply_advantage(1);
        dist
    }
}

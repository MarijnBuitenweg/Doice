use std::str::FromStr;

use crate::{
    structure::{expression::Expression, lin_comb::LinComb},
    Layouter, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Debug, Clone)]
pub struct Mirror {
    expr: Expression,
    mirror: LinComb,
}

impl FunctionInit for Mirror {
    const DOC: &'static str = "Mirrors the input expression's probability distribution to the other side of the y-axis.\nUsage: mirror(expr)";

    fn generate(input: &str) -> Result<Expression, crate::DiceError> {
        Ok(Box::new(Self {
            expr: LinComb::from_str(input)?.into(),
            mirror: LinComb::from_str("(2*d2-3)").unwrap(),
        }))
    }
}

impl Rollable for Mirror {
    fn roll(&self) -> RollOut {
        let mut txt = Layouter::from("((");
        let expr_out = self.expr.roll();
        txt += expr_out.txt;
        txt.append(")*[");
        let sign = self.mirror.roll_quiet();
        txt.append(&(sign.to_string() + "])"));

        RollOut {
            value: sign * expr_out.value,
            txt,
        }
    }

    fn dist(&self) -> crate::ProbDist {
        self.expr.dist() * &self.mirror.dist()
    }
}

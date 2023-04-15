use std::str::FromStr;

use itertools::Itertools;

use crate::{
    structure::lin_comb::LinComb, utils::split_once_parenth, DiceError, Expression, Layouter,
    ProbDist, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Clone, Default, Debug)]
pub struct Sum {
    expr: Expression,
    n: Expression,
}

impl FunctionInit for Sum {
    const DOC: &'static str =
        "Rolls the provided expression n times and takes the sum.\nUsage: sum(expr, n)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (expr, n) = split_once_parenth(input, ',').ok_or("invalid args passed to sum")?;
        let n: Expression = LinComb::from_str(n)?.into();
        if n.dist().is_empty() {
            return Err("sum must be provided with a non-empty n".into());
        }

        Ok(Sum {
            expr: LinComb::from_str(expr)?.into(),
            n,
        }
        .into())
    }
}

impl Rollable for Sum {
    fn roll(&self) -> RollOut {
        let mut space = Layouter::default();
        space.append(" ");

        let mut out = RollOut::default();
        out.txt.append("[");

        let n = self.n.roll_quiet();

        out = Itertools::intersperse_with((0..n).map(|_| self.expr.roll()), || RollOut {
            value: 0,
            txt: space.clone(),
        })
        .fold(out, |acc, elem| acc + elem);

        out.txt.append("]");
        out
    }

    fn dist(&self) -> ProbDist {
        self.expr
            .dist()
            .rep_auto_convolution(&self.n.dist())
            .unwrap_or_default()
    }
}

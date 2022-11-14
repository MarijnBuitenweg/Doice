use std::str::FromStr;

use itertools::Itertools;

use crate::{
    structure::lin_comb::LinComb, DiceError, Expression, Layouter, ProbDist, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Clone, Default, Debug)]
pub struct Sum {
    expr: Expression,
    n: usize,
}

impl FunctionInit for Sum {
    const DOC: &'static str =
        "Rolls the provided expression n times and takes the sum.\nUsage: sum(expr, n)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (expr, n) = input
            .split(',')
            .collect_tuple()
            .ok_or("invalid args passed to sum")?;

        Ok(Sum {
            expr: LinComb::from_str(expr)?.into(),
            n: n.parse()
                .map_err(|_| "number of times to sum could not be parsed")?,
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

        out = Itertools::intersperse_with((0..self.n).map(|_| self.expr.roll()), || RollOut {
            value: 0,
            txt: space.clone(),
        })
        .fold(out, |acc, elem| acc + elem);

        out.txt.append("]");
        out
    }

    fn dist(&self) -> ProbDist {
        self.expr.dist() * self.n
    }
}

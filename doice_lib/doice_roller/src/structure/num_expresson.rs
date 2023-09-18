use std::str::FromStr;
use crate::{DiceError, ProbDist, Rollable, RollOut};
use crate::structure::expression::Expression;
use crate::structure::lin_comb::LinComb;

#[derive(Debug, Clone)]
pub enum NumericExpression {
    Constant(f64),
    Stochastic(Expression),
}

impl Default for NumericExpression {
    fn default() -> Self {
        NumericExpression::Constant(0.0)
    }
}

impl NumericExpression {
    pub fn evaluate(&self) -> f64 {
        match self {
            NumericExpression::Constant(val) => *val,
            NumericExpression::Stochastic(exp) => exp.roll_quiet() as f64,
        }
    }
}

impl FromStr for NumericExpression {
    type Err = DiceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if let Ok(num) = s.parse() {
            Ok(NumericExpression::Constant(num))
        } else {
            Ok(NumericExpression::Stochastic(LinComb::from_str(s)?.into()))
        }
    }
}
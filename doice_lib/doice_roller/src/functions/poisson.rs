use std::collections::BTreeMap;

use std::ops::Mul;
use crate::functions::FunctionInit;
use crate::{DiceError, ProbDist, Rollable, RollOut};
use crate::structure::expression::Expression;

use crate::structure::num_expresson::NumericExpression;

fn factorial(n: usize) -> f64 {
    if n == 0 {
        1.0
    } else {
        (1..=n).map(|i| i as f64).fold(n as f64, f64::mul)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Poisson {
    avg_events: NumericExpression,
}

impl Poisson {
    fn calc_prob(num_events: usize, avg_events: f64) -> f64 {
        let lambda = avg_events;
        let k = num_events;
        (lambda.powi(k as i32)*(-lambda).exp()) / factorial(k)
    }

    fn recalc_dist(&self, avg_events: f64) -> ProbDist {
        // TODO: Make this work for more than 20 events.
        let mut dist = BTreeMap::<isize, f64>::new();
        for i in 0..20 {
            dist.insert(i, dbg!(Self::calc_prob(i as usize, avg_events)));
        }
        dist.try_into().expect("Bad math in recalc_dist")
    }
}

impl Rollable for Poisson {
    fn roll(&self) -> RollOut {
        self.recalc_dist(self.avg_events.evaluate()).roll()
    }

    fn dist(&self) -> ProbDist {
        match &self.avg_events {
            NumericExpression::Constant(lambda) => {self.recalc_dist(*lambda)}
            NumericExpression::Stochastic(expr) => {
                let mut accumulator = BTreeMap::<isize, f64>::new();
                for (lambda, scale) in expr.dist().inner() {
                    let dist = self.recalc_dist(lambda as f64);
                    for (outcome, prob) in dist.inner() {
                        accumulator.entry(outcome).and_modify(|p| *p += prob*scale).or_insert(prob*scale);
                    }
                }
                accumulator.try_into().expect("Mathematical error in the Poisson dist.")
            }
        }
    }
}

impl FunctionInit for Poisson {
    const DOC: &'static str = "Poisson distribution for a given average number of events.";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        Ok(Poisson {
            avg_events: input.parse()?,
        }.into())
    }
}
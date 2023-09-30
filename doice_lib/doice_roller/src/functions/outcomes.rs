use crate::functions::FunctionInit;
use crate::{DiceError, ProbDist, Rollable, RollOut, SampleDist, Value};
use crate::structure::expression::Expression;

#[derive(Clone, Debug)]
pub struct Outcomes {
    prob_dist: ProbDist,
}

impl Rollable for Outcomes {
    fn roll(&self) -> RollOut {
        self.prob_dist.roll()
    }

    fn dist(&self) -> ProbDist {
        self.prob_dist.clone()
    }
}

impl FunctionInit for Outcomes {
    const DOC: &'static str = "Random process with the specified set of equally likely outcomes.\nUsage: outcomes(outcome1, outcome2, ...)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let mut samples = SampleDist::default();
        samples.remove_samples(0);
        for outcome in input.split(',') {
            samples.add_sample(outcome.trim().parse::<Value>().or(Err("Invalid outcome."))?);
        }

        Ok(Outcomes {
            prob_dist: samples.into(),
        }.into())
    }
}


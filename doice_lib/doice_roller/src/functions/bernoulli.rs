use std::collections::BTreeMap;

use rand::{distributions::Uniform, thread_rng, Rng};

use crate::{ProbDist, RollOut, Rollable};

use super::FunctionInit;

/// Function representing a possibly biased coin toss
#[derive(Clone, Copy, Debug)]
pub struct Bernoulli {
    /// Probability of an outcome of 1
    p: f64,
}

impl Default for Bernoulli {
    fn default() -> Self {
        Self { p: 0.5 }
    }
}

impl FunctionInit for Bernoulli {
    const DOC: &'static str = "Simulates a possibly biased cointoss AKA Bernoulli trial with a provided or default probability of success.\nUsage: ber(p=0.5)";

    fn generate(input: &str) -> Result<crate::structure::expression::Expression, crate::DiceError> {
        let input = input.trim();
        // If valid probability has been provided, use it
        if let Ok(p) = input.parse() {
            Ok(Self { p }.into())
        // Otherwise use default
        } else {
            Ok(Self::default().into())
        }
    }
}

impl Rollable for Bernoulli {
    fn roll(&self) -> RollOut {
        let dist = Uniform::new(0.0, 1.0);
        let mut rng = thread_rng();
        // If success, return 1
        if self.p > rng.sample(dist) {
            RollOut {
                value: 1,
                txt: "[1]".into(),
            }
        // Otherwise, return 0
        } else {
            RollOut {
                value: 0,
                txt: "[0]".into(),
            }
        }
    }

    /// May panic if p is not within 0.0..=1.0
    fn dist(&self) -> crate::ProbDist {
        ProbDist::try_from(BTreeMap::from([(0, 1.0 - self.p), (1, self.p)])).unwrap()
    }

    fn roll_quiet(&self) -> crate::Value {
        let dist = Uniform::new(0.0, 1.0);
        let mut rng = thread_rng();
        // If success, return 1, otherwise return 0
        (self.p > rng.sample(dist)).into()
    }
}

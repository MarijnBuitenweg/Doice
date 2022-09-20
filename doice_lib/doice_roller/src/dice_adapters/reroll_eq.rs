use std::collections::BTreeMap;

use crate::{
    Layouter, ProbDist,
    {
        dice_roller::{DiceAdapter, DiceRoller},
        Value,
    },
};

use super::DiceAdapterGen;

#[derive(Clone, Debug, Default)]
pub struct RerollEq {
    reroll_value: usize,
}

impl DiceAdapter for RerollEq {
    fn run(
        &self,
        mut input: Vec<Value>,
        _out_txt: &mut Layouter,
        d_info: &DiceRoller,
    ) -> Vec<Value> {
        for reroll in input
            .iter_mut()
            .filter(|roll| **roll == self.reroll_value as isize)
        {
            *reroll = d_info.roll_quiet();
        }

        input
    }

    fn dist(&self, input: ProbDist) -> ProbDist {
        let r_val = self.reroll_value as isize;
        // If the reroll value cannot be rolled, do nothing
        if !input.contains_key(&r_val) {
            return input;
        }
        // Extract the map from the distribution so we can do some more mutation
        let mut input: BTreeMap<_, _> = input.into();
        let r_chance = input[&r_val];
        // First change the probability the reroll value
        input.entry(r_val).and_modify(|v| *v *= r_chance);
        // Then change the rest
        for (_, prob) in input.iter_mut().filter(|(&k, _)| k != r_val) {
            *prob += *prob * r_chance;
        }
        // Try to convert the map back into a valid probability distribution
        input
            .try_into()
            .expect("Computing rerollEq distribution resulted in a bad distribution!")
    }
}

impl From<usize> for RerollEq {
    fn from(value: usize) -> Self {
        RerollEq {
            reroll_value: value,
        }
    }
}

impl DiceAdapterGen for RerollEq {
    const IDENT: &'static str = "r";
}

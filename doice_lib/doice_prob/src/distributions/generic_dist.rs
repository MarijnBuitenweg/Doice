use std::{collections::BTreeMap, iter::Sum, ops::Div};

use num::One;

use crate::{chance::Chance, prelude::*};

use super::SampleDist;

/// Can represent any arbitrary dicrete probability distribution.
pub struct ProbDist<T>(BTreeMap<isize, Chance<T>>);

impl<T: BasicNum + Clone> ProbDist<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn validate(&self) -> Result<(), ChanceError> {
        // Check every chance
        self.0.values().try_for_each(Chance::validate)?;
        // Check total chance
        let total: Chance<T> = self.0.values().cloned().sum();
        if !total.is_one() {
            return Err(ChanceError::Overflow);
        }
        Ok(())
    }

    /// Returns the smallest possible outcome
    #[must_use]
    pub fn min(&self) -> Option<isize> {
        self.0.keys().copied().next()
    }

    /// Returns the highest possible outcome
    #[must_use]
    pub fn max(&self) -> Option<isize> {
        self.0.keys().copied().next_back()
    }

    /// Removes all probabilities that are equal to 0
    pub fn remove_null(&mut self) {
        self.0.retain(|_, prob| *prob != Chance::zero());
    }

    #[must_use]
    pub fn get_rev_cumulative_prob(&self) -> BTreeMap<isize, T> {
        let mut total = T::zero();
        let mut out = BTreeMap::new();
        // Iter over a BTreeMap is always sorted, yay
        for (outcome, prob) in self.0.iter().rev() {
            total = total + prob.inner().clone();
            out.insert(*outcome, total.clone());
        }

        out
    }

    /// Scale the probabilities such that the total probability is 1
    pub fn proper_scale(&mut self) {
        let total_prob: Chance<T> = self.0.values().cloned().sum();
        if total_prob != Chance::one() {
            let scale_factor = Chance::one() / total_prob;
            for v in self.0.values_mut() {
                *v *= scale_factor.clone();
            }
        }
    }

    pub fn read_samples(&mut self, samples: &SampleDist) {
        let total: usize = samples.values().sum();
        // First, read all the sample counts
        for (&outcome, &sample) in samples.iter() {
            let chance = match Chance::new(match T::from_f64(sample as f64 / total as f64) {
                Some(num) => num,
                None => continue,
            }) {
                Ok(val) => val,
                Err(_) => continue,
            };

            self.0.insert(outcome, chance);
        }
        // Then rescale it
        self.proper_scale();
    }

    pub fn apply_advantage(&mut self, adv: isize) {
        // Don't do anything if we're empty
        if self.len() == 0 || adv == 0 {
            return;
        }

        if adv > 0 {
            self.apply_positive_advantage(adv as usize);
        } else {
            self.apply_disadvantage(adv.unsigned_abs());
        }
    }

    #[must_use]
    pub fn get_cumulative_prob(&self) -> BTreeMap<isize, Chance<T>> {
        let mut total = 0.0;
        let mut out = BTreeMap::new();
        // Iter over a BTreeMap is always sorted, yay
        for (outcome, prob) in &self.0 {
            total += prob;
            out.insert(*outcome, total);
        }

        out
    }

    fn apply_disadvantage(&mut self, disadv: usize) {
        for _ in 0..disadv {
            // Initialization
            let cumul_dist = self.get_cumulative_prob(); // = P(X <= x)
            let mut cumulative = cumul_dist.values();
            let mut entries = self.0.values_mut();

            // Computation
            let mut total_below = 0.0;
            let first = entries.next().unwrap();
            // Pd(X = min) = 1 - P(X >= min + 1)^2
            *first = 1.0 - (1.0 - cumulative.next().unwrap()).powi(2);
            total_below += *first;

            for (val, cumul) in entries.zip(cumulative) {
                // P(X > x) = 1 - P(X <= x)
                // Pd(X = x>min) = 1 - P(X >= x+1)^2 - Pd(X < x)
                *val = 1.0 - (1.0 - cumul).powi(2) - total_below;
                total_below += *val;
            }
        }
    }

    fn apply_positive_advantage(&mut self, adv: usize) {
        for _ in 0..adv {
            let mut total_above = 0.0;
            let mut next = BTreeMap::new();
            let cumulative = self.get_cumulative_prob();
            let mut rev_iter = self.keys().rev();
            let first = rev_iter.next().unwrap();
            // The cumulative probabilities, in descending order, with the first one skipped and a zero appended to the end
            // Because we always need the cumulative probability of the X <= x - 1
            let mut rev_cumul = cumulative.values().rev().skip(1).chain([0.0].iter());
            // Pa(X = max) = 1 - P(X <= max - 1)^2
            total_above += 1.0 - rev_cumul.next().unwrap().powi(2);
            next.insert(*first, total_above);

            // Now for the rest of them
            for (outcome, cumul_below) in rev_iter.zip(rev_cumul) {
                // Pa(X=x<max) = 1 - P(X <= x - 1)^2 - Pa(X > x)
                let new_prob = 1.0 - cumul_below.powi(2) - total_above;
                // Add new probability to total
                total_above += new_prob;
                // Insert new entry
                next.insert(*outcome, new_prob);
            }

            // Next should now contain the probability weight function with advantage applied
            *self = ProbDist(next);
        }
    }
}

impl<T: ToPrimitive> ProbDist<T> {
    pub fn as_f64(&self) -> ProbDist<f64> {
        ProbDist(
            self.0
                .iter()
                .map(|(k, v)| (*k, v.to_f64_chance()))
                .collect(),
        )
    }

    /// The nth moment of the distribution AKA E(X^n).
    /// #[must_use]
    pub fn moment(&self, n: u32) -> f64 {
        self.0
            .iter()
            .map(|(outcome, prob)| {
                outcome.pow(n) as f64
                    * prob
                        .to_f64()
                        .expect("probability must be representable by f64")
            })
            .sum()
    }

    /// Variance of the distribution, equal to sigma^2
    #[must_use]
    pub fn var(&self) -> f64 {
        self.moment(2) - self.moment(1).powi(2)
    }

    /// The deviation of the distribution, equal to sqrt(var)
    #[must_use]
    pub fn sigma(&self) -> f64 {
        self.var().sqrt()
    }
}

impl<T: BasicNum + Clone> TryFrom<BTreeMap<isize, Chance<T>>> for ProbDist<T> {
    type Error = ChanceError;

    fn try_from(value: BTreeMap<isize, Chance<T>>) -> Result<Self, Self::Error> {
        let out = ProbDist(value);
        out.validate()?;
        Ok(out)
    }
}

impl<T: BasicNum> Default for ProbDist<T> {
    fn default() -> Self {
        Self(BTreeMap::from([(0, Chance::one())]))
    }
}

impl<T> AsRef<BTreeMap<isize, Chance<T>>> for ProbDist<T> {
    fn as_ref(&self) -> &BTreeMap<isize, Chance<T>> {
        &self.0
    }
}

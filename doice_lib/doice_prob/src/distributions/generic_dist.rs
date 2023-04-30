use std::collections::BTreeMap;

use num::One;

use crate::{chance::Chance, prelude::*};

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
            out.insert(*outcome, total);
        }

        out
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

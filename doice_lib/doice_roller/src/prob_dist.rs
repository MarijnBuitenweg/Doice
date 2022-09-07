use std::{
    collections::BTreeMap,
    ops::{Add, Deref, Div, Mul, Neg, Range},
    time::{Duration},
};

use instant::Instant;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

use super::{Roll, Rollable, SampleDist};

const ADD_TIMEOUT: u64 = 2000; //[ms]
const TIMEOUT_CHECK_INTERVAL: usize = 20_000; //[iterations]
const CLT_THRESHOLD: usize = 1000; // Will approximate a summation with more than these terms using CLT

/// Type representing a valid probability mass function
#[derive(Clone)]
pub struct ProbDist(BTreeMap<isize, f64>);

impl ProbDist {
    /// Same as default()
    pub fn new() -> Self {
        Default::default()
    }

    /// Generates a normal distribution for the given range of outcomes
    pub fn normal(mean: f64, variance: f64, range: Range<isize>) -> Self {
        let mut out = BTreeMap::new();
        let sigma = variance.sqrt();

        for outcome in range {
            out.insert(
                outcome,
                (-0.5 * ((outcome as f64 - mean) / sigma).powi(2)).exp(),
            );
        }

        let mut out = ProbDist(out);
        out.proper_scale();
        out
    }

    /// Scale the probabilities such that the total probability is 1
    pub fn proper_scale(&mut self) {
        let total_prob: f64 = self.0.values().sum();
        if total_prob != 1.0f64 {
            let scale_factor = total_prob.recip();
            for v in self.0.values_mut() {
                *v *= scale_factor;
            }
        }
    }

    pub fn min(&self) -> Option<isize> {
        self.0.keys().copied().next()
    }

    pub fn max(&self) -> Option<isize> {
        self.0.keys().copied().next_back()
    }

    /// Removes all probabilities that are equal to 0
    pub fn remove_null(&mut self) {
        self.0.retain(|_, prob| *prob != 0.0);
    }

    /// Expected output AKA average
    /// Equal to moment(1)
    pub fn expectation(&self) -> f64 {
        self.moment(1)
    }

    /// Equal to E(X^n)
    pub fn moment(&self, n: u32) -> f64 {
        self.0
            .iter()
            .map(|(outcome, prob)| outcome.pow(n) as f64 * prob)
            .sum()
    }

    /// Variance of the distribution, equal to sigma^2
    pub fn var(&self) -> f64 {
        self.moment(2) - self.moment(1).powi(2)
    }

    /// The deviation of the distribution, equal to sqrt(var)
    pub fn sigma(&self) -> f64 {
        self.var().sqrt()
    }

    pub fn get_rev_cumulative_prob(&self) -> BTreeMap<isize, f64> {
        let mut total = 0.0;
        let mut out = BTreeMap::new();
        // Iter over a BTreeMap is always sorted, yay
        for (outcome, prob) in self.0.iter().rev() {
            total += prob;
            out.insert(*outcome, total);
        }

        out
    }

    pub fn get_cumulative_prob(&self) -> BTreeMap<isize, f64> {
        let mut total = 0.0;
        let mut out = BTreeMap::new();
        // Iter over a BTreeMap is always sorted, yay
        for (outcome, prob) in self.0.iter() {
            total += prob;
            out.insert(*outcome, total);
        }

        out
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
            dbg!(*first);
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

    pub fn retain<F: FnMut(&isize, &mut f64) -> bool>(&mut self, f: F) {
        self.0.retain(f);
    }

    pub fn read_samples(&mut self, samples: &SampleDist) {
        // First, read all the sample counts
        for (&outcome, &sample) in samples.iter() {
            self.0
                .entry(outcome)
                .and_modify(|v| *v = sample as f64)
                .or_insert(sample as f64);
        }
        // Then rescale it
        self.proper_scale();
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn peak(&self) -> Option<(isize, f64)> {
        self.0.iter().max_by(|x, y| x.1.total_cmp(y.1)).map(|(&k, &v)| (k, v))
    }
}

impl Default for ProbDist {
    fn default() -> Self {
        let mut out = BTreeMap::new();
        out.insert(0, 1.0);
        ProbDist(out)
    }
}

impl Deref for ProbDist {
    type Target = BTreeMap<isize, f64>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Roll> for ProbDist {
    fn from(roll: Roll) -> Self {
        roll.dist()
    }
}

impl Add<&Self> for ProbDist {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        let (shortest, longest) = if self.len() > rhs.len() {
            (rhs, &self)
        } else {
            (&self, rhs)
        };

        let timestamp = Instant::now();
        let mut out = BTreeMap::new();
        for (outcome, prob) in shortest.iter() {
            for (i, (k, v)) in longest.iter().enumerate() {
                out.entry(outcome + k)
                    .and_modify(|e| *e += prob * v)
                    .or_insert(prob * v);
                if i % TIMEOUT_CHECK_INTERVAL == 0
                    && timestamp.elapsed() > Duration::from_millis(5 * ADD_TIMEOUT)
                {
                    //println!("ProbDist::Add<&Self> timed out!");
                    return ProbDist::default();
                }
            }
        }

        ProbDist(out)
    }
}

impl Add<isize> for ProbDist {
    type Output = Self;

    fn add(self, rhs: isize) -> Self::Output {
        let mut out = BTreeMap::new();
        for (k, v) in self.iter() {
            out.insert(k + rhs, *v);
        }
        ProbDist(out)
    }
}

#[cfg(feature = "rayon")]
impl Mul<&Self> for ProbDist {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let (shortest, longest) = if self.len() > rhs.len() {
            (rhs, &self)
        } else {
            (&self, rhs)
        };

        //let mut out = BTreeMap::new();
        let out = shortest
            .par_iter()
            .map(|(&outcome, &prob)| {
                longest
                    .par_iter()
                    .map(move |(k, v)| (k * outcome, prob * v))
                    .collect()
            })
            .reduce(BTreeMap::new, |mut a, b| {
                for (k, v) in b.iter() {
                    // Yes, it totally is suspicious to multiply in the add operation
                    // But I've thought about it and it's fine
                    #[allow(clippy::suspicious_arithmetic_impl)]
                    a.entry(*k).and_modify(|p| *p += v).or_insert(*v);
                }
                a
            });

        // for (outcome, prob) in shortest.iter() {
        //     // For every entry in shortest, add a scaled version of longes to the output
        //     for (k, v) in longest.iter() {
        //         out.entry(outcome * k)
        //             .and_modify(|p| *p += prob * v)
        //             .or_insert(prob * v);
        //     }
        // }

        ProbDist(out)
    }
}

/// Fallback impl when no rayon
#[cfg(not(feature = "rayon"))]
impl Mul<&Self> for ProbDist {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let (shortest, longest) = if self.len() > rhs.len() {
            (rhs, &self)
        } else {
            (&self, rhs)
        };

        let mut out = BTreeMap::new();
        for (outcome, prob) in shortest.iter() {
            // For every entry in shortest, add a scaled version of longes to the output
            for (k, v) in longest.iter() {
                out.entry(outcome * k)
                    .and_modify(|p| *p += prob * v)
                    .or_insert(prob * v);
            }
        }

        ProbDist(out)
    }
}

fn approx_as_norm(dist: ProbDist, rhs: usize) -> ProbDist {
    let new_mean = rhs as f64 * dist.expectation();
    let new_variance = rhs as f64 * dist.var();
    let new_sigma = new_variance.sqrt();
    let min = rhs as isize * dist.0.keys().next().unwrap();
    let max = rhs as isize * dist.0.keys().next_back().unwrap();
    // Find the smallest appropriate range
    let range = if (min.abs_diff(max) as f64) < 8.0 * new_sigma {
        min..(max + 1)
    } else {
        ((new_mean - 4.0 * new_sigma).floor() as isize)
            ..((new_mean + 4.0 * new_sigma).ceil() as isize)
    };

    ProbDist::normal(new_mean, new_variance, range)
}

impl Mul<usize> for ProbDist {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        if rhs > CLT_THRESHOLD {
            return approx_as_norm(self, rhs);
        }

        let mut out = ProbDist::default();
        let timestamp = Instant::now();

        for _count in 0..rhs {
            out = out + &self;
            // If time is running out, approximate as normal dist
            if timestamp.elapsed() > Duration::from_millis(ADD_TIMEOUT) {
                return approx_as_norm(self, rhs);
            }
        }

        out
    }
}

impl Div<&Self> for ProbDist {
    type Output = Self;

    fn div(self, rhs: &Self) -> Self::Output {
        let (shortest, longest) = if self.len() > rhs.len() {
            (rhs, &self)
        } else {
            (&self, rhs)
        };

        let mut out = BTreeMap::new();
        for (outcome, prob) in shortest.iter() {
            // For every entry in shortest, add a scaled version of longes to the output
            for (k, v) in longest.iter() {
                out.entry(k / outcome)
                    .and_modify(|p| *p += prob * v)
                    .or_insert(prob * v);
            }
        }

        ProbDist(out)
    }
}

impl Neg for ProbDist {
    type Output = Self;

    fn neg(self) -> Self::Output {
        ProbDist(self.iter().map(|(k, v)| (-k, *v)).collect())
    }
}

impl TryFrom<BTreeMap<isize, f64>> for ProbDist {
    type Error = ();

    fn try_from(data: BTreeMap<isize, f64>) -> Result<Self, Self::Error> {
        // Compute the total
        let total: f64 = data.iter().map(|(_, v)| *v).sum();
        // If it's close enough to 1, declare it a valid ProbDist
        if (total - 1.0).abs() <= 0.01 {
            Ok(ProbDist(data))
        } else {
            Err(())
        }
    }
}

impl From<ProbDist> for BTreeMap<isize, f64> {
    fn from(dist: ProbDist) -> Self {
        dist.0
    }
}
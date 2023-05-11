use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use rand::{thread_rng, Rng};

use super::ProbDist;

#[derive(Clone, Default, Debug)]
pub struct SampleDist {
    dist: BTreeMap<isize, usize>,
}

impl SampleDist {
    #[must_use]
    pub fn new() -> Self {
        SampleDist::default()
    }

    pub fn add_sample(&mut self, at: isize) {
        self.dist.entry(at).and_modify(|val| *val += 1).or_insert(1);
    }

    pub fn add_samples(&mut self, samples: &[isize]) {
        for sample in samples {
            self.add_sample(*sample);
        }
    }

    pub fn clear(&mut self) {
        self.dist.clear();
    }
}

impl Deref for SampleDist {
    type Target = BTreeMap<isize, usize>;

    fn deref(&self) -> &Self::Target {
        &self.dist
    }
}

impl DerefMut for SampleDist {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.dist
    }
}

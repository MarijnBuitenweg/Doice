use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use rand::{thread_rng, Rng};

use super::{Layouter, ProbDist, Rollable};

#[derive(Clone, Default, Debug)]
pub struct SampleDist {
    dist: BTreeMap<isize, usize>,
}

impl SampleDist {
    #[must_use]
    pub fn new() -> Self {
        Default::default()
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

impl Rollable for SampleDist {
    fn roll(&self) -> super::RollOut {
        let total = self.iter().map(|(_, s)| *s).sum();
        let mut raw_roll = thread_rng().gen_range(1..total);
        let mut out_roll = *self.first_key_value().unwrap().0;
        for (&outcome, &samples) in self.iter() {
            if raw_roll < samples {
                out_roll = outcome;
                break;
            }
            raw_roll -= samples;
        }

        let mut out_txt = Layouter::default();
        out_txt.append(&format!("[{out_roll}]"));

        super::RollOut {
            value: out_roll,
            txt: out_txt,
        }
    }

    fn dist(&self) -> super::ProbDist {
        let mut dist = ProbDist::default();
        dist.read_samples(self);
        dist
    }
}

use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use rand::{thread_rng, Rng};

use crate::Value;

use super::{Layouter, ProbDist, Rollable};

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

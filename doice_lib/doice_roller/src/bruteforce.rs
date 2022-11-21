use std::time::Duration;

use instant::Instant;

use super::{ProbDist, Rollable, SampleDist};

/// Trait enabling the bruteforcing of the probability distribution of any rollable thing
pub trait BruteForceProbDist {
    const BRUTEFORCE_TIME: Duration;

    fn bruteforce_probdist(&self) -> ProbDist;
}

/// Simplest blanket impl of my life lmao
#[cfg(feature = "rayon")]
impl<T: Rollable + Sync> BruteForceProbDist for T {
    const BRUTEFORCE_TIME: Duration = Duration::from_millis(2000);

    fn bruteforce_probdist(&self) -> ProbDist {
        use rayon::prelude::*;
        use std::thread::available_parallelism;

        // Measure time for a single roll
        let start = Instant::now();
        let first_roll = self.roll_quiet();
        let single_roll_time = start.elapsed();
        // Initialize sample dist
        let mut samples = SampleDist::new();
        samples.add_sample(first_roll);
        // Determine how many rolls to do
        let roll_num = available_parallelism().unwrap().get() as u128
            * Self::BRUTEFORCE_TIME.as_nanos()
            / single_roll_time.as_nanos();
        // Then roll them in parallel
        let many_samples: Vec<_> = (0..roll_num)
            .into_par_iter()
            .map(|_| self.roll_quiet())
            .collect();
        // Then add all the samples to the list, and convert the samples to a probdist
        samples.add_samples(&many_samples);
        let mut prob = ProbDist::default();
        prob.read_samples(&samples);
        prob
    }
}

/// If rayon is not enabled, fall back on single threaded impl
#[cfg(not(feature = "rayon"))]
impl<T: Rollable> BruteForceProbDist for T {
    const BRUTEFORCE_TIME: Duration = Duration::from_millis(2000);

    fn bruteforce_probdist(&self) -> ProbDist {
        // Measure time for a single roll
        let start = Instant::now();
        let first_roll = self.roll().value;
        let single_roll_time = start.elapsed();
        // Initialize sample dist
        let mut samples = SampleDist::new();
        samples.add_sample(first_roll);
        // Determine how many rolls to do
        let roll_num = Self::BRUTEFORCE_TIME.as_nanos() / single_roll_time.as_nanos();
        // Then roll them
        let many_samples: Vec<_> = (0..roll_num).map(|_| self.roll().value).collect();
        // Then add all the samples to the list, and convert the samples to a probdist
        samples.add_samples(&many_samples);
        let mut prob = ProbDist::default();
        prob.read_samples(&samples);
        prob
    }
}

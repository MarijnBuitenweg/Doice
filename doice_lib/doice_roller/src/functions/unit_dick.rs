use itertools::Itertools;

use crate::{DiceError, Expression, ProbDist, RollOut, Rollable, SampleDist};

use super::FunctionInit;

const GIRTH: isize = 20;
const LENGTH: f64 = 5.0;
const SCALE: f64 = 1000.0;

#[derive(Clone, Default, Debug)]
pub struct UnitDick {
    dickst: SampleDist,
}

impl FunctionInit for UnitDick {
    fn generate(input: &str) -> Result<Expression, DiceError> {
        if let Some((length, girth)) = input.split(',').collect_tuple() {
            let length = length
                .parse()
                .map_err(|_| "could not parse length in dick")?;
            let girth = girth.parse().map_err(|_| "could not parse girth in dick")?;
            let mut dist = SampleDist::default();
            generate_ball(&mut dist, -2 * girth + 1, girth);
            generate_phallus(&mut dist, girth, length);
            generate_ball(&mut dist, 2 * girth - 1, girth);
            return Ok(Box::new(UnitDick { dickst: dist }));
        }

        let mut dist = SampleDist::default();
        generate_ball(&mut dist, -2 * GIRTH + 1, GIRTH);
        generate_phallus(&mut dist, GIRTH, LENGTH);
        generate_ball(&mut dist, 2 * GIRTH - 1, GIRTH);
        Ok(Box::new(UnitDick { dickst: dist }))
    }
}

impl Rollable for UnitDick {
    fn roll(&self) -> RollOut {
        self.dickst.roll()
    }

    fn dist(&self) -> ProbDist {
        let mut dist = ProbDist::default();
        dist.read_samples(&self.dickst);
        dist
    }
}

fn generate_ball(dist: &mut SampleDist, centre: isize, girth: isize) {
    for x in (-girth)..(girth) {
        let y = (1.0 - (x.pow(2) as f64 / (girth.pow(2) as f64))).sqrt() * SCALE;
        dist.entry(x + centre)
            .and_modify(|v| *v += y as usize)
            .or_insert(y as usize);
    }
}

fn generate_phallus(dist: &mut SampleDist, girth: isize, length: f64) {
    for x in (-girth + 1)..(girth) {
        let y = ((1.0 - (x.pow(2) as f64 / (girth.pow(2) as f64))).sqrt() + length) * SCALE;
        dist.entry(x)
            .and_modify(|v| *v += y as usize)
            .or_insert(y as usize);
    }
}

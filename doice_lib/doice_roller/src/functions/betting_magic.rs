use std::str::FromStr;

use crate::{
    bruteforce::BruteForceProbDist, structure::expression::Expression, utils::split_once_parenth,
    DiceError, DiceRoller, RollOut, Rollable, SampleDist,
};

use super::FunctionInit;

#[derive(Debug, Default, Clone, Copy)]
pub struct BettingMagic {
    chips: usize,
}

impl FunctionInit for BettingMagic {
    const DOC: &'static str = "Betteroo Betteraa";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let chips = input
            .trim()
            .parse()
            .map_err(|_| "number of chips could not be parsed")?;

        Ok(BettingMagic { chips }.into())
    }
}

impl Rollable for BettingMagic {
    fn roll(&self) -> crate::RollOut {
        let roller = DiceRoller::from_str("2d6").unwrap();
        let mut own_chips = 0;
        let _point_roll = roller.roll_quiet();

        let point = match roller.roll_quiet() {
            2 => 0,
            3 | 12 => 0,
            7 | 11 => {
                own_chips += self.chips;
                self.chips
            }
            _ => self.chips,
        };

        crate::RollOut {
            value: point as isize,
            txt: Default::default(),
        }
    }

    fn dist(&self) -> crate::ProbDist {
        self.bruteforce_probdist()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BettingMagic2 {
    chips: isize,
    late_chips: isize,
    point: bool,
}

impl FunctionInit for BettingMagic2 {
    const DOC: &'static str = "Betteroo Betteraa";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (chips, input) = split_once_parenth(input, ',').ok_or("invalid args passed to bet2")?;
        let (late_chips, point) =
            split_once_parenth(input, ',').ok_or("invalid args passed to bet2")?;

        let chips = chips
            .trim()
            .parse()
            .map_err(|_| "number of chips could not be parsed")?;

        let late_chips = late_chips
            .trim()
            .parse()
            .map_err(|_| "number of late chips could not be parsed")?;

        let point = point.contains('p'); //bet2(10, p)

        Ok(BettingMagic2 {
            chips,
            late_chips,
            point,
        }
        .into())
    }
}

impl Rollable for BettingMagic2 {
    fn roll(&self) -> crate::RollOut {
        let roller = DiceRoller::from_str("2d6").unwrap();
        let mut own_chips = 0;
        let point_roll = roller.roll_quiet();

        let point = match point_roll {
            2 => 0,
            3 | 12 => 0,
            7 | 11 => {
                own_chips += self.chips;
                0
            }
            _ => self.chips,
        };

        if point == 0 {
            return RollOut {
                value: own_chips + self.late_chips,
                txt: Default::default(),
            };
        }

        let dist = roller.dist();
        let double_chance = if self.point {
            *dist.get(&point_roll).unwrap()
        } else {
            0.0
        };
        let lose_chance = if self.point {
            *dist.get(&7).unwrap() + *dist.get(&2).unwrap()
        } else {
            *dist.get(&point_roll).unwrap() + *dist.get(&2).unwrap()
        };
        let keep_chance = if !self.point {
            *dist.get(&7).unwrap()
        } else {
            0.0
        };

        let mut sadist = SampleDist::new();
        let factor = (2_i32.pow(16) - 1) as f64;
        sadist
            .entry(0)
            .or_insert((lose_chance * factor).floor() as usize);
        sadist
            .entry(1)
            .or_insert((keep_chance * factor).floor() as usize);
        sadist
            .entry(2)
            .or_insert((double_chance * factor).floor() as usize);

        RollOut {
            value: (point + self.late_chips) * sadist.roll_quiet(),
            txt: Default::default(),
        }
    }

    fn dist(&self) -> crate::ProbDist {
        self.bruteforce_probdist()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BettingMagic3 {
    chips: isize,
    late_chips: isize,
    smort: bool,
}

impl FunctionInit for BettingMagic3 {
    const DOC: &'static str = "Betteroo Betteraa";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (chips, input) = split_once_parenth(input, ',').ok_or("invalid args passed to bet2")?;
        let (late_chips, smort) =
            split_once_parenth(input, ',').ok_or("invalid args passed to bet2")?;

        let chips = chips
            .trim()
            .parse()
            .map_err(|_| "number of chips could not be parsed")?;

        let late_chips = late_chips
            .trim()
            .parse()
            .map_err(|_| "number of late chips could not be parsed")?;

        let smort = smort.contains('s'); //bet2(20, 80, s)

        Ok(BettingMagic3 {
            chips,
            late_chips,
            smort,
        }
        .into())
    }
}

impl Rollable for BettingMagic3 {
    fn roll(&self) -> crate::RollOut {
        let roller = DiceRoller::from_str("2d6").unwrap();
        let mut own_chips = 0;
        let point_roll = roller.roll_quiet();

        let point = match point_roll {
            2 => 0,
            3 | 12 => 0,
            7 | 11 => {
                own_chips += self.chips;
                0
            }
            _ => self.chips,
        };

        if point == 0 {
            return RollOut {
                value: own_chips + self.late_chips,
                txt: Default::default(),
            };
        }

        let dist = roller.dist();
        let point_bet = if self.smort {
            point_roll == 8 || point_roll == 6 || point_roll == 5 || point_roll == 9
        } else {
            true
        };
        let double_chance = if point_bet {
            *dist.get(&point_roll).unwrap()
        } else {
            0.0
        };
        let lose_chance = if point_bet {
            *dist.get(&7).unwrap() + *dist.get(&2).unwrap()
        } else {
            *dist.get(&point_roll).unwrap() + *dist.get(&2).unwrap()
        };
        let keep_chance = if !point_bet {
            *dist.get(&7).unwrap()
        } else {
            0.0
        };

        let mut sadist = SampleDist::new();
        let factor = (2_i32.pow(16) - 1) as f64;
        sadist
            .entry(0)
            .or_insert((lose_chance * factor).floor() as usize);
        sadist
            .entry(1)
            .or_insert((keep_chance * factor).floor() as usize);
        sadist
            .entry(2)
            .or_insert((double_chance * factor).floor() as usize);

        RollOut {
            value: (point + self.late_chips) * sadist.roll_quiet(),
            txt: Default::default(),
        }
    }

    fn dist(&self) -> crate::ProbDist {
        self.bruteforce_probdist()
    }
}

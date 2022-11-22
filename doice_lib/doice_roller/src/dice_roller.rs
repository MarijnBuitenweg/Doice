use itertools::Itertools;
use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, str::FromStr};

use crate::Value;

use super::{layouter::Layouter, prob_dist::ProbDist, RollOut, Rollable};

use rand::{distributions::Uniform, prelude::*};

const MAX_DIE: usize = 1_000_000;
const MAX_DICE_COUNT: usize = usize::MAX;
const MAX_TEXT_DICE: usize = 1000;

#[derive(Clone, Default, Debug)]
pub struct DiceRoller {
    dice_type: usize,
    dice_count: usize,
    advantage: isize,
}

impl DiceRoller {
    /// This makes it efficient to generate an arbitrary diceroll.
    /// The normal maximum values for a diceroll are not enforced here!
    #[must_use]
    pub fn new(dice_type: usize, dice_count: usize, advantage: isize) -> Self {
        Self {
            dice_type,
            dice_count,
            advantage,
        }
    }

    /// Calculates the average of itself, not taking into account any modifiers like advantage or adapters
    #[must_use]
    pub fn unmod_avg(&self) -> f64 {
        (self.dice_count * self.dice_type) as f64 / 2.0 + 0.5
    }

    /// Rolls 1 die, without producing text
    #[must_use]
    fn roll_quiet(&self) -> isize {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as isize + 1);
        match self.advantage.cmp(&0) {
            Ordering::Less => rng
                .sample_iter(dist)
                .take(self.advantage.unsigned_abs())
                .min()
                .unwrap(),
            Ordering::Equal => rng.sample(dist),
            Ordering::Greater => rng
                .sample_iter(dist)
                .take(self.advantage as usize)
                .max()
                .unwrap(),
        }
    }

    /// Rolls n dice, without producing text
    #[must_use]
    pub fn roll_n_quiet(&self, n: usize) -> Vec<isize> {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as isize + 1);
        let mut rolls = Vec::with_capacity(n);
        let mut buf = vec![0; 1 + self.advantage.unsigned_abs()];

        // Perform the dice rolls, with (dis)advantage
        for _ in 0..n {
            // Refill the buffer with rolls
            for num in buf.iter_mut() {
                *num = rng.sample(dist);
            }
            rolls.push(match self.advantage.cmp(&0) {
                Ordering::Equal => buf[0],
                Ordering::Greater => {
                    let roll;
                    (_, roll) = buf
                        .iter()
                        .enumerate()
                        .max_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
                Ordering::Less => {
                    let roll;
                    (_, roll) = buf
                        .iter()
                        .enumerate()
                        .min_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
            });
        }

        rolls
    }

    #[must_use]
    pub fn dice_type(&self) -> usize {
        self.dice_type
    }

    #[must_use]
    pub fn dice_count(&self) -> usize {
        self.dice_count
    }

    #[must_use]
    pub fn advantage(&self) -> isize {
        self.advantage
    }

    pub fn set_dice_count(&mut self, dice_count: usize) {
        self.dice_count = dice_count;
    }
}

impl FromStr for DiceRoller {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.trim();
        // Determine the length of the number used to set the number of dice
        let count_len = value.chars().take_while(|c| c.is_numeric()).count();
        // Try to parse the number of dice, on fail use 1
        let dice_count = value[0..count_len].parse().unwrap_or(1);
        // Check if d is present
        let mut chars = value.chars().skip(count_len);
        if chars
            .next()
            .ok_or_else(|| "Dice string too short!".to_string())?
            != 'd'
        {
            return Err("Improper dice definition! [no d]".to_string());
        }

        // Handling advantage flags
        let mut adv_flag_count = 0;
        let mut advantage = 0;
        for c in chars.take_while(|c| "&|".contains(*c)) {
            adv_flag_count += 1;
            advantage += match c {
                '|' => 1,
                '&' => -1,
                _ => 0,
            };
        }

        // Determine where the dice type should start
        let pre_type_len = count_len + 1 + adv_flag_count;
        // Determine how long the dice type is
        let type_len = value
            .chars()
            .skip(pre_type_len)
            .take_while(|c| c.is_numeric())
            .count();
        // Then parse it, 20 seems like sensible default
        let dice_type = value[pre_type_len..(pre_type_len + type_len)]
            .parse()
            .unwrap_or(20);

        if dice_type > MAX_DIE {
            return Err("Die size too large!".to_string());
        }

        if dice_type == 0 {
            return Err("0-dice are not supported!".to_string());
        }

        // if dice_count > MAX_DICE_COUNT {
        //     return Err("Too many dice!".to_string());
        // }

        let roller = DiceRoller {
            dice_type,
            dice_count,
            advantage,
        };

        Ok(roller)
    }
}

impl Rollable for DiceRoller {
    fn roll(&self) -> super::RollOut {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as isize + 1);
        let mut roll_total = 0;
        let mut buf = vec![0; 1 + self.advantage.unsigned_abs()];
        let mut out_txt = Layouter::default();
        let do_txt = self.dice_count < MAX_TEXT_DICE;

        if self.dice_count > 1 {
            out_txt.append("[");
        }

        // Perform the dice rolls, with (dis)advantage
        for count in 0..self.dice_count {
            // Refill the buffer with rolls
            for num in &mut buf {
                *num = rng.sample(dist);
            }
            let mut used_i = 0;
            roll_total += match self.advantage.cmp(&0) {
                Ordering::Equal => buf[0],
                Ordering::Greater => {
                    let roll;
                    (used_i, roll) = buf
                        .iter()
                        .enumerate()
                        .max_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
                Ordering::Less => {
                    let roll;
                    (used_i, roll) = buf
                        .iter()
                        .enumerate()
                        .min_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
            };

            // Add the text for this one diceroll
            if do_txt {
                out_txt.append("[");
                for (i, elem) in buf
                    .iter()
                    .map(ToString::to_string)
                    .enumerate()
                    .intersperse((usize::MAX, " ".to_string()))
                {
                    if [usize::MAX, used_i].contains(&i) {
                        out_txt.append(&elem);
                    } else {
                        out_txt.append_strikethrough(&elem);
                    }
                }
                out_txt.append("]");

                if count != self.dice_count - 1 {
                    out_txt.append(" + ");
                }
            }
        }

        if !do_txt {
            out_txt.append("...");
        }

        if self.dice_count > 1 {
            out_txt.append("]");
        }

        RollOut {
            value: roll_total,
            txt: out_txt,
        }
    }

    fn dist(&self) -> super::prob_dist::ProbDist {
        // Build distribution for single die
        let density: f64 = 1.0f64 / (self.dice_type as f64);
        let dist: BTreeMap<_, _> = (1..=self.dice_type)
            .map(|res| (res as isize, density))
            .collect();
        let mut dist = ProbDist::try_from(dist).expect("Bad single die probability distribution!");
        // Extend it for advantage
        dist.apply_advantage(self.advantage);

        // Then extend it for multiple
        dist = dist * self.dice_count;

        dist
    }

    fn roll_quiet(&self) -> isize {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as Value + 1);
        let mut buf = vec![0; 1 + self.advantage.unsigned_abs()].into_boxed_slice();

        // Perform the dice rolls, with (dis)advantage
        let rolls = (0..self.dice_count).map(|_| {
            // Refill the buffer with rolls
            for num in buf.iter_mut() {
                *num = rng.sample(dist);
            }
            // Then take the correct value based on the advantage
            match self.advantage.cmp(&0) {
                Ordering::Equal => buf[0],
                Ordering::Greater => *buf.iter().max().expect("Empty Buffer?"),
                Ordering::Less => *buf.iter().min().expect("Empty Buffer?"),
            }
        });

        rolls.sum()
    }
}

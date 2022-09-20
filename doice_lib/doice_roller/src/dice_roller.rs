use itertools::Itertools;
use std::{cmp::Ordering, collections::BTreeMap, fmt::Debug, str::FromStr};

use super::{layouter::Layouter, prob_dist::ProbDist, RollOut, Rollable, Value};

use dyn_clone::DynClone;
use rand::{distributions::Uniform, prelude::*};

use crate::dice_adapters::generate_dice_adapters;

const MAX_DIE: usize = 1_000_000;
const MAX_DICE_COUNT: usize = 1_000_000_000;

pub trait DiceAdapter: DynClone + Send + Sync + Debug {
    /// Will be called on the entire output
    fn run(&self, input: Vec<Value>, out_txt: &mut Layouter, d_info: &DiceRoller) -> Vec<Value>;
    /// Will be called to find the probDist of a single die
    fn dist(&self, input: ProbDist) -> ProbDist;
}

pub type DiceOption = Box<dyn DiceAdapter>;

impl<T: DiceAdapter + 'static> From<T> for DiceOption {
    fn from(opt: T) -> Self {
        Box::new(opt)
    }
}

impl Clone for DiceOption {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

#[derive(Clone, Default, Debug)]
pub struct DiceRoller {
    dice_type: usize,
    dice_count: usize,
    advantage: isize,
    option: Option<DiceOption>,
}

impl DiceRoller {
    /// This makes it efficient to generate an arbitrary diceroll.
    /// The normal maximum values for a diceroll are not enforced here!
    pub fn new(
        dice_type: usize,
        dice_count: usize,
        advantage: isize,
        option: Option<DiceOption>,
    ) -> Self {
        Self {
            dice_type,
            dice_count,
            advantage,
            option,
        }
    }

    /// Calculates the average of itself, not taking into account any modifiers like advantage or adapters
    pub fn unmod_avg(&self) -> f64 {
        (self.dice_count * self.dice_type) as f64 / 2.0 + 0.5
    }

    /// Rolls 1 die, without producing text
    pub fn roll_quiet(&self) -> isize {
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

    pub fn dice_type(&self) -> usize {
        self.dice_type
    }

    pub fn dice_count(&self) -> usize {
        self.dice_count
    }

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

        let pre_adapter_len = pre_type_len + type_len;

        if dice_type > MAX_DIE {
            return Err("Die size too large!".to_string());
        }

        if dice_type == 0 {
            return Err("0-dice are not supported!".to_string());
        }

        if dice_count > MAX_DICE_COUNT {
            return Err("Too many dice!".to_string());
        }

        let mut roller = DiceRoller {
            dice_type,
            dice_count,
            advantage,
            option: Default::default(),
        };

        roller.option = generate_dice_adapters(&value[pre_adapter_len..], &roller);

        Ok(roller)
    }
}

impl Rollable for DiceRoller {
    fn roll(&self) -> super::RollOut {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as isize + 1);
        let mut rolls = Vec::with_capacity(self.dice_count);
        let mut buf = vec![0; 1 + self.advantage.unsigned_abs()];
        let mut out_txt = Layouter::default();
        let do_txt = self.dice_count < MAX_DICE_COUNT / 2_usize.pow(10);

        if self.dice_count > 1 {
            out_txt.append("[");
        }

        // Perform the dice rolls, with (dis)advantage
        for count in 0..self.dice_count {
            // Refill the buffer with rolls
            for num in buf.iter_mut() {
                *num = rng.sample(dist);
            }
            let mut used_i = 0;
            rolls.push(match self.advantage.cmp(&0) {
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
            });

            // Add the text for this one diceroll
            if do_txt {
                out_txt.append("[");
                for (i, elem) in buf
                    .iter()
                    .map(|n| n.to_string())
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

        // Execute the diceoption
        if let Some(opt) = &self.option {
            rolls = opt.run(rolls, &mut out_txt, self);
        }

        RollOut {
            value: rolls.iter().sum(),
            txt: out_txt,
        }
    }

    fn dist(&self) -> super::prob_dist::ProbDist {
        // Build distribution for single die
        let density: f64 = 1.0f64 / (self.dice_type as f64);
        let dist: BTreeMap<_, _> = (1..(self.dice_type + 1))
            .map(|res| (res as isize, density))
            .collect();
        let mut dist = ProbDist::try_from(dist).expect("Bad single die probability distribution!");
        // Extend it for advantage
        dist.apply_advantage(self.advantage);
        // Then apply option
        if let Some(opt) = &self.option {
            dist = opt.dist(dist);
        }

        // Then extend it for multiple
        dist = dist * self.dice_count;

        dist
    }

    fn roll_quiet(&self) -> isize {
        let mut rng = thread_rng();
        let dist = Uniform::<isize>::new(1, self.dice_type as isize + 1);
        let mut buf = vec![0; 1 + self.advantage.unsigned_abs()];
        let mut out_txt = Layouter::default();
        let _do_txt = self.dice_count < MAX_DICE_COUNT / 2_usize.pow(10);

        if self.dice_count > 1 {
            out_txt.append("[");
        }

        // Perform the dice rolls, with (dis)advantage
        let rolls = (0..self.dice_count).map(|_| {
            // Refill the buffer with rolls
            for num in buf.iter_mut() {
                *num = rng.sample(dist);
            }
            let mut _used_i = 0;
            match self.advantage.cmp(&0) {
                Ordering::Equal => buf[0],
                Ordering::Greater => {
                    let roll;
                    (_used_i, roll) = buf
                        .iter()
                        .enumerate()
                        .max_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
                Ordering::Less => {
                    let roll;
                    (_used_i, roll) = buf
                        .iter()
                        .enumerate()
                        .min_by_key(|v| v.1)
                        .expect("Empty Buffer?");
                    *roll
                }
            }
        });

        // Execute the diceoption
        // if let Some(opt) = &self.option {
        //     rolls = opt.run(rolls, &mut out_txt, self);
        // }

        rolls.sum()
    }
}

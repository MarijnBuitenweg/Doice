use std::str::FromStr;

use crate::{BruteForceProbDist, DiceRoller, Expression, Layouter, RollOut, Rollable};

use super::{crit::Crit, FunctionInit};

/// atk(dmgroll, bonus, to_hit, ac, (adv))
/// Will probably be called from character manager to display avg dmg off attacks
#[derive(Default, Clone, Debug)]
pub struct Attack {
    rolls: Expression,
    dmg_bonus: u8,
    d20: DiceRoller,
    ac: u8,
    to_hit_bonus: u8,
    critters: Box<[Expression]>,
}

impl Rollable for Attack {
    fn roll(&self) -> RollOut {
        let to_hit_roll = self.d20.roll_quiet();
        let to_hit = to_hit_roll + self.to_hit_bonus as isize;
        let mut out_txt = Layouter::new();
        let ac = self.ac;

        // Handle to hit
        let damage = match to_hit_roll {
            // Miss
            1 => 0,
            // Maybe hit
            2..=19 => {
                if to_hit >= ac as isize {
                    self.rolls.roll_quiet() + self.dmg_bonus as isize
                } else {
                    0
                }
            }
            // Crit
            20 => {
                self.critters.iter().map(|c| c.roll_quiet()).sum::<isize>()
                    + self.dmg_bonus as isize
            }
            _ => {
                panic!("Invalid d20 roll.")
            }
        };

        out_txt.append(&format!(
            "[{to_hit_roll} + {0} to hit against {ac} -> {damage}]",
            self.to_hit_bonus,
        ));

        RollOut {
            value: damage,
            txt: out_txt,
        }
    }

    fn dist(&self) -> crate::ProbDist {
        self.bruteforce_probdist()
    }
}

impl FunctionInit for Attack {
    const DOC: &'static str = "Performs attack and damage rolls for an attack against a target with a given AC.\nUsage: atk(dmg roll, dmg bonus, to hit bonus, ac, (adv))";

    fn generate(input: &str) -> Result<Expression, crate::DiceError> {
        let (dmg_roll, input) = input.split_once(',').ok_or("no dmg roll provided")?;
        let (dmg_bonus, input) = input.split_once(',').ok_or("no dmg bonus provided")?;
        let (to_hit_bonus, input) = input.split_once(',').ok_or("no to hit bonus provided")?;
        let (ac, input) = input.split_once(',').ok_or("no ac provided")?;
        let adv = input.contains("adv");

        let crit_vec = vec![Crit::generate(dmg_roll).unwrap()];
        Ok(Attack {
            rolls: DiceRoller::from_str(dmg_roll).unwrap().into(),
            dmg_bonus: dmg_bonus.parse().or(Err("invalid dmg bonus"))?,
            d20: if adv { "d|" } else { "d" }.parse().unwrap(),
            ac: ac.parse().or(Err("invalid ac"))?,
            to_hit_bonus: to_hit_bonus.parse().or(Err("invalid to hit bonus"))?,
            critters: crit_vec.into_boxed_slice(),
        }
        .into())
    }
}

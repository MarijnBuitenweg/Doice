use crate::{
    dice_roller::DiceRoller, layouter::LINE_ORANGE, BruteForceProbDist, DiceError, Expression,
    ProbDist, RollOut, Rollable,
};
use egui::TextFormat;
use itertools::Itertools;

use super::FunctionInit;

#[derive(Clone, Debug)]
pub struct Empower {
    roll: DiceRoller,
    single_roll: DiceRoller,
    prof: usize,
}

impl FunctionInit for Empower {
    const DOC: &'static str =
        "Rolls damage roll as if the roll was empowered (5e sorcerer metamagic).\nUsage: emp(dmg roll, prof bonus)";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        // Extract arguments
        let (roll, prof) = input
            .split_once(',')
            .ok_or("improper arguments given to empower")?;

        let roll: DiceRoller = roll.trim().parse()?;
        let mut single_roll = roll.clone();
        single_roll.set_dice_count(1);

        // Parse arguments
        Ok(Box::new(Empower {
            roll,
            single_roll,
            prof: prof
                .trim()
                .parse()
                .map_err(|_| "proficiency bonus could not be parsed in empower")?,
        }))
    }
}

impl Rollable for Empower {
    fn roll(&self) -> RollOut {
        let roll_cnt = self.roll.dice_count();
        let avg = self.single_roll.unmod_avg() as isize;
        let mut rerolls = self.prof;
        let mut rolls = (0..roll_cnt).map(|_| self.single_roll.roll()).collect_vec();
        let mut rerolled = vec![false; roll_cnt];

        // L Ö Ö P
        let mut should_retry = true;
        while should_retry && rerolls != 0 {
            should_retry = false;
            // If it can find one to reroll, reroll it and look again
            if let Some((roll, rerolled)) = rolls
                .iter_mut()
                .zip(rerolled.iter_mut())
                .filter(|(_, rerolled)| !**rerolled)
                .filter(|(roll, _)| roll.value < avg)
                .min_by_key(|(roll, _)| roll.value)
            {
                reroll(&self.single_roll, roll);
                *rerolled = true;
                should_retry = true;
                rerolls -= 1;
            }
        }

        // Combine output
        let mut out = RollOut::default();
        out.txt.append("[");
        out = rolls.into_iter().fold(out, |a, b| a + b);
        out.txt.append("]");
        out
    }

    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}

fn reroll(single_roll: &DiceRoller, roll: &mut RollOut) {
    // Strikethrough the old
    roll.txt.sections[1].1 = TextFormat {
        strikethrough: LINE_ORANGE,
        ..Default::default()
    };

    // And add the new roll
    let new_roll = single_roll.roll_quiet();
    roll.txt.sections[2].0 = format!("->{new_roll}]");
    roll.value = new_roll;
}

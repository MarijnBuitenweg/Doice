use egui::TextFormat;
use itertools::Itertools;

use crate::{
    dice_roller::DiceRoller, layouter::LINE, BruteForceProbDist, DiceError, Expression, Layouter,
    ProbDist, RollOut, Rollable,
};

use super::FunctionInit;

#[derive(Clone, Default)]
pub struct StatRoller;

impl FunctionInit for StatRoller {
    fn generate(_: &str) -> Result<Expression, DiceError> {
        Ok(Box::new(Self))
    }
}

impl Rollable for StatRoller {
    fn roll(&self) -> RollOut {
        // Construct basic dice roll
        let roll: DiceRoller = "1d6".parse().unwrap();
        // Roll it
        let mut roll_outs = (0..4).map(|_| Rollable::roll(&roll)).collect_vec();
        let mut total: isize = roll_outs.iter_mut().map(|out| out.value).sum();
        let min = roll_outs.iter_mut().min_by_key(|out| out.value).unwrap();
        min.txt.sections[1].1 = TextFormat {
            strikethrough: LINE,
            ..Default::default()
        };
        total -= min.value;

        let mut out_txt = Layouter::default();
        out_txt.append("[");
        for roll in roll_outs {
            out_txt = out_txt + roll.txt;
            out_txt.append(" ");
        }
        out_txt.pop();
        out_txt.append("]");

        RollOut {
            value: total,
            txt: out_txt,
        }
    }

    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}

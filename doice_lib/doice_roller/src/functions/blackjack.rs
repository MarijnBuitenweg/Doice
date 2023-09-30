use crate::{DiceError, ProbDist, Rollable, RollOut};
use crate::bruteforce::BruteForceProbDist;
use crate::functions::FunctionInit;
use crate::functions::outcomes::Outcomes;
use crate::structure::expression::Expression;

#[derive(Debug, Clone)]
pub struct Blackjack {
    rounds: usize,
    initial: isize,
    outcomes: Expression,
}

impl Blackjack {
    fn sum_total(&self, rolls: &[isize]) -> isize {
        let total = rolls.iter().sum::<isize>() + self.initial;
        if total > 21 {
            0
        } else {
            total
        }
    }
}

impl FunctionInit for Blackjack {
    const DOC: &'static str = "Returns the outcome of n rounds of blackjack.\nUsage: blackjack(n, (initial hand value))";

    fn generate(input: &str) -> Result<Expression, DiceError> {
        let (rounds_txt, initial) = match input.split_once(',') {
            Some((rounds, initial)) => (rounds, initial.trim().parse().or(Err("Invalid initial value."))?),
            None => (input.trim(), 0),
        };

        Ok(Blackjack {
            rounds: rounds_txt.parse::<usize>().or(Err("Invalid number of rounds."))?,
            initial,
            outcomes: Outcomes::generate("2,3,4,5,6,7,8,9,10,10,10,10,11")?,
        }.into())
    }
}

impl Rollable for Blackjack {
    fn roll(&self) -> RollOut {
        let mut rolls = (0..self.rounds).map(|_| self.outcomes.roll_quiet()).collect::<Vec<_>>();
        while rolls.iter().sum::<isize>() + self.initial > 21isize && rolls.contains(&11) {
            *rolls.iter_mut().find(|&&mut x| x == 11).unwrap() = 1;
        }

        let total = self.sum_total(&rolls);

        RollOut {
            value: total,
            txt: format!("Blackjack: {:?}", rolls).into(),
        }
    }

    fn dist(&self) -> ProbDist {
        self.bruteforce_probdist()
    }
}



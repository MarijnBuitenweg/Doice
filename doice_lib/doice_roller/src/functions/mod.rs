use self::crit::Crit;

use super::{DiceError, Expression, Rollable};

mod adv;
mod attack;
mod crit;
mod empower;
mod stat_roll;
mod sum;
mod unit_dick;

pub trait FunctionInit: Rollable {
    fn generate(input: &str) -> Result<Expression, DiceError>;
}

type FunctionGenerator = fn(&str) -> Result<Expression, DiceError>;
const FUNCTION_GENERATORS: &[(&str, FunctionGenerator)] = &[
    ("emp", empower::Empower::generate),
    ("stat", stat_roll::StatRoller::generate),
    ("dick", unit_dick::UnitDick::generate),
    ("adv", adv::Adv::generate),
    ("sum", sum::Sum::generate),
    ("crit", Crit::generate),
    ("atk", attack::Attack::generate),
];

pub fn interpret_function(src: &str) -> Result<Expression, DiceError> {
    let (ident, input) = src
        .split_once('(')
        .ok_or_else(|| "function string lacks '('".to_string())?;
    let input = input
        .strip_suffix(')')
        .ok_or_else(|| "missing closing ')' in function string".to_string())?;

    let (_, generator) = FUNCTION_GENERATORS
        .iter()
        .find(|(name, _)| ident == *name)
        .ok_or_else(|| "Function name could not be found".to_string())?;
    (generator)(input)
}

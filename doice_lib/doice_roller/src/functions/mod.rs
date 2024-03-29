use self::crit::Crit;

use super::{DiceError, Expression, Rollable};

mod add_nonzero;
mod adv;
mod attack;
mod bernoulli;
mod betting_magic;
mod crit;
mod empower;
mod mirror;
mod stat_roll;
mod sum;
mod unit_dick;
mod poisson;
mod outcomes;
mod blackjack;
mod panic;

pub trait FunctionInit: Rollable {
    const DOC: &'static str;

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
    ("mirror", mirror::Mirror::generate),
    ("ber", bernoulli::Bernoulli::generate),
    ("addnz", add_nonzero::AddNonZero::generate),
    ("bet", betting_magic::BettingMagic::generate),
    ("bet2", betting_magic::BettingMagic2::generate),
    ("bet3", betting_magic::BettingMagic3::generate),
    ("poisson", poisson::Poisson::generate),
    ("outcomes", outcomes::Outcomes::generate),
    ("blackjack", blackjack::Blackjack::generate),
    ("panic", panic::Panic::generate),
];

pub const FUNCTION_DOCS: &[(&str, &str)] = &[
    ("Empower", empower::Empower::DOC),
    ("Stat roll", stat_roll::StatRoller::DOC),
    ("Advantage", adv::Adv::DOC),
    ("Sum", sum::Sum::DOC),
    ("Critical attack damage", Crit::DOC),
    ("Attack", attack::Attack::DOC),
    ("Mirror", mirror::Mirror::DOC),
    ("Bernoulli / Coin toss", bernoulli::Bernoulli::DOC),
    ("Poisson", poisson::Poisson::DOC),
    ("Add nonzero", add_nonzero::AddNonZero::DOC),
    ("List of outcomes", outcomes::Outcomes::DOC),
    ("Blackjack", blackjack::Blackjack::DOC),
    ("Panic", panic::Panic::DOC),
];

pub fn interpret_function(src: &str) -> Result<Expression, DiceError> {
    let src = src.trim();
    let (ident, input) = src.split_once('(').ok_or("function string lacks '('")?;
    let input = input
        .strip_suffix(')')
        .ok_or("missing closing ')' in function string")?;

    let (_, generator) = FUNCTION_GENERATORS
        .iter()
        .find(|(name, _)| ident == *name)
        .ok_or("Function name could not be found")?;
    (generator)(input)
}

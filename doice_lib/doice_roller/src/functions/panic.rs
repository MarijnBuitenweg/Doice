use crate::{DiceError, ProbDist, Rollable, RollOut};
use crate::functions::FunctionInit;
use crate::structure::expression::Expression;

#[derive(Debug, Default, Clone)]
pub struct Panic {}

impl FunctionInit for Panic {
    const DOC: &'static str = "Will halt and catch fire.\nUsage: panic()";

    fn generate(_input: &str) -> Result<Expression, DiceError> {
        panic!("The user wants to see the world BURN.")
    }
}
impl Rollable for Panic {
    fn roll(&self) -> RollOut {
        todo!()
    }

    fn dist(&self) -> ProbDist {
        todo!()
    }
}
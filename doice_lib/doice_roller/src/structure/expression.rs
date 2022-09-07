use std::str::FromStr;

use crate::{
    dice_roller::DiceRoller, functions::interpret_function, DiceError, Expression, Rollable,
};

use super::{literal::Literal, nop::Nothing, parenth::Parenth};

impl FromStr for Expression {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // If there are parentheses, the expression must either be a function, or just some stuff in parentheses
        if let Some(i) = src.find('(') {
            // Guard to detect unclosed parentheses
            match src.find(')') {
                Some(j) if j <= i => Err("')' before '('"),
                None => Err("no ')' after '('"),
                _ => Ok(()),
            }?;

            // If src starts with '(' the expression is just some stuff in parentheses
            if i == 0 {
                return Ok(Parenth::from_str(src)?.into());
            // Otherwise, there is some text before '(', so the expression must be a function
            } else {
                return interpret_function(src);
            }
        }

        // If there are no parentheses, the expression must be a literal or a diceroll
        // A literal will only contain a 'd' if it's in hex, which it will only be if it contains 0x
        // So if an expression contains a d and no 0x, it is a dice roll
        // If it is not a dice roll, it must be a literal
        let lower = src.to_lowercase();
        if lower.contains('d') && !lower.contains("0x") {
            Ok(DiceRoller::from_str(src)?.into())
        } else {
            Ok(Literal::from_str(src)?.into())
        }
    }
}

impl<T: Rollable + 'static + Send + Sync> From<T> for Expression {
    fn from(expr: T) -> Self {
        Box::new(expr)
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        dyn_clone::clone_box(self.as_ref())
    }
}

impl Default for Expression {
    fn default() -> Self {
        Nothing::default().into()
    }
}

use std::str::FromStr;

use crate::{
    layouter::Layouter, prob_dist::ProbDist, DiceError, Expression, RollOut, Rollable, Value,
};

use super::{literal::Literal, nop::Nothing};

#[derive(Clone, Debug)]
pub enum Sign {
    Positive,
    Negative,
}

impl Sign {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sign::Positive => " + ",
            Sign::Negative => " - ",
        }
    }
}

impl From<Sign> for &str {
    fn from(value: Sign) -> Self {
        value.as_str()
    }
}

#[derive(Clone, Debug)]
enum Operator {
    Mul(Box<Term>),
    Div(Box<Term>),
    Nop(),
}

#[derive(Clone, Debug)]
pub struct Term {
    roll: Expression,
    op: Operator,
    sign: Sign,
}

impl Term {
    pub fn sign(&self) -> &Sign {
        &self.sign
    }
}

impl From<Expression> for Term {
    fn from(expr: Expression) -> Self {
        Term {
            roll: expr,
            op: Operator::Nop(),
            sign: Sign::Positive,
        }
    }
}

impl From<isize> for Term {
    fn from(value: isize) -> Self {
        let sign = if value >= 0 {
            Sign::Positive
        } else {
            Sign::Negative
        };

        Term {
            roll: Literal::from(value.abs()).into(),
            op: Operator::Nop(),
            sign,
        }
    }
}

impl FromStr for Term {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        // If src is empty, return an empty term
        if src.is_empty() {
            return Ok(Term {
                roll: Nothing::new().into(),
                op: Operator::Nop(),
                sign: Sign::Positive,
            });
        }

        // Trim the term for simplicity and safety
        let mut src = dbg!(src.trim());

        let first_char = src.as_bytes()[0].into();
        let sign = match first_char {
            '-' => Sign::Negative,
            _ => Sign::Positive,
        };

        // Skip initial + or -
        if "+-".contains(first_char) {
            (_, src) = src.split_at(1);
        }

        let mut roll_txt = src;

        let mut parenth = 0;
        let op = if let Some((i, c)) = src
            .char_indices()
            // Parenth filter
            .filter(|(_, c)| {
                if *c == ')' {
                    parenth -= 1;
                    parenth = parenth.max(0);
                }
                let par_out = parenth;
                if *c == '(' {
                    parenth += 1;
                }
                par_out == 0
            })
            .find(|(_, c)| "/*".contains(*c))
        {
            let mut mul_txt: &str;
            (roll_txt, mul_txt) = src.split_at(i);
            mul_txt = mul_txt.trim_start_matches(c);
            match c {
                '*' => Operator::Mul(Box::new(mul_txt.parse()?)),
                _ => Operator::Div(Box::new(mul_txt.parse()?)),
            }
        } else {
            Operator::Nop()
        };

        Ok(Term {
            roll: roll_txt.parse()?,
            op,
            sign,
        })
    }
}

impl Rollable for Term {
    fn roll(&self) -> RollOut {
        let rhs = self.roll.roll();

        let mut out = match &self.op {
            Operator::Mul(expr) => {
                let mut res = expr.roll();
                res.txt.append_front("*");
                res.value *= rhs.value;
                res
            }
            Operator::Div(expr) => {
                let mut res = expr.roll();
                res.txt.append_front("/");
                res.value = (res.value as f64 / rhs.value as f64).floor() as Value;
                res
            }
            Operator::Nop() => RollOut {
                value: rhs.value,
                txt: Layouter::default(),
            },
        };

        // Add the text for the rhs
        out.txt = rhs.txt + out.txt;

        // Apply the sign
        if let Sign::Negative = self.sign {
            RollOut {
                value: -out.value,
                txt: Layouter::from(" - ") + out.txt,
            }
        } else {
            RollOut {
                value: out.value,
                txt: out.txt,
            }
        }
    }

    fn roll_quiet(&self) -> isize {
        let rhs = self.roll.roll_quiet();

        // Calculate absolute result
        let abs_result = match &self.op {
            Operator::Mul(expr) => rhs * expr.roll_quiet(),
            Operator::Div(expr) => (expr.roll_quiet() as f64 / rhs as f64).floor() as Value,
            Operator::Nop() => rhs,
        };

        // Apply sign
        match self.sign {
            Sign::Positive => abs_result,
            Sign::Negative => -abs_result,
        }
    }

    fn dist(&self) -> ProbDist {
        let mut dist = self.roll.dist();

        match &self.op {
            Operator::Mul(expr) => {
                dist = expr.dist() * &dist;
            }
            Operator::Div(expr) => {
                dist = expr.dist() / &dist;
            }
            Operator::Nop() => {}
        }

        match self.sign {
            Sign::Positive => dist,
            Sign::Negative => -dist,
        }
    }
}

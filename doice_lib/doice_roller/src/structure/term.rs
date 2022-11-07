use std::str::FromStr;

use crate::{
    layouter::Layouter, prob_dist::ProbDist, DiceError, Expression, RollOut, Rollable, Value,
};

use super::{literal::Literal, nop::Nothing};

#[derive(Clone, Debug)]
enum Sign {
    Positive,
    Negative,
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
        let mut src = src.trim();

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

        let op = if let Some(c) = src.chars().find(|c| "/*".contains(*c)) {
            let mut mul_terms = src.splitn(2, c);
            roll_txt = mul_terms.next().unwrap();
            let mul_txt = mul_terms.next().unwrap();
            match c {
                '*' => Operator::Mul(Box::new(mul_txt.parse()?)),
                _ => Operator::Div(Box::new(mul_txt.parse()?)),
            }
        } else {
            Operator::Nop()
        };

        Ok(Term {
            roll: dbg!(roll_txt).parse()?,
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
                res.txt.append("*");
                res.value *= rhs.value;
                res
            }
            Operator::Div(expr) => {
                let mut res = expr.roll();
                res.txt.append("/");
                res.value = (res.value as f64 / rhs.value as f64).floor() as Value;
                res
            }
            Operator::Nop() => RollOut {
                value: rhs.value,
                txt: Layouter::default(),
            },
        };

        // Add the text for the rhs
        out.txt = out.txt + rhs.txt;

        // Apply the sign
        match self.sign {
            Sign::Positive => RollOut {
                value: out.value,
                txt: Layouter::from(" + ") + out.txt,
            },
            Sign::Negative => RollOut {
                value: -out.value,
                txt: Layouter::from(" - ") + out.txt,
            },
        }
    }

    fn roll_quiet(&self) -> isize {
        let rhs = self.roll.roll_quiet();

        match &self.op {
            Operator::Mul(expr) => rhs * expr.roll_quiet(),
            Operator::Div(expr) => (expr.roll_quiet() as f64 / rhs as f64).floor() as Value,
            Operator::Nop() => rhs,
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

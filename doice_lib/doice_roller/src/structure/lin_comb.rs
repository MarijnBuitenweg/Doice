use std::str::FromStr;

use crate::{prob_dist::ProbDist, DiceError, Expression, RollOut, Rollable};

use super::term::Term;

/// Strips the input string of its whitespace, and rips out anything in parentheses
fn strip_parenth(src: &str) -> (String, Vec<String>) {
    let mut stripped: String = src.chars().filter(|c| !c.is_whitespace()).collect();
    //println!("Testing...");
    //println!("After first strip: {}", stripped);
    let mut in_parenth = false;
    let mut temp = String::new();
    let mut parenth_text = Vec::<String>::new();
    stripped = stripped
        .chars()
        .filter(|c| {
            if in_parenth {
                temp.push(*c);
            }
            let mut filter_out = in_parenth;
            match c {
                // If open parenth is detecte make sure the next characters will be ignored
                '(' => {
                    //println!("Found (");
                    in_parenth = true;
                }
                // If close parenth is detected, remove it from the buffer it was added to and make sure the next characters are no longer ignored
                ')' => {
                    //println!("Found )");
                    temp.pop();
                    parenth_text.push(temp.clone());
                    temp.clear();
                    filter_out = false;
                    in_parenth = false;
                }
                _ => {}
            }
            !filter_out
        })
        .collect();

    (stripped, parenth_text)
}

/// Fills the space between parentheses in terms with the given strings
fn fill_parentheses(terms: &mut [String], parenth: &[String]) {
    // Find all terms with parentheses
    let par_terms = terms.iter_mut().filter(|s| s.contains("()"));
    let mut par_text = parenth.iter();
    // Iterate through them with their corresponding parenth text
    for term in par_terms {
        *term = term
            .split_inclusive('(')
            .intersperse_with(|| par_text.next().unwrap())
            .flat_map(|s| s.chars())
            .collect();
    }
}

#[derive(Clone)]
pub struct LinComb {
    terms: Vec<Term>,
}

impl LinComb {
    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }
}

impl From<Expression> for LinComb {
    fn from(expr: Expression) -> Self {
        LinComb {
            terms: vec![expr.into()],
        }
    }
}

impl FromStr for LinComb {
    type Err = DiceError;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let (mut stripped, parenth) = strip_parenth(src);

        // Reverse string to make it so that the signs are included when splitting
        stripped = stripped.chars().rev().collect();
        // Split terms
        let terms = stripped.split_inclusive(&['+', '-'][..]);
        // Rereverse the terms
        let mut terms: Vec<String> = terms.map(|s| s.chars().rev().collect()).collect();
        // Refill parentheses
        fill_parentheses(&mut terms, &parenth);
        // Convert text to terms
        let terms = terms
            .iter()
            .map(|t| t.as_str().parse())
            .collect::<Result<_, _>>()?;
        Ok(LinComb { terms })
    }
}

impl Rollable for LinComb {
    fn roll(&self) -> RollOut {
        let mut out = RollOut::default();
        // Roll all terms
        for term in self.terms.iter().rev() {
            // And add their texts together
            let res = term.roll();
            out.value += res.value;
            out.txt = out.txt + res.txt;
        }
        // Remove initial plus or minus
        if !out.txt.sections.is_empty() {
            out.txt.remove(0);
        }
        out
    }

    fn dist(&self) -> ProbDist {
        let mut out = ProbDist::default();
        for term in self.terms.iter() {
            out = out + &term.dist();
        }
        out
    }

    fn roll_quiet(&self) -> isize {
        self.terms.iter().map(Rollable::roll_quiet).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::structure::lin_comb::fill_parentheses;

    use super::strip_parenth;

    #[test]
    fn strip_parenth_test() {
        let source = "aFunction(an expression) + some number + fun() + sin(2*pi)";
        let (stripped, parenth) = strip_parenth(source);
        assert_eq!(stripped, "aFunction()+somenumber+fun()+sin()");
        assert_eq!(parenth.len(), 3);
        assert_eq!(parenth[0], "anexpression");
        assert_eq!(parenth[1], "");
        assert_eq!(parenth[2], "2*pi");

        let mut stripped: [String; 1] = [stripped];
        fill_parentheses(&mut stripped, &parenth);
        assert_eq!(
            stripped[0],
            "aFunction(anexpression)+somenumber+fun()+sin(2*pi)"
        );
    }
}

use std::{convert::Infallible, error::Error, fmt::Display};

#[derive(Debug, Default, Clone)]
pub struct DiceError {
    desc: String,
}

impl From<String> for DiceError {
    fn from(str: String) -> Self {
        DiceError { desc: str }
    }
}

impl From<DiceError> for String {
    fn from(err: DiceError) -> Self {
        err.desc
    }
}

impl From<&str> for DiceError {
    fn from(input: &str) -> Self {
        input.to_string().into()
    }
}

impl Display for DiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for DiceError {}

impl From<Infallible> for DiceError {
    fn from(_: Infallible) -> Self {
        Default::default()
    }
}

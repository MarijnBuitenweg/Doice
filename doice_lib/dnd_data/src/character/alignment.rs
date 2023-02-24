use std::fmt::Display;

use super::field_interpret::CharacterParseError;

#[derive(Default)]
pub struct CharacterAlignment {
    pub entropy: Entropy,
    pub goodness: Goodness,
}

impl TryFrom<&str> for CharacterAlignment {
    type Error = CharacterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let words = value
            .split_once(char::is_whitespace)
            .ok_or("only a single word in alignment field")?;
        Ok(CharacterAlignment {
            entropy: words.0.try_into()?,
            goodness: words.1.try_into()?,
        })
    }
}

impl Display for CharacterAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.entropy == Entropy::Neutral && self.goodness == Goodness::Neutral {
            write!(f, "true neutral")
        } else {
            write!(f, "{} {}", self.entropy, self.goodness)
        }
    }
}

#[derive(PartialEq, Eq)]
#[derive(Default)]
pub enum Entropy {
    Chaotic,
    #[default]
    Neutral,
    Lawful,
}

impl Entropy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Entropy::Chaotic => "chaotic",
            Entropy::Neutral => "neutral",
            Entropy::Lawful => "lawful",
        }
    }
}



impl TryFrom<&str> for Entropy {
    type Error = CharacterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "chaotic" => Entropy::Chaotic,
            "neutral" => Entropy::Neutral,
            "lawful" => Entropy::Lawful,
            _ => return Err("invalid entropy in alignment".into()),
        })
    }
}

impl Display for Entropy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(PartialEq, Eq)]
#[derive(Default)]
pub enum Goodness {
    Evil,
    #[default]
    Neutral,
    Good,
}



impl TryFrom<&str> for Goodness {
    type Error = CharacterParseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "evil" => Goodness::Evil,
            "neutral" => Goodness::Neutral,
            "good" => Goodness::Good,
            _ => return Err("invalid goodness in alignment".into()),
        })
    }
}

impl Goodness {
    pub fn as_str(&self) -> &'static str {
        match self {
            Goodness::Evil => "evil",
            Goodness::Neutral => "neutral",
            Goodness::Good => "good",
        }
    }
}

impl Display for Goodness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

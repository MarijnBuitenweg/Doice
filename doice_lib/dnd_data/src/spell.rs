use include_dir::{include_dir, Dir};
#[cfg(feature = "rayon")]
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, ops::Deref};

use doice_utils::{Named, Search};

/// Should not be included in exe if flag is not set
#[cfg(feature = "include_data")]
pub static SPELL_SOURCE: Option<Dir> = Some(include_dir!("$CARGO_MANIFEST_DIR/src/data/spells"));
#[cfg(not(feature = "include_data"))]
pub static SPELL_SOURCE: Option<Dir> = None;

#[derive(Serialize, Deserialize)]
pub struct SpellTime {
    pub number: u8,
    pub unit: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpellRange {
    //type: String,
}

#[derive(Serialize, Deserialize)]
pub struct SpellDuration {
    //type: String,
}

#[derive(Serialize, Deserialize)]
pub struct ScalingLevelDice {
    pub label: String,
    pub scaling: BTreeMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct SpellClasses {
    pub fromClassList: Option<Vec<ClassId>>,
}

#[derive(Serialize, Deserialize)]
pub struct ClassId {
    pub name: String,
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct Spell {
    pub name: String,
    pub source: String,
    pub page: u32,
    pub level: u8,
    pub school: String,
    pub time: Vec<SpellTime>,
    pub range: SpellRange,
    pub components: BTreeMap<String, Value>,
    pub duration: Vec<SpellDuration>,
    pub entries: Vec<Value>,
    pub scalingLevelDice: Option<Value>,
    pub damageInflict: Option<Vec<String>>,
    pub savingThrow: Option<Vec<String>>,
    pub classes: SpellClasses,
}

impl Named for Spell {
    fn search_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpellBook {
    spell: Box<[Spell]>,
}

impl TryFrom<&str> for SpellBook {
    type Error = serde_json::Error;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(src)
    }
}

impl TryFrom<&[u8]> for SpellBook {
    type Error = serde_json::Error;

    fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
        serde_json::from_slice(src)
    }
}

impl Deref for SpellBook {
    type Target = Box<[Spell]>;

    fn deref(&self) -> &Self::Target {
        &self.spell
    }
}

#[derive(Serialize, Deserialize)]
pub struct Spells {
    books: Box<[SpellBook]>,
}

impl Spells {
    /// Loads spells from source in binary, should never fail
    pub fn from_source() -> Self {
        Self {
            books: if let Some(source) = &SPELL_SOURCE {
                source
                    .files()
                    .map(|f| {
                        println!("{}", f.path().display());
                        serde_json::from_slice(f.contents()).expect("Spell deserialization failed")
                    })
                    .collect()
            } else {
                Box::new([])
            },
        }
    }
}

impl<'col, 'item> Search<'col, 'item> for Spells {
    const MAX_SCORE: f64 = 100_000.0;

    type SearchBy = &'item str;

    type SearchFor = &'col Spell;

    fn calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.books
            .iter()
            .flat_map(|book| book.calculate_distances(name).into_iter())
            .collect()
    }
}

#[cfg(feature = "rayon")]
impl<'col, 'item> doice_utils::ParSearch<'col, 'item> for Spells {
    fn par_calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.books
            .par_iter()
            .map(|book| book.par_calculate_distances(name).into_par_iter())
            .flatten()
            .collect()
    }
}

#[cfg(not(feature = "rayon"))]
impl<'col, 'item> doice_utils::ParSearch<'col, 'item> for Spells {
    fn par_calculate_distances(&'col self, name: Self::SearchBy) -> Vec<(u32, Self::SearchFor)> {
        self.books
            .iter()
            .flat_map(|book| book.par_calculate_distances(name).into_iter())
            .collect()
    }
}

impl Deref for Spells {
    type Target = [SpellBook];

    fn deref(&self) -> &Self::Target {
        &self.books
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spell_deser_test() {
        let ts = std::time::Instant::now();
        let spells = Spells::from_source();
        if spells.is_empty() {
            panic!("No spells were deserialized!");
        }
        println!("Deserialized spells in: {:#?}", ts.elapsed()); // 40ms
    }
}

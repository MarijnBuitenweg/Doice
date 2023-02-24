use std::{collections::HashMap, fmt::Display, ops::{Deref, DerefMut}};

use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize)]
pub struct ProtoStats {
    str: u8,
    dex: u8,
    con: u8,
    int: u8,
    wis: u8,
    cha: u8,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Stats(HashMap<String, u8>);

impl Deref for Stats {
    type Target = HashMap<String, u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Stats {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}



impl From<HashMap<String, u8>> for Stats {
    fn from(map: HashMap<String, u8>) -> Self {
        Self(map)
    }
}

#[derive(Serialize, Deserialize)]
pub struct ProtoSkills {
    acrobatics: String,
    animal_handling: String,
    arcana: String,
    athletics: String,
    deception: String,
    history: String,
    insight: String,
    intimidation: String,
    investigation: String,
    medicine: String,
    nature: String,
    perception: String,
    performance: String,
    persuasion: String,
    religion: String,
    sleight_of_hand: String,
    stealth: String,
    survival: String,
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Skills(HashMap<String, Skill>);

impl Deref for Skills {
    type Target = HashMap<String, Skill>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Skills {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<HashMap<String, Skill>> for Skills {
    fn from(map: HashMap<String, Skill>) -> Self {
        Self(map)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
pub enum ProfLvl {
    #[default]
    Nothing = 0,
    Proficiency = 1,
    Expertise = 2,
}



impl Display for ProfLvl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PROF_TYPES: &[&str] = &["", "prof", "exp"];
        write!(f, "{}", PROF_TYPES[*self as usize])
    }
}

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct Skill {
    pub prof: ProfLvl,
    pub stat_name: String,
}

impl Skill {
    pub fn new<T: ToString>(prof: ProfLvl, stat_name: T) -> Self {
        Self {
            prof,
            stat_name: stat_name.to_string(),
        }
    }
}

use std::{convert::Infallible, error::Error, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::DnData;

use super::{
    state_variable::{svupdaters, StateVariable, UpdateMoment},
    stats_skills::{Skills, Stats},
    Character, ClassLvl,
};

/// This struct is going to be initialized by deser-ing YAML
/// The yaml fields are going to contain things that need further parsing
#[derive(Serialize, Deserialize)]
pub struct ProtoChar {
    pub(crate) name: String,
    pub(crate) lvls: String,
    pub(crate) alignment: String,
    pub(crate) speed: u32,
    pub(crate) hp: String,
    pub(crate) hit_dice: String,
    pub(crate) stats: Stats,
    pub(crate) skills: Skills,
}

impl ProtoChar {
    /// Tries to convert itself into a proper character struct
    pub fn construct<'a>(&self, data: &'a DnData) -> Result<Character<'a>, CharacterParseError> {
        let mut character = Default::default();
        int_classes(&mut character, &self.lvls, data)?;
        let total_lvl = character.levels.iter().map(|lvl| lvl.lvl).sum();
        int_alignment(&mut character, &self.alignment)?;
        int_hp(&mut character, &self.hp)?;
        //int_speed(&mut character, &self.speed)?;
        int_hitdice(&mut character, &self.hit_dice, total_lvl)?;
        Ok(character)
    }
}

#[derive(Debug, Default, Clone)]
pub struct CharacterParseError {
    desc: String,
}

impl From<String> for CharacterParseError {
    fn from(str: String) -> Self {
        CharacterParseError { desc: str }
    }
}

impl From<&str> for CharacterParseError {
    fn from(src: &str) -> Self {
        CharacterParseError { desc: src.into() }
    }
}

impl From<CharacterParseError> for String {
    fn from(err: CharacterParseError) -> Self {
        err.desc
    }
}

impl Display for CharacterParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl Error for CharacterParseError {}

impl From<Infallible> for CharacterParseError {
    fn from(_: Infallible) -> Self {
        Default::default()
    }
}

type FieldInterpreter =
    for<'a> fn(&mut Character<'a>, &str, &'a DnData) -> Result<(), CharacterParseError>;

fn int_classes<'a>(
    character: &mut Character<'a>,
    src: &str,
    data: &'a DnData,
) -> Result<(), CharacterParseError> {
    character.levels = src
        .split(',')
        .map(|txt| ClassLvl::from_txt(txt, data))
        .collect::<Result<Vec<ClassLvl>, CharacterParseError>>()?;
    Ok(())
}

fn int_alignment(character: &mut Character, src: &str) -> Result<(), CharacterParseError> {
    character.alignment = src.try_into()?;
    Ok(())
}

fn int_hp(character: &mut Character, src: &str) -> Result<(), CharacterParseError> {
    let (current, max) = src.split_once('/').ok_or("no maximum health was given")?;
    let sv = StateVariable::new("hp")
        .updateable_on(UpdateMoment::LR | UpdateMoment::MANUAL)
        .update_using(svupdaters::regain_all)
        .with_max_val(max.parse().or(Err("max health could not be parsed"))?)
        .with_init_val(
            current
                .parse()
                .or(Err("current health could not be parsed"))?,
        )
        .build();
    character.state.push(sv);
    Ok(())
}

fn int_speed(character: &mut Character, src: &str) -> Result<(), CharacterParseError> {
    character.speed = src
        .trim_end_matches('f')
        .parse()
        .or(Err("speed could not be parsed"))?;
    Ok(())
}

fn int_hitdice(
    character: &mut Character,
    src: &str,
    total_lvl: u8,
) -> Result<(), CharacterParseError> {
    let (current, _max) = src
        .split_once('/')
        .ok_or("no maximum number of hit dice was given")?;
    let sv = StateVariable::new("hitdice")
        .updateable_on(UpdateMoment::LR | UpdateMoment::MANUAL)
        .update_using(svupdaters::regain_half)
        .with_max_val(total_lvl as u32)
        .with_init_val(
            current
                .parse()
                .or(Err("current hitdice could not be parsed"))?,
        )
        .build();
    character.state.push(sv);
    Ok(())
}

use std::{fmt::Display};

use {
    doice_utils::{Named, ParSearch, Search},
};

use self::{
    alignment::CharacterAlignment,
    field_interpret::{CharacterParseError, ProtoChar},
    inventory::InventoryItem,
    state_variable::{UpdateMoment},
};

use super::{
    class::{Class, SubclassData},
    DnData,
};

mod alignment;
mod field_interpret;
mod inventory;
mod state_variable;
pub use state_variable::StateVariable;
mod stats_skills;
pub use stats_skills::{Skills, Skill, Stats, ProfLvl};

pub struct ClassLvl<'a> {
    pub lvl: u8,
    pub class: &'a Class,
    pub subclass: &'a SubclassData,
}

impl<'a> ClassLvl<'a> {
    /// Input should be formatted like
    /// ```
    /// assert_eq!(ClassLvL::from_txt("Great Old One Warlock 3").is_err(), false);
    /// ```
    pub fn from_txt(src: &str, data: &'a DnData) -> Result<ClassLvl<'a>, CharacterParseError> {
        let mut words_rev = src.split_whitespace().rev();

        // Extract text for specific fields
        let lvl_txt = words_rev.next().ok_or("no class specified")?;
        let class_txt = words_rev.next().ok_or("no class specified")?;
        // Make whatever's left be the subclass, by rereversing the word order and introducing whitespace
        let subclass_txt: String =
            itertools::Itertools::intersperse(words_rev.rev(), " ").collect();

        // Obtain references to the correct class and subclass
        let class = data
            .classes
            .par_find_closest_match(class_txt)
            .ok_or("no class was found")?
            .1;
        let subclass = class
            .subclass
            .par_find_closest_match(&subclass_txt)
            .ok_or("subclass could not be found")?
            .1;
        let lvl = lvl_txt
            .parse()
            .or(Err("class lvl number could not be parsed"))?;

        // Construct the output
        Ok(ClassLvl {
            lvl,
            class,
            subclass,
        })
    }
}

impl Display for ClassLvl<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.subclass.name,
            self.class.search_name(),
            self.lvl
        )
    }
}

/// Originally the plan was to make the file format for this easily human readable and editable by using a custom data format
/// However, this takes way too much time to implement
/// So we're gonna use an off-the-shelf format, in this case YAML
#[derive(Default)]
pub struct Character<'a> {
    // General stuff
    /// The character's name
    pub name: String,
    /// What class levels the character has
    pub levels: Vec<ClassLvl<'a>>,
    /// Proficiency bonus
    pub prof_bonus: u8,
    /// A character's height in ft
    pub height: Option<f64>,
    /// Contains state variables such as hp, hit dice, and more
    pub state: Vec<StateVariable>,
    /// Hit die type
    pub hit_die: u8,
    /// Alignment
    pub alignment: CharacterAlignment,
    /// Inventory
    pub inventory: Vec<InventoryItem<'a>>,
    /// S P O O D [ft/round]
    pub speed: u32,
    /// Stats
    pub stats: Stats,
    /// Skills
    pub skills: Skills,
}

impl<'a> Character<'a> {
    pub fn from_txt(src: &str, data: &'a DnData) -> Result<Character<'a>, CharacterParseError> {
        let proto: ProtoChar =
            serde_yaml::from_str(src).or(Err("YAML parsing failed while parsing character"))?;

        proto.construct(data)
    }

    #[cfg(feature = "include_data")]
    pub fn init_test(data: &'a DnData) -> Self {
        use std::collections::HashMap;

        use self::state_variable::svupdaters;

        Character {
            name: "Klaas".into(),
            levels: vec![ClassLvl::from_txt("light cleric 6", data).unwrap()],
            prof_bonus: 3,
            height: None,
            state: vec![StateVariable::new("hp")
                .with_max_val(45)
                .with_init_val(34)
                .update_using(svupdaters::regain_all)
                .updateable_on(UpdateMoment::LR)
                .build()],
            hit_die: 8,
            alignment: CharacterAlignment::try_from("lawful good").unwrap(),
            inventory: Vec::new(),
            speed: 30,
            stats: HashMap::from([
                ("STR".to_string(), 8),
                ("DEX".to_string(), 12),
                ("CON".to_string(), 14),
                ("INT".to_string(), 16),
                ("WIS".to_string(), 18),
                ("CHA".to_string(), 20),
            ])
            .into(),
            skills: HashMap::from([
                (
                    "Actrobatics".to_string(),
                    Skill::new(ProfLvl::Nothing, "DEX"),
                ),
                (
                    "Athletics".to_string(),
                    Skill::new(ProfLvl::Proficiency, "STR"),
                ),
                (
                    "Intimidation".to_string(),
                    Skill::new(ProfLvl::Expertise, "CHA"),
                ),
            ])
            .into(),
        }
    }

    #[cfg(not(feature = "include_data"))]
    pub fn init_test(_data: &'a DnData) -> Self {
        Default::default()
    }

    pub fn proto(&self) -> ProtoChar {
        let hp = self.state.find_closest_match("hp").unwrap().1;

        ProtoChar {
            name: self.name.clone(),
            lvls: self.levels.iter().map(|lvl| lvl.to_string()).collect(),
            alignment: self.alignment.to_string(),
            speed: self.speed,
            hp: format!("{}/{}", hp.current(), hp.max()),
            hit_dice: "testing".into(),
            stats: self.stats.clone(),
            skills: self.skills.clone(),
        }
    }

    pub fn short_rest(&mut self) {
        for sv in self.state.iter_mut() {
            sv.update(UpdateMoment::SR);
        }
    }

    pub fn long_rest(&mut self) {
        for sv in self.state.iter_mut() {
            sv.update(UpdateMoment::LR);
        }
    }
}

impl Display for Character<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_yaml::to_string(&self.proto()).unwrap())
    }
}

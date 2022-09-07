use crate::dice_rolls_legacy::Roller;

pub const STATNAMES: [&'static str; 6] = ["Strength", "Dexterity", "Constitution", "Intelligence", "Wisdom", "Charisma"];
pub const SKILLNAMES:[&'static str; 18]  = ["Acrobatics", "Animal handling", "Arcana", "Athletics", "Deception", "History", "Insight", "Intimidation", "Investigation", "Medicine", "Nature", "Perception", "Performance", "Persuasion", "Religion", "Sleight of hand", "Stealth", "Survival"];
pub const CLASSNAMES:[&'static str; 13] = ["Artificer", "Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
pub const GENERALNAMES:[&'static str; 8] = ["Lvl", "HP", "THP", "Max HP", "Speed", "Initiative", "Passive Perception", "Armor Class"];
pub const STATUSNAMES:[&'static str; 1] = ["Exhaustion"];
const PROFMASK: usize = 1 << 1usize.leading_zeros();
const EXPMASK: usize = 1 << (1usize.leading_zeros() - 1);
const STATMASK: usize = usize::MAX >> 2;

pub struct Spell {
    lvl: usize,
    shots: usize,
    atk_roll: Option<Roller>,
    dmg_roll: Option<Roller>,
    note: String,
}

pub struct DndCharacter {
    genStats: [Option<usize>; GENERALNAMES.len()],
    status: [usize; STATUSNAMES.len()],
    stats: [Option<usize>; STATNAMES.len()],
    saves: [Option<usize>; STATNAMES.len()],
    skills: [Option<usize>; SKILLNAMES.len()],
    slots: [usize; 9],
    cantrips: Vec<Spell>,
    spells: Vec<Spell>,
}

impl DndCharacter {
    
}
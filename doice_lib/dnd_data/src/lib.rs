#![allow(dead_code)]

#[allow(non_snake_case)]
mod feat;

use egui::Ui;
use feat::OptFeats;
#[allow(non_snake_case)]
pub mod spell;
use serde_json::Value;
pub use spell::{Spells, Spell};
#[allow(non_snake_case)]
mod item;
pub use item::BaseItems;
#[allow(non_snake_case)]
mod class;
pub use class::Classes;

pub mod character;

pub struct DnData {
    pub spells: Spells,
    pub base_items: BaseItems,
    pub opt_feats: OptFeats,
    pub classes: Classes,
}

impl DnData {
    pub fn new() -> Self {
        Self {
            spells: Spells::from_source(),
            base_items: BaseItems::from_source(),
            opt_feats: OptFeats::from_source(),
            classes: Classes::from_source(),
        }
    }
}

impl Default for DnData {
    fn default() -> Self {
        Self::new()
    }
}

pub fn display_entry(ui: &mut Ui, val: Value) {
    match val {
        Value::String(txt) => {
            ui.label(txt);
        },
        Value::Object(obj) => match obj.get("type") {
            Some(Value::String(e_type_txt)) if e_type_txt == "list" => {
                if let Some(Value::Array(items)) = obj.get("items") {
                    for item in items.iter() {
                        if let Value::String(txt) = item {
                            ui.label(format!("⏺\t{}", txt));
                        }
                    }
                }
            }
            _ => {}
        },
        _ => {}
    }
}

pub trait CharacterFeature {}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn main_deser_test() {
        let ts = Instant::now();
        let _data = DnData::new();
        println!("It took {:#?} to deserialize all the data", ts.elapsed());
    }
}

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use doice_utils::Named;

#[cfg(feature = "include_data")]
pub static BASEITEMS_SOURCE: Option<&str> = Some(include_str!("data/items-base.json"));
#[cfg(not(feature = "include_data"))]
pub static BASEITEMS_SOURCE: Option<&str> = None;

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub source: String,
    pub page: u32,
    pub srd: Option<bool>,
    pub basicrules: Option<bool>,
    #[serde(rename = "type")]
    pub item_type: String,
    pub rarity: String,
    pub weight: Option<f32>,
    pub value: Option<f32>,
    pub ac: Option<u8>,
    pub armor: Option<bool>,
    pub weapon: Option<bool>,
    pub firearm: Option<bool>,
    pub axe: Option<bool>,
    #[serde(rename = "stealth")]
    pub stealth_disadv: Option<bool>,
    pub entries: Option<Vec<Value>>,
    pub weaponCategory: Option<String>,
    pub property: Option<Vec<String>>,
    pub dmg1: Option<String>,
    pub dmgType: Option<String>,
    pub ammoType: Option<String>,
}

impl Named for Item {
    fn search_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct BaseItems {
    baseitem: Box<[Item]>,
}

impl BaseItems {
    pub fn from_source() -> Self {
        if let Some(source) = &BASEITEMS_SOURCE {
            Self::try_from(*source).expect("Error during deserialization of base items")
        } else {
            BaseItems {
                baseitem: Box::new([]),
            }
        }
    }
}

impl TryFrom<&str> for BaseItems {
    type Error = serde_json::Error;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(src)
    }
}

impl Deref for BaseItems {
    type Target = [Item];

    fn deref(&self) -> &Self::Target {
        &self.baseitem
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;

    #[test]
    fn base_items_deser_test() {
        let ts = Instant::now();
        let base_items = BaseItems::from_source();
        if base_items.is_empty() {
            panic!("No base items were deserialized");
        }
        println!("Deserialized base items in {:#?}", ts.elapsed()); // 2.3ms
    }

    #[test]
    fn base_items_search_test() {
        let _items = BaseItems::from_source();
    }
}

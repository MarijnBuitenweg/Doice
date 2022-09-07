use std::ops::Deref;

use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use doice_utils::Named;

#[cfg(feature = "include_data")]
pub static CLASS_SOURCE: Option<Dir> = Some(include_dir!("$CARGO_MANIFEST_DIR/src/data/class"));
#[cfg(not(feature = "include_data"))]
pub static CLASS_SOURCE: Option<Dir> = None;

#[derive(Serialize, Deserialize)]
pub struct Class {
    pub class: Vec<ClassData>,
    pub subclass: Vec<SubclassData>,
    pub classFeature: Vec<ClassFeature>,
    pub subclassFeature: Vec<SubclassFeature>,
}

impl Named for Class {
    fn search_name(&self) -> &str {
        self.class[0].search_name()
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassData {
    pub name: String,
    pub source: String,
}

impl Named for ClassData {
    fn search_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubclassData {
    pub name: String,
    pub shortName: String,
    pub source: String,
    pub page: Option<u32>,
}

impl Named for SubclassData {
    fn search_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct ClassFeature {
    pub name: String,
    pub source: String,
    pub page: u32,
    pub level: u8,
    pub entries: Vec<Value>,
}

impl Named for ClassFeature {
    fn search_name(&self) -> &str {
        &self.name
    }
}

#[derive(Serialize, Deserialize)]
pub struct SubclassFeature {
    pub name: String,
    pub source: String,
    pub subclassShortName: String,
    pub page: u32,
    pub level: u8,
    pub entries: Vec<Value>,
}

pub struct Classes {
    classes: Box<[Class]>,
}

impl Classes {
    pub fn from_source() -> Self {
        Self {
            classes: if let Some(source) = &CLASS_SOURCE {
                source
                    .files()
                    .map(|f| {
                        println!("{:#?}", f.path());
                        serde_json::from_slice(f.contents()).expect("Deser of file failed")
                    })
                    .collect()
            } else {
                Box::default()
            }
        }
    }
}

impl Deref for Classes {
    type Target = [Class];
    
    fn deref(&self) -> &Self::Target {
        &self.classes
    }
}

#[cfg(test)]
mod tests {
    pub static FIGHTER_SOURCE: &[u8] = include_bytes!("data/class/class-fighter.json");
    use super::*;
    
    #[test]
    fn class_deser_test() {
        let _data = Classes::from_source();
    }
    
    #[test]
    fn fighter_deser_test() {
        let _data: Class = serde_json::from_slice(FIGHTER_SOURCE).expect("fighters deser failed");
    }
}

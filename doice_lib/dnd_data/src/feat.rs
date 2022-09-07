use std::ops::Deref;

use serde::{Deserialize, Serialize};

#[cfg(feature = "include_data")]
pub static OPTIONAL_FEATS_SOURCE: Option<&str> = Some(include_str!("data/optionalfeatures.json"));
#[cfg(not(feature = "include_data"))]
pub static OPTIONAL_FEATS_SOURCE: Option<&str> = None;

/// Contains all data of an optional feature
#[derive(Serialize, Deserialize)]
pub struct OptFeat {
    pub name: String,
    pub source: String,
    pub page: u32,
    pub featureType: Vec<String>,
    pub isClassFeatureVariant: Option<bool>,
    pub entries: Vec<serde_json::Value>,
}

/// Contains all data concerning optional features
#[derive(Serialize, Deserialize)]
pub struct OptFeats {
    optionalfeature: Box<[OptFeat]>,
}

impl OptFeats {
    pub fn from_source() -> Self {
        if let Some(source) = &OPTIONAL_FEATS_SOURCE {
            Self::try_from(*source).expect("Optional feat deserialization failed!")
        } else {
            OptFeats {
                optionalfeature: Box::default(),
            }
        }
    }
}

impl TryFrom<&str> for OptFeats {
    type Error = serde_json::Error;

    fn try_from(src: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(src)
    }
}

/// Makes it possible to simply read the contents
impl Deref for OptFeats {
    type Target = [OptFeat];

    fn deref(&self) -> &Self::Target {
        &self.optionalfeature
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_optfeats() {
        let ts = std::time::Instant::now();
        let opt_feats: OptFeats =
            serde_json::from_str(OPTIONAL_FEATS_SOURCE.unwrap()).expect("Data could not be interpreted!");
        if opt_feats.optionalfeature.is_empty() {
            panic!("Opt feats is empty!");
        }
        println!("Deserialized optfeats in: {:#?}", ts.elapsed()); // 4.6ms
    }
}

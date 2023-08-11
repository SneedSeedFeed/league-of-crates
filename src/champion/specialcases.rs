use crate::champion::specialcases::SpecialCaseError::{FileError, ParseError};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

enum Bonuses {
    movespeed(f32),
    attackspeed(f32),
}
enum Stats {
    movespeedperlevel(f32),
    attackrangeperlevel(f32),
    attackspeedratio(f32),
}

pub struct SpecialCases {
    special_case_champs: Vec<String>,
    adjustments: HashMap<String, ChampAdjustment>,
}

pub enum SpecialCaseError {
    FileError,
    ParseError(Option<serde_json::Error>),
}

impl SpecialCases {
    pub fn new(filename: &Path) -> Result<Self, SpecialCaseError> {
        let data = fs::read_to_string(filename).map_err(|_| FileError)?;
        let json: Value = serde_json::from_str(&data).map_err(|err| ParseError(Some(err)))?;

        SpecialCases::new_from_value(json)
    }

    pub fn new_from_value(json: Value) -> Result<Self, SpecialCaseError> {
        todo!()
    }
}

struct ChampAdjustment {
    bonuses: Option<Vec<Bonuses>>,
    stats: Option<Vec<Stats>>,
}

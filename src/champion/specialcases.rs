use crate::champion::specialcases::SpecialCaseError::{FileError, ParseError};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

pub enum Bonus {
    //just used for cassio having 4 MS on passive at level 1
    //might need more in future idk
    movespeed(f32),
}

pub enum Stats {
    movespeedperlevel(f32),
    attackrangeperlevel(f32),
    attackspeedratio(f32),
    attackspeedbonusat1(f32),
}

pub struct ChampAdjustment {
    pub bonus: Option<Vec<Bonus>>,
    pub stats: Option<Vec<Stats>>,
}

pub struct SpecialCases {
    pub special_case_champs: Vec<String>,
    pub adjustments: HashMap<String, ChampAdjustment>,
}

pub enum SpecialCaseError {
    FileError,
    ParseError(Option<serde_json::Error>),
}

impl SpecialCases {
    pub fn new_from_value(json: Value) -> Result<Self, SpecialCaseError> {
        if json.as_object().is_none() {
            return Err(ParseError(None));
        }
        let raw: &serde_json::Map<String, Value> = json.as_object().unwrap();

        if raw.get("special_case_champs").is_none() || raw.get("special_cases").is_none() {
            return Err(ParseError(None));
        } else if !raw.get("special_case_champs").unwrap().is_array()
            || raw.get("special_cases").unwrap().as_object().is_none()
        {
            return Err(ParseError(None));
        }

        let mut champ_list: Vec<String> = Vec::new();
        for champ in raw.get("special_case_champs").unwrap().as_array().unwrap() {
            match champ {
                Value::String(x) => champ_list.push(x.to_string()),
                _ => return Err(ParseError(None)),
            }
        }

        let mut champ_map: HashMap<String, ChampAdjustment> = HashMap::new();
        for champ in raw
            .get("special_cases")
            .unwrap()
            .as_object()
            .unwrap()
            .iter()
        {
            champ_map.insert(
                champ.0.to_string(),
                SpecialCases::parse_adjustment_val(champ.1)?,
            );
        }

        Ok(SpecialCases {
            special_case_champs: champ_list,
            adjustments: champ_map,
        })
    }

    fn parse_adjustment_val(json: &Value) -> Result<ChampAdjustment, SpecialCaseError> {
        let mut stats: Option<Vec<Stats>> = None;
        let mut bonus: Option<Vec<Bonus>> = None;

        if let Some(x) = json.get("stats") {
            match x {
                Value::Object(y) => {
                    stats = SpecialCases::parse_stats(y);
                }
                _ => return Err(ParseError(None)),
            }
        };
        if let Some(x) = json.get("bonus") {
            match x {
                Value::Object(y) => {
                    bonus = SpecialCases::parse_bonus(y);
                }
                _ => return Err(ParseError(None)),
            }
        };
        Ok(ChampAdjustment { bonus, stats })
    }

    //These functions suck
    fn parse_bonus(json: &serde_json::Map<String, Value>) -> Option<Vec<Bonus>> {
        let mut vec: Vec<Bonus> = Vec::new();
        for (str, val) in json {
            if str == "movespeed" {
                vec.push(Bonus::movespeed(
                    val.as_f64().expect("Bonus movespeed f64") as f32
                ))
            }
        }
        if vec.len() > 0 {
            Some(vec)
        } else {
            None
        }
    }

    fn parse_stats(json: &serde_json::Map<String, Value>) -> Option<Vec<Stats>> {
        let mut vec: Vec<Stats> = Vec::new();
        for (str, val) in json {
            if str == "movespeedperlevel" {
                vec.push(Stats::movespeedperlevel(
                    val.as_f64().expect("movespeedperlevel not f64") as f32,
                ))
            } else if str == "attackrangeperlevel" {
                vec.push(Stats::attackrangeperlevel(
                    val.as_f64().expect("attackrangeperlevel not f64") as f32,
                ))
            } else if str == "attackspeedratio" {
                vec.push(Stats::attackspeedratio(
                    val.as_f64().expect("attackspeedratio not f64") as f32,
                ))
            } else if str == "attackspeedbonusat1" {
                vec.push(Stats::attackspeedbonusat1(
                    val.as_f64().expect("attackspeedbonusat1 not f64") as f32,
                ))
            }
        }
        if vec.len() > 0 {
            Some(vec)
        } else {
            None
        }
    }
}

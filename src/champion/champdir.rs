use crate::champion::champdir::ChampCreationError::{FileError, ParseError};
use crate::champion::champion::{ChampInfo, Champion};
use crate::champion::champstats::ChampStats;
use crate::champion::specialcases::SpecialCases;
use crate::champion::specialcases::SpecialCaseError;
use serde::Deserialize;
use serde_json::{from_value, Map, Value};
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
pub struct RawChampStats {
    pub hp: f32,
    pub hpperlevel: f32,
    pub mp: f32,
    pub mpperlevel: f32,
    pub movespeed: f32,
    pub armor: f32,
    pub armorperlevel: f32,
    pub spellblock: f32,
    pub spellblockperlevel: f32,
    pub attackrange: f32,
    pub hpregen: f32,
    pub hpregenperlevel: f32,
    pub mpregen: f32,
    pub mpregenperlevel: f32,
    pub crit: f32,
    pub critperlevel: f32,
    pub attackdamage: f32,
    pub attackdamageperlevel: f32,
    pub attackspeedperlevel: f32,
    pub attackspeed: f32,
}

impl Into<ChampStats> for RawChampStats {
    fn into(self) -> ChampStats {
        ChampStats {
            hp: self.hp,
            hpperlevel: self.hpperlevel,
            mp: self.mp,
            mpperlevel: self.mpperlevel,
            movespeed: self.movespeed,
            movespeedperlevel: 0.0,
            armor: self.armor,
            armorperlevel: self.armorperlevel,
            spellblock: self.spellblock,
            spellblockperlevel: self.spellblockperlevel,
            attackrange: self.attackrange,
            attackrangeperlevel: 0.0,
            hpregen: self.hpregen,
            hpregenperlevel: self.hpregenperlevel,
            mpregen: self.mpregen,
            mpregenperlevel: self.mpregenperlevel,
            crit: self.crit,
            critperlevel: self.critperlevel,
            attackdamage: self.attackdamage,
            attackdamageperlevel: self.attackdamageperlevel,
            attackspeedperlevel: self.attackspeedperlevel,
            attackspeed: self.attackspeed,
            attackspeedratio: self.attackspeed,
            attackspeedbonusat1: 0.0,
        }
    }
}

#[derive(Deserialize)]
struct RawChampion {
    id: String,
    key: String,
    name: String,
    title: String,
    blurb: String,
    info: ChampInfo,
    tags: Vec<String>,
    partype: String,
    stats: RawChampStats,
}

impl Into<Champion> for RawChampion {
    fn into(self) -> Champion {
        Champion {
            id: self.id,
            // This panics, cry about it
            key: self.key.parse().unwrap(),
            name: self.name,
            title: self.title,
            blurb: self.blurb,
            info: self.info,
            tags: self.tags,
            partype: self.partype,
            stats: self.stats.into(),
        }
    }
}

pub struct ChampDir {
    pub champions: Vec<Champion>,
}

#[derive(Debug)]
pub enum ChampCreationError {
    FileError,
    ParseError(Option<serde_json::Error>),
    SpecialDataError(String)
}

impl ChampDir {
    pub fn new(filename: &Path, special_case_file: &Path) -> Result<Self, ChampCreationError> {
        let data = fs::read_to_string(filename).map_err(|_| FileError)?;
        let json: Value = serde_json::from_str(&data).map_err(|err| ParseError(Some(err)))?;

        let special_case_data = fs::read_to_string(special_case_file).map_err(|_| FileError)?;
        let special_case_json: Value = serde_json::from_str(&special_case_data).map_err(|err| ParseError(Some(err)))?;

        ChampDir::new_from_value(json,special_case_json)
    }

    pub fn new_from_value(champ_json: Value, special_case_json:Value) -> Result<Self, ChampCreationError> {
        let corrections = SpecialCases::new_from_value(special_case_json).map_err(|x| match x{
            SpecialCaseError::FileError => FileError,
            SpecialCaseError::ParseError(err) => ParseError(err)
        })?;


        return match &champ_json["data"] {
            Value::Object(x) => {
                let x = ChampDir::parse_champs(x)?;
                Ok(ChampDir { champions: ChampDir::process_and_correct(x, corrections)? })
            }
            _ => Err(ParseError(None)),
        };
    }

    pub fn get_by_key(&self, key: i16) -> Option<&Champion> {
        self.champions.iter().find(|x| x.key == key)
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Champion> {
        self.champions.iter().find(|x| x.name == name)
    }

    fn parse_champs(champs: &Map<String, Value>) -> Result<Vec<RawChampion>, ChampCreationError> {
        let mut champ_vec: Vec<RawChampion> = Vec::new();
        for val in champs.values() {
            champ_vec.push(ChampDir::parse_champ(val)?);
        }
        Ok(champ_vec)
    }

    fn parse_champ(champ: &Value) -> Result<RawChampion, ChampCreationError> {
        Ok(from_value::<RawChampion>(champ.clone())
            .map_err(|err| ParseError(Some(err)))?)
    }

    fn process_and_correct(raw_champs: Vec<RawChampion>, corrections:SpecialCases) -> Result<Vec<Champion>, ChampCreationError>{
        todo!()
    }
}

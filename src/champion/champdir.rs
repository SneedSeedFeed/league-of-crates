use crate::champion::champdir::ChampCreationError::{FileError,ParseError};
use crate::champion::champdir::ChampStatError::{LevelRangeError, ZeroError};
use serde::Deserialize;
use serde_json::{from_value, Map, Value};
use std::fs;
use std::path::Path;

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

pub struct Champion {
    pub id: String,
    pub key: i16,
    pub name: String,
    pub title: String,
    pub blurb: String,
    pub info: ChampInfo,
    pub tags: Vec<String>,
    pub partype: String,
    pub stats: ChampStats,
}

#[derive(Debug)]
pub enum ChampStatError {
    ZeroError,
    LevelRangeError,
}

pub struct EffectiveHealth {
    physical: f32,
    magical: f32,
}

impl Champion {
    pub fn get_stats_level(&self, level: u32) -> Result<LevelStats, ChampStatError> {
        self.stats.get_stats_level(level)
    }

    pub fn get_effective_hp_at_level(
        &self,
        level: u32,
    ) -> Result<EffectiveHealth, ChampStatError> {
        Ok(self.get_stats_level(level)?.get_effective_health())
    }

    pub fn get_effective_hp_range(
        &self,
        min_level: u32,
        max_level: u32,
    ) -> Result<Vec<EffectiveHealth>, ChampStatError> {
        Ok(self
            .stats
            .get_stats_range(min_level, max_level)?
            .iter()
            .map(|x| x.get_effective_health())
            .collect())
    }
}

pub struct LevelStats {
    pub hp: f32,
    pub mp: f32,
    pub movespeed: f32,
    pub armor: f32,
    pub spellblock: f32,
    pub attackrange: f32,
    pub hpregen: f32,
    pub mpregen: f32,
    pub crit: f32,
    pub attackdamage: f32,
    pub attackspeed: f32,
}

impl LevelStats {
    pub fn get_effective_health(&self) -> EffectiveHealth {
        EffectiveHealth {
            physical: self.hp * (1f32 + 0.01 * self.armor),
            magical: self.hp * (1f32 + 0.01 * self.spellblock),
        }
    }
}

#[derive(Deserialize)]
pub struct ChampInfo {
    pub attack: i8,
    pub defense: i8,
    pub magic: i8,
    pub difficulty: i8,
}

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
        }
    }
}

pub struct ChampStats {
    pub hp: f32,
    pub hpperlevel: f32,
    pub mp: f32,
    pub mpperlevel: f32,
    pub movespeed: f32,
    pub movespeedperlevel: f32,
    pub armor: f32,
    pub armorperlevel: f32,
    pub spellblock: f32,
    pub spellblockperlevel: f32,
    pub attackrange: f32,
    pub attackrangeperlevel: f32,
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
    pub attackspeedratio: f32,
}

impl ChampStats {
    pub fn get_stats_level(&self, level: u32) -> Result<LevelStats, ChampStatError> {
        if level == 0 {
            return Err(ZeroError);
        }

        Ok(LevelStats {
            hp: self.hp + self.hpperlevel * (level - 1) as f32,
            mp: self.mp + self.mpperlevel * (level - 1) as f32,
            movespeed: self.movespeed,
            armor: self.armor + self.armorperlevel * (level - 1) as f32,
            spellblock: self.spellblock + self.spellblockperlevel * (level - 1) as f32,
            attackrange: self.attackrange,
            hpregen: self.hpregen + self.hpregenperlevel * (level - 1) as f32,
            mpregen: self.mpregen + self.mpregenperlevel * (level - 1) as f32,
            crit: self.crit + self.critperlevel * (level - 1) as f32,
            attackdamage: self.attackdamage + self.attackdamageperlevel * (level - 1) as f32,
            attackspeed: self.attackspeed
                * (1f32 + (self.attackspeedperlevel * (level - 1) as f32) / 100f32),
        })
    }

    pub fn get_stats_range(
        &self,
        start: u32,
        stop: u32,
    ) -> Result<Vec<LevelStats>, ChampStatError> {
        if start >= stop {
            return Err(LevelRangeError);
        }
        if start == 0 {
            return Err(ZeroError);
        }
        Ok((start..stop)
            .into_iter()
            .map(|i| self.get_stats_level(i).unwrap())
            .collect())
    }
}

pub struct ChampDir {
    pub champions: Vec<Champion>,
}

#[derive(Debug)]
pub enum ChampCreationError {
    FileError,
    ParseError(Option<serde_json::Error>),
}

impl ChampDir {
    pub fn new(filename: &Path) -> Result<Self, ChampCreationError> {
        let data = fs::read_to_string(filename).map_err(|_| FileError)?;
        let json: Value = serde_json::from_str(&data).map_err(|err| ParseError(Some(err)))?;

        ChampDir::new_from_value(json)
    }

    pub fn new_from_value(json: Value) -> Result<Self, ChampCreationError> {
        return match &json["data"] {
            Value::Object(x) => {
                let x = ChampDir::parse_champs(x)?;
                Ok(ChampDir { champions: x })
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

    fn parse_champs(champs: &Map<String, Value>) -> Result<Vec<Champion>, ChampCreationError> {
        let mut champ_vec: Vec<Champion> = Vec::new();

        for val in champs.values() {
            champ_vec.push(ChampDir::parse_champ(val)?);
        }
        Ok(champ_vec)
    }

    fn parse_champ(champ: &Value) -> Result<Champion, ChampCreationError> {
        Ok(from_value::<RawChampion>(champ.clone())
            .map_err(|err| ParseError(Some(err)))?
            .into())
    }
}
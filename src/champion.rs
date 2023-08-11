pub mod champion {
    use crate::champion::champion::ChampCreationError::{FileError, ParseError};
    use serde::Deserialize;
    use serde_json::{from_value, Map, Value};
    use std::fs;
    use std::path::Path;
    use crate::champion::champion::ChampStatError::{ZeroError};

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
        stats: ChampStats,
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
                stats: self.stats,
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
        ZeroError
    }

    impl Champion {
        pub fn get_stats_level(&self, level:u32) -> Result<CalcStats, ChampStatError>{
            self.stats.get_stats_level(level)
        }

    }

    pub struct CalcStats {
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

    #[derive(Deserialize)]
    pub struct ChampInfo {
        pub attack: i8,
        pub defense: i8,
        pub magic: i8,
        pub difficulty: i8,
    }

    #[derive(Deserialize)]
    pub struct ChampStats {
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

    impl ChampStats{
        fn get_stats_level(&self, level: u32) -> Result<CalcStats, ChampStatError> {
            if level == 0 { return Err(ZeroError); }

            Ok(CalcStats {
                hp: self.hp + self.hpperlevel * (level-1) as f32,
                mp: self.mp + self.mpperlevel * (level-1) as f32,
                movespeed: self.movespeed,
                armor: self.armor + self.armorperlevel * (level-1) as f32,
                spellblock: self.spellblock + self.spellblockperlevel * (level-1) as f32,
                attackrange: self.attackrange,
                hpregen: self.hpregen + self.hpregenperlevel * (level-1) as f32,
                mpregen: self.mpregen + self.mpregenperlevel * (level-1) as f32,
                crit: self.crit + self.critperlevel * (level-1) as f32,
                attackdamage: self.attackdamage + self.attackdamageperlevel * (level-1) as f32,
                attackspeed: self.attackspeed * (1f32 + (self.attackspeedperlevel * (level-1)as f32)/100f32),
            })
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

            return match &json["data"] {
                Value::Object(x) => {
                    let x = ChampDir::parse_champs(x)?;
                    Ok(ChampDir { champions: x })
                }
                _ => Err(ParseError(None)),
            };
        }

        pub fn get_by_key(&self, key: i16) -> Option<&Champion>{
            self.champions.iter().find(|x| x.key == key)
        }

        pub fn get_by_name(&self, name:&str) -> Option<&Champion>{
            self.champions.iter().find(|x| x.name==name)
        }

        fn parse_champs(champs: &Map<String, Value>) -> Result<Vec<Champion>, ChampCreationError> {
            let mut champ_vec: Vec<Champion> = Vec::new();

            for val in champs.values() {
                champ_vec.push(ChampDir::parse_champ(val)?);
            }
            Ok(champ_vec)
        }

        fn parse_champ(champ: &Value) -> Result<Champion, ChampCreationError> {
            Ok(from_value::<RawChampion>(champ.clone()).map_err(|err| ParseError(Some(err)))?.into())
        }
    }
}

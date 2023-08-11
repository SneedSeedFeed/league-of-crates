use crate::champion::champstats::{ChampStatError, ChampStats, EffectiveHealth, LevelStats};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ChampInfo {
    pub attack: i8,
    pub defense: i8,
    pub magic: i8,
    pub difficulty: i8,
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

impl Champion {
    pub fn get_stats_level(&self, level: u32) -> Result<LevelStats, ChampStatError> {
        self.stats.get_stats_level(level)
    }

    pub fn get_effective_hp_at_level(&self, level: u32) -> Result<EffectiveHealth, ChampStatError> {
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

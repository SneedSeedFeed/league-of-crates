use crate::champion::champstats::ChampStatError::{LevelRangeError, ZeroError};

#[derive(Debug)]
pub enum ChampStatError {
    ZeroError,
    LevelRangeError,
}

pub struct EffectiveHealth {
    physical: f32,
    magical: f32,
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

impl LevelStats {
    pub fn get_effective_health(&self) -> EffectiveHealth {
        EffectiveHealth {
            physical: self.hp * (1f32 + 0.01 * self.armor),
            magical: self.hp * (1f32 + 0.01 * self.spellblock),
        }
    }
}

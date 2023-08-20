use serde::de::{MapAccess, SeqAccess};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::error;
use std::fmt::{Display, Formatter};

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct GameData {
    #[serde(rename = "mCharacterName")]
    pub name: String,

    #[serde(rename = "baseHP")]
    pub base_hp: f32,

    #[serde(rename = "hpPerLevel")]
    pub hp_per_level: f32,

    #[serde(rename = "baseStaticHPRegen")]
    pub base_hp_regen: f32,

    #[serde(rename = "hpRegenPerLevel")]
    pub hp_regen_per_level: f32,

    #[serde(rename = "baseDamage")]
    pub base_attack_damage: f32,

    #[serde(rename = "damagePerLevel")]
    pub attack_damage_per_level: f32,

    #[serde(rename = "baseArmor")]
    pub base_armor: f32,

    #[serde(rename = "armorPerLevel")]
    pub armor_per_level: f32,

    #[serde(rename = "baseSpellBlock")]
    pub base_magic_resist: f32,

    #[serde(rename = "spellBlockPerLevel")]
    pub magic_resist_per_level: f32,

    #[serde(rename = "baseMoveSpeed")]
    pub base_move_speed: f32,

    #[serde(rename = "attackRange")]
    pub base_attack_range: f32,

    #[serde(rename = "attackSpeed")]
    pub base_attack_speed: f32,

    #[serde(rename = "attackSpeedRatio")]
    pub attack_speed_ratio: f32,

    #[serde(rename = "attackSpeedPerLevel")]
    pub attack_speed_per_level: f32,

    #[serde(rename = "primaryAbilityResource")]
    pub primary_ability_resource: AbilityResource,

    #[serde(rename = "secondaryAbilityResource")]
    pub secondary_ability_resource: AbilityResource,

    #[serde(rename = "acquisitionRange")]
    pub acquisition_range: f32,

    #[serde(rename = "selectionHeight")]
    pub selection_height: f32,

    #[serde(rename = "selectionRadius")]
    pub selection_radius: f32,
}

impl GameData{
    pub fn get_stats_level(&self, level:u32) -> Result<LevelStats, ChampStatError>{
        if level==0{
            return Err(ChampStatError::ZeroError)
        }

        Ok(LevelStats{
            hp:  self.base_hp + self.hp_per_level * (level - 1) as f32,
            move_speed: self.base_move_speed,
            armor: self.base_armor + self.armor_per_level * (level-1) as f32,
            magic_resist: self.base_magic_resist + self.magic_resist_per_level * (level-1) as f32,
            attack_range: self.base_attack_range,
            hp_regen: self.base_hp_regen + self.hp_regen_per_level * (level-1) as f32,
            attack_damage: self.base_attack_damage + self.attack_damage_per_level * (level-1) as f32,
            attack_speed: self.base_attack_speed + self.attack_speed_ratio*(self.attack_speed_per_level*0.01*(level-1) as f32),
            primary_resource_base: self.primary_ability_resource.base + self.primary_ability_resource.per_level * (level-1) as f32,
            primary_resource_regen: self.primary_ability_resource.base_regen + self.primary_ability_resource.regen_per_level * (level-1) as f32,
            secondary_resource_base: self.secondary_ability_resource.base + self.secondary_ability_resource.per_level * (level-1) as f32,
            secondary_resource_regen: self.secondary_ability_resource.base_regen + self.secondary_ability_resource.regen_per_level * (level -1) as f32,
        })
    }

    pub fn get_stats_range(&self, start:u32, stop:u32) -> Result<Vec<LevelStats>, ChampStatError>{
        if start>=stop{
            return Err(ChampStatError::LevelRangeError)
        }
        if start == 0 {
            return Err(ChampStatError::ZeroError)
        }
        Ok((start..stop)
            .into_iter()
            .map(|i| self.get_stats_level(i).unwrap())
            .collect())
    }
}

impl Default for GameData {
    fn default() -> Self {
        GameData {
            primary_ability_resource: AbilityResource::default(),
            secondary_ability_resource: AbilityResource::default(),
            name: "".to_string(),
            base_hp: 100f32,
            hp_per_level: 0f32,
            base_hp_regen: 1.0f32,
            hp_regen_per_level: 0f32,
            base_attack_damage: 10f32,
            attack_damage_per_level: 0f32,
            base_armor: 1f32,
            armor_per_level: 0f32,
            base_magic_resist: 0f32,
            magic_resist_per_level: 0f32,
            base_move_speed: 100f32,
            base_attack_range: 100f32,
            base_attack_speed: 0f32,
            attack_speed_ratio: 1f32,
            attack_speed_per_level: 0f32,
            acquisition_range: 750f32,
            selection_height: -1f32,
            selection_radius: -1f32,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AbilityResource {
    #[serde(rename = "arIncrements")]
    pub increments: f32,

    #[serde(rename = "arBaseStaticRegen")]
    pub base_regen: f32,

    #[serde(rename = "arIsShown")]
    pub is_shown: bool,

    #[serde(rename = "HideEmptyPips")]
    pub hide_empty_pips: bool,

    #[serde(rename = "arNegativeSpacer")]
    pub negative_spacer: bool,

    #[serde(rename = "arOverrideEmptyPipName")]
    pub override_empty_pip_name: String,

    #[serde(rename = "arHasRegenText")]
    pub has_regen_text: bool,

    #[serde(rename = "arAllowMaxValueToBeOverridden")]
    pub allow_max_value_to_be_overridden: bool,

    #[serde(rename = "arDisplayAsPips")]
    pub display_as_pips: bool,

    #[serde(rename = "asOverrideMediumPipName")]
    pub override_medium_pip_name: String,

    #[serde(rename = "arBase")]
    pub base: f32,

    #[serde(rename = "arOverrideSpacerName")]
    pub override_spacer_name: String,

    #[serde(rename = "arType")]
    pub resource_type: ResourceType,

    #[serde(rename = "arMaxSegments")]
    pub max_segments: i32,

    #[serde(rename = "arOverrideLargePipName")]
    pub override_large_pip_name: String,

    #[serde(rename = "arIsShownOnlyOnLocalPlayer")]
    pub is_shown_only_on_local_player: bool,

    #[serde(rename = "arOverrideSmallPipName")]
    pub override_small_pip_name: String,

    #[serde(rename = "arPerLevel")]
    pub per_level: f32,

    #[serde(rename = "arRegenPerLevel")]
    pub regen_per_level: f32,
}

impl Default for AbilityResource {
    fn default() -> Self {
        AbilityResource {
            increments: 0f32,
            base_regen: 1f32,
            is_shown: true,
            hide_empty_pips: false,
            negative_spacer: false,
            override_empty_pip_name: "".to_string(),
            has_regen_text: true,
            allow_max_value_to_be_overridden: false,
            display_as_pips: false,
            override_medium_pip_name: "".to_string(),
            base: 100f32,
            override_spacer_name: "".to_string(),
            resource_type: ResourceType::None,
            max_segments: 0,
            override_large_pip_name: "".to_string(),
            is_shown_only_on_local_player: false,
            override_small_pip_name: "".to_string(),
            per_level: 0f32,
            regen_per_level: 0f32,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(from = "i32")]
pub enum ResourceType {
    Wind,
    Heat,
    Energy,
    Shield,
    BattleFury,
    Ammo,
    Rage,
    Ferocity,
    DragonFury,
    PrimalFury,
    None,
    Moonlight,
    Other,
    Bloodwell,
    Mana,
}

impl From<i32> for ResourceType {
    fn from(value: i32) -> Self {
        match value {
            0 => ResourceType::Mana,
            1 => ResourceType::Energy,
            2 => ResourceType::None,
            3 => ResourceType::Shield,
            4 => ResourceType::BattleFury,
            5 => ResourceType::DragonFury,
            6 => ResourceType::PrimalFury,
            7 => ResourceType::Heat,
            8 => ResourceType::Rage,
            9 => ResourceType::Ferocity,
            10 => ResourceType::Bloodwell,
            11 => ResourceType::Wind,
            12 => ResourceType::Other,
            13 => ResourceType::Moonlight,
            14 => ResourceType::Ammo,
            _ => ResourceType::None,
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(from = "String")]
pub enum Roles {
    Tank,
    Assassin,
    Marksman,
    Mage,
    Support,
    Fighter,
    Other(String),
}

impl From<String> for Roles {
    fn from(value: String) -> Self {
        match value.as_str() {
            "tank" => Roles::Tank,
            "assassin" => Roles::Assassin,
            "marksman" => Roles::Marksman,
            "mage" => Roles::Mage,
            "support" => Roles::Support,
            "fighter" => Roles::Fighter,
            _ => Roles::Other(value),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Champion {
    pub id: i32,
    pub name: String,
    pub alias: String,
    pub roles: Vec<Roles>,

    #[serde(skip)]
    pub stats: Option<GameData>,
}

impl Champion {
    pub async fn populate_gamedata(&mut self) -> Result<(), Box<dyn error::Error>> {
        if self.id == -1 || self.stats.is_some() {
            return Ok(());
        }

        const CHAMP_LOC: &str =
            "https://raw.communitydragon.org/latest/game/data/characters/{}/{}.bin.json";

        let request = reqwest::get(CHAMP_LOC.replace("{}", self.alias.to_lowercase().as_str()))
            .await?
            .json::<Value>()
            .await?;

        let character_records = request
            .as_object()
            .expect("Champ json is not object")
            .get(
                "Characters/{}/CharacterRecords/Root"
                    .replace("{}", self.alias.as_str())
                    .as_str(),
            )
            .expect("Champ json has no CharacterRecords");

        self.stats = Some(serde_json::from_value(character_records.to_owned())?);

        Ok(())
    }

    pub fn get_stats_level(&self, level: u32) -> Result<LevelStats, ChampStatError> {
        if self.stats.is_some() {
            Ok(self.stats.as_ref().unwrap().get_stats_level(level)?)
        } else {
            Err(ChampStatError::StatsNotLoaded)
        }
    }

    pub fn get_stats_range(&self, start:u32, stop:u32) -> Result<Vec<LevelStats>, ChampStatError>{
        if self.stats.is_some() {
            Ok(self.stats.as_ref().unwrap().get_stats_range(start, stop)?)
        } else {
            Err(ChampStatError::StatsNotLoaded)
        }
    }

    pub fn get_effective_hp_at_level(&self, level: u32) -> Result<EffectiveHealth, ChampStatError> {
        if level==0 {
            return Err(ChampStatError::ZeroError)
        }
        Ok(self.get_stats_level(level)?.get_effective_health())
    }

    pub fn get_effective_hp_range(
        &self,
        min_level: u32,
        max_level: u32,
    ) -> Result<Vec<EffectiveHealth>, ChampStatError> {
        if self.stats.is_none(){
            return Err(ChampStatError::StatsNotLoaded)
        }

        Ok(self
            .stats.as_ref().unwrap()
            .get_stats_range(min_level, max_level)?
            .iter()
            .map(|x| x.get_effective_health())
            .collect())
    }
}

#[derive(Debug)]
pub enum ChampStatError {
    ZeroError,
    LevelRangeError,
    StatsNotLoaded,
}

impl Display for ChampStatError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Stat calculation error occurred")
    }
}

impl error::Error for ChampStatError {}

pub struct LevelStats {
    pub hp: f32,
    pub move_speed: f32,
    pub armor: f32,
    pub magic_resist: f32,
    pub attack_range: f32,
    pub hp_regen: f32,
    pub attack_damage: f32,
    pub attack_speed: f32,
    pub primary_resource_base: f32,
    pub primary_resource_regen: f32,
    pub secondary_resource_base: f32,
    pub secondary_resource_regen: f32,
}

impl LevelStats {
    pub fn get_effective_health(&self) -> EffectiveHealth {
        EffectiveHealth {
            physical: self.hp * (1f32 + 0.01 * self.armor),
            magical: self.hp * (1f32 + 0.01 * self.magic_resist),
        }
    }
}

pub struct EffectiveHealth {
    pub physical: f32,
    pub magical: f32,
}

#[derive(Debug, Deserialize)]
#[serde(transparent)]
pub struct ChampDir {
    pub champions: Vec<Champion>,
}

impl ChampDir {
    pub async fn from_cdragon() -> Result<ChampDir, Box<dyn error::Error>> {
        let res = reqwest::get(
            "https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/champion-summary.json",
        )
            .await?
            .json::<Value>()
            .await?;

        let mut dir = serde_json::from_value::<ChampDir>(res)?;

        // Removing the None champ
        dir.champions.remove(0);

        Ok(dir)
    }
    pub fn get_by_name(&self, name: String) -> Option<&Champion> {
        self.champions.iter().find(|x| x.name == name)
    }
    pub fn get_by_alias(&self, alias: String) -> Option<&Champion> {
        self.champions.iter().find(|x| x.alias == alias)
    }
    pub fn get_by_key(&self, id: i32) -> Option<&Champion> {
        self.champions.iter().find(|x| x.id == id)
    }
}
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(default)]
struct GameData {
    #[serde(rename = "primaryAbilityResource")]
    primary_ability_resource: AbilityResource,

    #[serde(rename = "secondaryAbilityResource")]
    secondary_ability_resource: AbilityResource,

    #[serde(rename = "mCharacterName")]
    name: String,

    #[serde(rename = "baseHP")]
    base_hp: f32,

    #[serde(rename = "hpPerLevel")]
    hp_per_level: f32,

    #[serde(rename = "baseStaticHPRegen")]
    base_hp_regen: f32,

    #[serde(rename = "hpRegenPerLevel")]
    hp_regen_per_level: f32,

    #[serde(rename = "baseDamage")]
    base_attack_damage: f32,

    #[serde(rename = "damagePerLevel")]
    attack_damage_per_level: f32,

    #[serde(rename = "baseArmor")]
    base_armor: f32,

    #[serde(rename = "armorPerLevel")]
    armor_per_level: f32,

    #[serde(rename = "baseSpellBlock")]
    base_magic_resist: f32,

    #[serde(rename = "spellBlockPerLevel")]
    magic_resist_per_level: f32,

    #[serde(rename = "baseMoveSpeed")]
    base_move_speed: f32,

    #[serde(rename = "attackRange")]
    base_attack_range: f32,

    #[serde(rename = "attackSpeed")]
    base_attack_speed: f32,

    #[serde(rename = "attackSpeedRatio")]
    attack_speed_ratio: f32,

    #[serde(rename = "attackSpeedPerLevel")]
    attack_speed_per_level: f32,

    #[serde(rename = "acquisitionRange")]
    acquisition_range: f32,

    #[serde(rename = "selectionHeight")]
    selection_height: f32,

    #[serde(rename = "selectionRadius")]
    selection_radius: f32,
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
struct AbilityResource {
    #[serde(rename = "arIncrements")]
    increments: f32,

    #[serde(rename = "arBaseStaticRegen")]
    base_regen: f32,

    #[serde(rename = "arIsShown")]
    is_shown: bool,

    #[serde(rename = "HideEmptyPips")]
    hide_empty_pips: bool,

    #[serde(rename = "arNegativeSpacer")]
    negative_spacer: bool,

    #[serde(rename = "arOverrideEmptyPipName")]
    override_empty_pip_name: String,

    #[serde(rename = "arHasRegenText")]
    has_regen_text: bool,

    #[serde(rename = "arAllowMaxValueToBeOverridden")]
    allow_max_value_to_be_overridden: bool,

    #[serde(rename = "arDisplayAsPips")]
    display_as_pips: bool,

    #[serde(rename = "asOverrideMediumPipName")]
    override_medium_pip_name: String,

    #[serde(rename = "arBase")]
    base: f32,

    #[serde(rename = "arOverrideSpacerName")]
    override_spacer_name: String,

    #[serde(rename = "arType")]
    resource_type: ResourceType,

    #[serde(rename = "arMaxSegments")]
    max_segments: i32,

    #[serde(rename = "arOverrideLargePipName")]
    override_large_pip_name: String,

    #[serde(rename = "arIsShownOnlyOnLocalPlayer")]
    is_shown_only_on_local_player: bool,

    #[serde(rename = "arOverrideSmallPipName")]
    override_small_pip_name: String,

    #[serde(rename = "arPerLevel")]
    per_level: f32,

    #[serde(rename = "arRegenPerLevel")]
    regen_per_level: f32,
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
enum ResourceType {
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

struct Champion{
    stats: GameData
}

use crate::champion::champdir::ChampDir;
use approx::assert_relative_eq;

mod champion;
mod json_macros;
use crate::json_macros::thirteen_fifteen_one::champions_json;

macro_rules! new_dir {
    () => {
        ChampDir::new_from_value(champions_json!()).unwrap()
    };
}
macro_rules! new_dir_wrapped {
    () => {
        ChampDir::new_from_value(champions_json!())
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let champ_dir = new_dir_wrapped!();
        assert!(champ_dir.is_ok());
    }

    #[test]
    fn has_num_champs() {
        let champ_dir = new_dir!();
        assert_eq!(champ_dir.champions.len(), 164)
    }

    #[test]
    fn get_key_ok() {
        let champ_dir = new_dir!();
        let champ = champ_dir.get_by_key(516);
        assert!(champ.is_some());
        assert_eq!(champ.unwrap().name, "Ornn");
        assert!(champ_dir.get_by_key(2556).is_none())
    }

    #[test]
    fn get_name_ok() {
        let champ_dir = new_dir!();
        let champ = champ_dir.get_by_name("Ornn");
        assert!(champ.is_some());
        assert_eq!(champ.unwrap().name, "Ornn");
        assert!(champ_dir.get_by_name("Crystal Maiden").is_none())
    }

    #[test]
    fn calcs_ok() {
        let champ_dir = new_dir!();
        let calc = champ_dir
            .get_by_key(103)
            .unwrap()
            .get_stats_level(18)
            .unwrap();

        //Compare Ahri stats we calc vs ones from the wiki
        assert_relative_eq!(calc.hp, 2222f32);
        assert_relative_eq!(calc.hpregen, 12.7f32);
        assert_relative_eq!(calc.mp, 843f32);
        assert_relative_eq!(calc.mpregen, 21.6f32);
        assert_relative_eq!(calc.attackdamage, 104f32);

        // As long as we are correct to 3 decimal places IDGAF
        assert_relative_eq!(calc.attackspeed, 0.895f32, max_relative = 0.0005);

        assert_relative_eq!(calc.armor, 100.9f32);
        assert_relative_eq!(calc.spellblock, 52.1f32);
        assert_relative_eq!(calc.attackrange, 550f32);
        assert_relative_eq!(calc.movespeed, 330f32);

        let senna = champ_dir
            .get_by_name("Senna")
            .unwrap()
            .get_stats_level(18)
            .unwrap();

        assert_relative_eq!(senna.attackspeed, 0.897f32, max_relative = 0.0005)
    }
}

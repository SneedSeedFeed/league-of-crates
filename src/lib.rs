mod champion;

macro_rules! new_dir_wrapped{
    ()=>{
        ChampDir::from_cdragon().await
    }
}

macro_rules! new_dir{
    ()=>{
        ChampDir::from_cdragon().await?
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_relative_eq;
    use crate::champion::ChampDir;

    #[tokio::test]
    async fn it_works() {
        let champ_dir = new_dir_wrapped!();
        assert!(champ_dir.is_ok())
    }

    #[tokio::test]
    async fn has_num_champs() -> Result<(), Box<dyn std::error::Error>>{
        let champ_dir = new_dir!();
        assert_eq!(champ_dir.champions.len(),164);
        Ok(())
    }

    #[tokio::test]
    async fn get_key_ok() -> Result<(), Box<dyn std::error::Error>> {
        let champ_dir = new_dir!();
        let champ = champ_dir.get_by_key(516);
        assert!(champ.is_some());
        assert_eq!(champ.unwrap().name, "Ornn");
        assert!(champ_dir.get_by_key(2556).is_none());
        Ok(())
    }

    #[tokio::test]
    async fn get_name_ok() -> Result<(), Box<dyn std::error::Error>>  {
        let champ_dir = new_dir!();
        let champ = champ_dir.get_by_name("Ornn".to_string());
        assert!(champ.is_some());
        assert_eq!(champ.unwrap().name, "Ornn".to_string());
        assert!(champ_dir.get_by_name("Crystal Maiden".to_string()).is_none());
        Ok(())
    }

    #[tokio::test]
    async fn calcs_ok() -> Result<(), Box<dyn std::error::Error>> {
        let mut champ_dir = new_dir!();

        for champ in champ_dir.champions.iter_mut(){
            if champ.name == "Ahri".to_string() || champ.name == "Senna".to_string() {
                champ.populate_gamedata().await?;
            }
        }

        let calc = champ_dir
            .get_by_key(103)
            .unwrap()
            .get_stats_level(18)
            .unwrap();

        //Compare Ahri stats we calc vs ones from the wiki
        assert_relative_eq!(calc.hp, 2222f32);
        assert_relative_eq!(calc.hp_regen, 2.54f32);
        assert_relative_eq!(calc.primary_resource_base, 843f32);
        assert_relative_eq!(calc.primary_resource_regen, 4.32f32);
        assert_relative_eq!(calc.attack_damage, 104f32);

        // As long as we are correct to 3 decimal places IDGAF
        assert_relative_eq!(calc.attack_speed, 0.895f32, max_relative = 0.0005);

        assert_relative_eq!(calc.armor, 100.9f32);
        assert_relative_eq!(calc.magic_resist, 52.1f32);
        assert_relative_eq!(calc.attack_range, 550f32);
        assert_relative_eq!(calc.move_speed, 330f32);

        let senna = champ_dir
            .get_by_name("Senna".to_string())
            .unwrap()
            .get_stats_level(18)
            .unwrap();

        assert_relative_eq!(senna.attack_speed, 0.897f32, max_relative = 0.002);

        Ok(())
    }
}

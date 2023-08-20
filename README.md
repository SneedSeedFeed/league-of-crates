This is currently just a basic library to help parsing the binary data pulled by CDragon. In future I will be looking to expand this out a bit.

I'm a rust newbie and this is my starter learning project after finishing Rustlings. Hopefully it will help me achieve ~~world domination, full oxidation~~ some reasonable fluency in Rust and help others along the way with anything League of Legends related like maybe a calculator site or smthn idk.

```rust
// You should be handling these errors, or not I'm not your Dad
let champ_dir = ChampDir::from_cdragon().await?;

let best_champ_in_the_game = champ_dir.get_by_key(516).unwrap();

println!("{}", best_champ_in_the_game.name);
```
```
Output: Ornn
```

I will probably maybe potentially add proper docs in the future but while it's so shrimple I think I can get away with just some example code.

Current version repo: https://github.com/SneedSeedFeed/league-of-crates/tree/0.2.0-rework

Todo:
  - Let everything be loaded from local files a bit easier
  - Allow pulling from a specific version
  - Proper docs

## Changelog
- 0.3.5
  - Another cargo.toml update. I wish there was a cheat sheet to everything I can put in here.

- 0.3.4 - Added keywords and repo to the cargo.toml, in case you want to see my horrendous code.

- 0.3.3 - Pinned Serde version to 1.0.171, because of the precompiled binaries fiasco. Won't matter most likely.

- 0.3.2 - Derived Clone and PartialEq because I forgor that rust won't allow you to do that if you use this crate

- 0.3.1 - Updated readme

- 0.3.0
  - Switched to using CDragon, many a breaking change
  - Regen stats are now per 1 second not per 5
  - All resources are loaded properly and match game binaries as closely as is reasonable
  - Much cleaner deserialization as I read the documentation
  - Currently everything is grabbed from the latest version with reqwest, next step is allowing you to use local files and specific versions
  - Champions game data is not loaded initially,  you need to call populate_gamedata().await

- 0.2.0
  - Pain and suffering
  - Can create champ dir from a json value
  - Added macros that output the 13.15.1 champions.json as a value. Use champion_json!() and special_json!()
  - Added the "adjustments/specialcases/corrections/i need to fix all this code it's an ugly bodge" json to fix Riot's lack of information on attack speed ratios and shit
  - Didn't actually fully finish the corrections json so some champs attack speed will be off
  - Fixed the attack speed calculation
  - Genuinely some of the worst code I have ever wrote, and I will likely do a full rewrite of this stuff but hey at least we get Senna's attack speed at level 18 correct to 2 decimal places and actually expose the attack speed ratio stats

- 0.1.1 - Added readme
- 0.1.0 - Initial release
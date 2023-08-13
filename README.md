This is currently just a basic library to help parsing the champions.json file from Riot Games' Data Dragon. In future I will be looking to expand this out a bit.

I'm a rust newbie and this is my starter learning project after finishing Rustlings. Hopefully it will help me achieve ~~world domination, full oxidation~~ some reasonable fluency in Rust and help others along the way with anything League of Legends related like maybe a calculator site or smthn idk.

```rust
// You should be handling these errors, or not I'm not your Dad
let champ_dir = ChampDir::new(Path::new("champion.json")).unwrap()

let best_champ_in_the_game = champ_dir.get_by_key(516).unwrap();

println!("{}", best_champ_in_the_game.name);
```
```
Output: Ornn
```

I will probably maybe potentially add proper docs in the future but while it's so shrimple I think I can get away with just some example code. As everything comes from .json I tried to pass as many of the potential errors back as possible instead of risking a panic so watch out for that I guess.

## Changelog
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
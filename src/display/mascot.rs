use crate::models::GuiltLevel;
use colored::Colorize;

pub fn print_mascot(level: GuiltLevel) {
    let art = mascot_art(level);
    for line in art.lines() {
        println!("  {}", colorize_art(line, level));
    }
}

fn colorize_art(line: &str, level: GuiltLevel) -> String {
    match level {
        GuiltLevel::DigitalSaint => line.green().to_string(),
        GuiltLevel::CarbonCurious => line.cyan().to_string(),
        GuiltLevel::TreeTrimmer => line.yellow().to_string(),
        GuiltLevel::ForestFlattener => line.truecolor(255, 165, 0).to_string(),
        GuiltLevel::EcoTerrorist => line.red().to_string(),
        GuiltLevel::PlanetIncinerator => line.truecolor(139, 0, 0).to_string(),
        GuiltLevel::HeatDeathAccelerator => line.magenta().to_string(),
        GuiltLevel::Himanshu => line.white().bold().to_string(),
    }
}

fn mascot_art(level: GuiltLevel) -> &'static str {
    match level {
        GuiltLevel::DigitalSaint => {
            r#"
    🌳
   /|\
  / | \
 /  |  \
    |
   /|\
"#
        }
        GuiltLevel::CarbonCurious => {
            r#"
    🌿
   /|
  / | \
    |  \
    |
   /|\
"#
        }
        GuiltLevel::TreeTrimmer => {
            r#"
    ·
   /|
     | \
    |
    |
   /|\
"#
        }
        GuiltLevel::ForestFlattener => {
            r#"
    ✂️
    |
    |
    |
   /|\
  _____
"#
        }
        GuiltLevel::EcoTerrorist => {
            r#"
  🔥🔥🔥
 🔥 | 🔥
  🔥|🔥
    |
   /|\
  ~~~~~
"#
        }
        GuiltLevel::PlanetIncinerator => {
            r#"
 💀💀💀💀
 ~~~..~~~
  ~.  .~
   ....
  _/  \_
  ======
"#
        }
        GuiltLevel::HeatDeathAccelerator => {
            r#"
    ☢️
   .||.
  / || \
 / .||. \
|  ||||  |
 \______/
"#
        }
        GuiltLevel::Himanshu => {
            r#"
    Himanshu
"#
        }
    }
}

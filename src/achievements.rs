use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

use crate::models::{GuiltLevel, UsageBucket};

#[derive(Serialize, Deserialize, Default)]
pub struct AchievementStore {
    pub unlocked: HashMap<String, String>, // id -> date unlocked
}

struct AchievementDef {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    check: fn(&[UsageBucket]) -> bool,
}

const ACHIEVEMENTS: &[AchievementDef] = &[
    AchievementDef {
        id: "first_tree",
        name: "First Tree Killed",
        description: "Destroyed your first tree. The squirrels will remember.",
        check: |b| total_trees(b) >= 1.0,
    },
    AchievementDef {
        id: "hundred_dollar_day",
        name: "Hundred Dollar Day",
        description: "Spent $100 in a single day. Your wallet weeps with the planet.",
        check: |b| b.iter().any(|b| b.cost.total_cost_usd >= 100.0),
    },
    AchievementDef {
        id: "gigabyte_club",
        name: "Gigabyte Club",
        description: "Generated 1 billion tokens. That's a LOT of autocomplete.",
        check: |b| total_tokens(b) >= 1_000_000_000,
    },
    AchievementDef {
        id: "carbon_centurion",
        name: "Carbon Centurion",
        description: "100 kg of CO2. A Roman centurion would be proud. The planet is not.",
        check: |b| total_co2(b) >= 100_000.0,
    },
    AchievementDef {
        id: "the_deforester",
        name: "The Deforester",
        description: "10 trees destroyed. You're basically a logging company now.",
        check: |b| total_trees(b) >= 10.0,
    },
    AchievementDef {
        id: "heat_death_pioneer",
        name: "Heat Death Pioneer",
        description: "Reached Heat Death Accelerator. The universe noticed you.",
        check: |b| {
            b.iter()
                .any(|b| b.guilt.level == GuiltLevel::HeatDeathAccelerator)
        },
    },
];

fn total_trees(buckets: &[UsageBucket]) -> f64 {
    buckets.iter().map(|b| b.impact.trees_destroyed).sum()
}

fn total_tokens(buckets: &[UsageBucket]) -> u64 {
    buckets.iter().map(|b| b.tokens.total_tokens()).sum()
}

fn total_co2(buckets: &[UsageBucket]) -> f64 {
    buckets.iter().map(|b| b.impact.co2_grams).sum()
}

fn store_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("/tmp"))
        .join("ccguilt")
        .join("achievements.json")
}

pub fn load_store() -> AchievementStore {
    let path = store_path();
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        AchievementStore::default()
    }
}

fn save_store(store: &AchievementStore) {
    let path = store_path();
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Ok(json) = serde_json::to_string_pretty(store) {
        let _ = std::fs::write(&path, json);
    }
}

pub fn check_and_announce(buckets: &[UsageBucket]) {
    let mut store = load_store();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut any_new = false;

    for ach in ACHIEVEMENTS {
        if store.unlocked.contains_key(ach.id) {
            continue;
        }
        if (ach.check)(buckets) {
            store.unlocked.insert(ach.id.to_string(), today.clone());
            if !any_new {
                println!();
                any_new = true;
            }
            println!(
                "  {} {}",
                "Achievement Unlocked:".yellow().bold(),
                ach.name.bold(),
            );
            println!("    {}", ach.description.dimmed().italic());
        }
    }

    if any_new {
        save_store(&store);
    }
}

pub fn show_all() {
    let store = load_store();

    println!();
    println!("  {}", "HALL OF SHAME".bright_red().bold().underline());
    println!();

    for ach in ACHIEVEMENTS {
        if let Some(date) = store.unlocked.get(ach.id) {
            println!(
                "  {} {}  {}",
                "[✓]".green().bold(),
                ach.name.bold(),
                format!("— Unlocked {}", date).dimmed(),
            );
            println!("      {}", ach.description.dimmed());
        } else {
            println!(
                "  {} {}",
                "[ ]".dimmed(),
                ach.name.dimmed(),
            );
            println!("      {}", ach.description.dimmed());
        }
    }
    println!();
}

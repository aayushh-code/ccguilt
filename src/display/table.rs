use colored::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;

use crate::models::{GuiltLevel, UsageBucket};

pub fn render_table(buckets: &[UsageBucket], no_guilt: bool) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Header
    let mut headers = vec![
        Cell::new("Period").set_alignment(CellAlignment::Left),
        Cell::new("Tokens").set_alignment(CellAlignment::Right),
        Cell::new("Cost").set_alignment(CellAlignment::Right),
        Cell::new("Energy").set_alignment(CellAlignment::Right),
        Cell::new("CO2").set_alignment(CellAlignment::Right),
        Cell::new("Water").set_alignment(CellAlignment::Right),
        Cell::new("Trees").set_alignment(CellAlignment::Right),
    ];
    if !no_guilt {
        headers.push(Cell::new("Guilt").set_alignment(CellAlignment::Center));
    }
    table.set_header(headers);

    for bucket in buckets {
        let mut row = vec![
            Cell::new(&bucket.label),
            Cell::new(format_tokens(bucket.tokens.total_tokens())),
            Cell::new(format_cost(bucket.cost.total_cost_usd)),
            Cell::new(format_energy(bucket.impact.energy_wh)),
            Cell::new(format_co2(bucket.impact.co2_grams)),
            Cell::new(format_water(bucket.impact.water_ml)),
            Cell::new(format_trees(bucket.impact.trees_destroyed)),
        ];
        if !no_guilt {
            row.push(Cell::new(&bucket.guilt.title).fg(guilt_table_color(bucket.guilt.level)));
        }
        table.add_row(row);
    }

    // Totals row if multiple buckets
    if buckets.len() > 1 {
        let total_tokens: u64 = buckets.iter().map(|b| b.tokens.total_tokens()).sum();
        let total_cost: f64 = buckets.iter().map(|b| b.cost.total_cost_usd).sum();
        let total_energy: f64 = buckets.iter().map(|b| b.impact.energy_wh).sum();
        let total_co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
        let total_water: f64 = buckets.iter().map(|b| b.impact.water_ml).sum();
        let total_trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();

        let worst_guilt = buckets
            .iter()
            .map(|b| b.guilt.level)
            .max()
            .unwrap_or(GuiltLevel::DigitalSaint);
        let worst_title = buckets
            .iter()
            .find(|b| b.guilt.level == worst_guilt)
            .map(|b| b.guilt.title.clone())
            .unwrap_or_default();

        let mut total_row = vec![
            Cell::new("TOTAL".bold().to_string()).fg(comfy_table::Color::White),
            Cell::new(format_tokens(total_tokens)).fg(comfy_table::Color::White),
            Cell::new(format_cost(total_cost)).fg(comfy_table::Color::White),
            Cell::new(format_energy(total_energy)).fg(comfy_table::Color::White),
            Cell::new(format_co2(total_co2)).fg(comfy_table::Color::White),
            Cell::new(format_water(total_water)).fg(comfy_table::Color::White),
            Cell::new(format_trees(total_trees)).fg(comfy_table::Color::White),
        ];
        if !no_guilt {
            total_row.push(Cell::new(worst_title).fg(guilt_table_color(worst_guilt)));
        }
        table.add_row(total_row);
    }

    println!("{table}");
}

fn guilt_table_color(level: GuiltLevel) -> comfy_table::Color {
    match level {
        GuiltLevel::DigitalSaint => comfy_table::Color::Green,
        GuiltLevel::CarbonCurious => comfy_table::Color::Cyan,
        GuiltLevel::TreeTrimmer => comfy_table::Color::Yellow,
        GuiltLevel::ForestFlattener => comfy_table::Color::DarkYellow,
        GuiltLevel::EcoTerrorist => comfy_table::Color::Red,
        GuiltLevel::PlanetIncinerator => comfy_table::Color::DarkRed,
        GuiltLevel::HeatDeathAccelerator => comfy_table::Color::Magenta,
    }
}

fn format_tokens(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.1}B", n as f64 / 1e9)
    } else if n >= 1_000_000 {
        format!("{:.1}M", n as f64 / 1e6)
    } else if n >= 1_000 {
        format!("{:.1}K", n as f64 / 1e3)
    } else {
        n.to_string()
    }
}

fn format_cost(usd: f64) -> String {
    if usd >= 1000.0 {
        format!("${:.0}", usd)
    } else if usd >= 1.0 {
        format!("${:.2}", usd)
    } else if usd >= 0.01 {
        format!("{:.0}c", usd * 100.0)
    } else if usd > 0.0 {
        format!("{:.2}c", usd * 100.0)
    } else {
        "$0".to_string()
    }
}

fn format_energy(wh: f64) -> String {
    if wh >= 1_000_000.0 {
        format!("{:.1} MWh", wh / 1_000_000.0)
    } else if wh >= 1000.0 {
        format!("{:.1} kWh", wh / 1000.0)
    } else if wh >= 1.0 {
        format!("{:.1} Wh", wh)
    } else {
        format!("{:.2} mWh", wh * 1000.0)
    }
}

fn format_co2(grams: f64) -> String {
    if grams >= 1_000_000.0 {
        format!("{:.1} t", grams / 1_000_000.0)
    } else if grams >= 1000.0 {
        format!("{:.1} kg", grams / 1000.0)
    } else if grams >= 1.0 {
        format!("{:.1} g", grams)
    } else {
        format!("{:.2} mg", grams * 1000.0)
    }
}

fn format_water(ml: f64) -> String {
    if ml >= 1_000_000.0 {
        format!("{:.1} m3", ml / 1_000_000.0)
    } else if ml >= 1000.0 {
        format!("{:.1} L", ml / 1000.0)
    } else if ml >= 1.0 {
        format!("{:.0} mL", ml)
    } else {
        format!("{:.2} uL", ml * 1000.0)
    }
}

fn format_trees(trees: f64) -> String {
    if trees >= 100.0 {
        format!("{:.0}", trees)
    } else if trees >= 1.0 {
        format!("{:.2}", trees)
    } else if trees >= 0.01 {
        format!("{:.4}", trees)
    } else if trees > 0.0 {
        format!("{:.6}", trees)
    } else {
        "0".to_string()
    }
}

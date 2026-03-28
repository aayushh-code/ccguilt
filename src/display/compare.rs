use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;

use crate::display::format::*;
use crate::models::UsageBucket;

#[allow(dead_code)]
pub fn render_comparison(project_buckets: &[(String, Vec<UsageBucket>)]) {
    if project_buckets.is_empty() {
        return;
    }

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    // Header: Metric | Project1 | Project2 | ...
    let mut headers = vec![Cell::new("Metric").set_alignment(CellAlignment::Left)];
    for (name, _) in project_buckets {
        headers.push(Cell::new(name).set_alignment(CellAlignment::Right));
    }
    table.set_header(headers);

    // Compute totals per project
    let totals: Vec<_> = project_buckets
        .iter()
        .map(|(_, buckets)| {
            let tokens: u64 = buckets.iter().map(|b| b.tokens.total_tokens()).sum();
            let cost: f64 = buckets.iter().map(|b| b.cost.total_cost_usd).sum();
            let energy: f64 = buckets.iter().map(|b| b.impact.energy_wh).sum();
            let co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
            let water: f64 = buckets.iter().map(|b| b.impact.water_ml).sum();
            let trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();
            (tokens, cost, energy, co2, water, trees)
        })
        .collect();

    type ProjectTotals = (u64, f64, f64, f64, f64, f64);
    #[allow(clippy::type_complexity)]
    let metrics: Vec<(&str, Box<dyn Fn(&ProjectTotals) -> String>)> = vec![
        (
            "Tokens",
            Box::new(|t: &(u64, f64, f64, f64, f64, f64)| format_tokens(t.0)),
        ),
        ("Cost", Box::new(|t| format_cost(t.1))),
        ("Energy", Box::new(|t| format_energy(t.2))),
        ("CO2", Box::new(|t| format_co2(t.3))),
        ("Water", Box::new(|t| format_water(t.4))),
        ("Trees", Box::new(|t| format_trees(t.5))),
    ];

    for (name, fmt) in &metrics {
        let mut row = vec![Cell::new(*name)];
        for total in &totals {
            row.push(Cell::new(fmt(total)));
        }
        table.add_row(row);
    }

    println!("{}", "  Project Comparison".bold().underline());
    println!();
    println!("{table}");
}

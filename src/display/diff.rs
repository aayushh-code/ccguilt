use colored::Colorize;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;

use crate::display::format::*;
use crate::models::UsageBucket;

pub fn render_diff(
    label_a: &str,
    buckets_a: &[UsageBucket],
    label_b: &str,
    buckets_b: &[UsageBucket],
) {
    let sum = |bs: &[UsageBucket]| -> (u64, f64, f64, f64, f64) {
        (
            bs.iter().map(|b| b.tokens.total_tokens()).sum(),
            bs.iter().map(|b| b.cost.total_cost_usd).sum(),
            bs.iter().map(|b| b.impact.co2_grams).sum(),
            bs.iter().map(|b| b.impact.water_ml).sum(),
            bs.iter().map(|b| b.impact.trees_destroyed).sum(),
        )
    };

    let (tok_a, cost_a, co2_a, water_a, trees_a) = sum(buckets_a);
    let (tok_b, cost_b, co2_b, water_b, trees_b) = sum(buckets_b);

    println!();
    println!(
        "  {} {} vs {}",
        "Period Comparison:".bold().underline(),
        label_a.cyan(),
        label_b.cyan(),
    );
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Metric").set_alignment(CellAlignment::Left),
        Cell::new(label_a).set_alignment(CellAlignment::Right),
        Cell::new(label_b).set_alignment(CellAlignment::Right),
        Cell::new("Delta").set_alignment(CellAlignment::Right),
        Cell::new("Change").set_alignment(CellAlignment::Right),
    ]);

    add_diff_row(&mut table, "Tokens", tok_a as f64, tok_b as f64, |v| {
        format_tokens(v as u64)
    });
    add_diff_row(&mut table, "Cost", cost_a, cost_b, format_cost);
    add_diff_row(&mut table, "CO2", co2_a, co2_b, format_co2);
    add_diff_row(&mut table, "Water", water_a, water_b, format_water);
    add_diff_row(&mut table, "Trees", trees_a, trees_b, format_trees);

    println!("{table}");
}

fn add_diff_row(
    table: &mut Table,
    metric: &str,
    val_a: f64,
    val_b: f64,
    fmt: impl Fn(f64) -> String,
) {
    let delta = val_b - val_a;
    let pct = if val_a > 0.0 {
        (delta / val_a) * 100.0
    } else if val_b > 0.0 {
        100.0
    } else {
        0.0
    };

    let delta_str = if delta > 0.0 {
        format!("+{}", fmt(delta))
    } else if delta < 0.0 {
        format!("-{}", fmt(delta.abs()))
    } else {
        "0".to_string()
    };

    let pct_str = format!("{:+.1}%", pct);

    let color = if delta > 0.0 {
        comfy_table::Color::Red
    } else if delta < 0.0 {
        comfy_table::Color::Green
    } else {
        comfy_table::Color::White
    };

    table.add_row(vec![
        Cell::new(metric),
        Cell::new(fmt(val_a)),
        Cell::new(fmt(val_b)),
        Cell::new(&delta_str).fg(color),
        Cell::new(&pct_str).fg(color),
    ]);
}

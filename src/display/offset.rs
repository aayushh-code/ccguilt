use colored::Colorize;

use crate::display::format;

const TREE_PLANTING_COST_USD: f64 = 3.0;
const CARBON_CREDIT_USD_PER_TON: f64 = 30.0;
const TREE_CO2_KG_PER_YEAR: f64 = 22.0;

pub fn render_offset(total_co2_grams: f64) {
    if total_co2_grams < 1.0 {
        return;
    }

    let co2_kg = total_co2_grams / 1000.0;
    let co2_tons = co2_kg / 1000.0;

    let trees_needed = (co2_kg / TREE_CO2_KG_PER_YEAR).ceil() as u64;
    let tree_cost = trees_needed as f64 * TREE_PLANTING_COST_USD;
    let credit_cost = co2_tons * CARBON_CREDIT_USD_PER_TON;

    println!();
    println!("  {}", "Carbon Offset Options:".bold().underline());
    println!(
        "    {} {} trees (~{}) to offset annually",
        "Plant trees:".green(),
        trees_needed,
        format::format_cost(tree_cost),
    );
    println!(
        "    {} {} at ~$30/ton for {}",
        "Carbon credits:".cyan(),
        format::format_cost(credit_cost),
        format::format_co2(total_co2_grams),
    );
    println!(
        "    {}",
        "Or just... use Haiku instead of Opus. Same result, less drama."
            .dimmed()
            .italic(),
    );
}

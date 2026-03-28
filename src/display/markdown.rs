use crate::display::format::*;
use crate::models::UsageBucket;
use std::io::Write;

pub fn render_markdown<W: Write>(buckets: &[UsageBucket], writer: &mut W) -> anyhow::Result<()> {
    writeln!(writer, "# Claude Code Guilt Trip Report\n")?;
    writeln!(
        writer,
        "| Period | Tokens | Cost | Energy | CO2 | Water | Trees | Guilt |"
    )?;
    writeln!(
        writer,
        "|--------|--------|------|--------|-----|-------|-------|-------|"
    )?;
    for b in buckets {
        writeln!(
            writer,
            "| {} | {} | {} | {} | {} | {} | {} | {} |",
            b.label,
            format_tokens(b.tokens.total_tokens()),
            format_cost(b.cost.total_cost_usd),
            format_energy(b.impact.energy_wh),
            format_co2(b.impact.co2_grams),
            format_water(b.impact.water_ml),
            format_trees(b.impact.trees_destroyed),
            b.guilt.title,
        )?;
    }

    if buckets.len() > 1 {
        let total_tokens: u64 = buckets.iter().map(|b| b.tokens.total_tokens()).sum();
        let total_cost: f64 = buckets.iter().map(|b| b.cost.total_cost_usd).sum();
        let total_energy: f64 = buckets.iter().map(|b| b.impact.energy_wh).sum();
        let total_co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
        let total_water: f64 = buckets.iter().map(|b| b.impact.water_ml).sum();
        let total_trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();

        writeln!(
            writer,
            "| **TOTAL** | **{}** | **{}** | **{}** | **{}** | **{}** | **{}** | |",
            format_tokens(total_tokens),
            format_cost(total_cost),
            format_energy(total_energy),
            format_co2(total_co2),
            format_water(total_water),
            format_trees(total_trees),
        )?;
    }

    writeln!(writer, "\n*Sources: Jegham et al. 2025, EPA eGRID 2024, Li et al. 2023, Luccioni et al. 2023, USDA Forestry, IEA 2024*")?;
    Ok(())
}

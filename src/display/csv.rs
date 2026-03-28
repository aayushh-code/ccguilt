use crate::models::UsageBucket;
use std::io::Write;

pub fn render_csv<W: Write>(buckets: &[UsageBucket], writer: &mut W) -> anyhow::Result<()> {
    writeln!(
        writer,
        "period,tokens,cost_usd,energy_wh,co2_grams,water_ml,trees_destroyed,guilt_level"
    )?;
    for b in buckets {
        writeln!(
            writer,
            "{},{},{:.4},{:.4},{:.4},{:.4},{:.6},{}",
            b.label,
            b.tokens.total_tokens(),
            b.cost.total_cost_usd,
            b.impact.energy_wh,
            b.impact.co2_grams,
            b.impact.water_ml,
            b.impact.trees_destroyed,
            b.guilt.title,
        )?;
    }
    Ok(())
}

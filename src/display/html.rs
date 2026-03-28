use crate::display::format::*;
use crate::models::UsageBucket;
use std::io::Write;

pub fn render_html<W: Write>(buckets: &[UsageBucket], writer: &mut W) -> anyhow::Result<()> {
    let max_co2 = buckets
        .iter()
        .map(|b| b.impact.co2_grams)
        .fold(0.0_f64, f64::max);

    writeln!(
        writer,
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>Claude Code Guilt Trip Report</title>
<style>
  body {{ background: #1a1a2e; color: #e0e0e0; font-family: 'Courier New', monospace; padding: 2em; }}
  h1 {{ color: #ff4444; text-align: center; }}
  h2 {{ color: #ff6666; }}
  table {{ border-collapse: collapse; width: 100%; margin: 1em 0; }}
  th {{ background: #16213e; color: #ff4444; padding: 0.7em; text-align: right; border-bottom: 2px solid #ff4444; }}
  th:first-child {{ text-align: left; }}
  td {{ padding: 0.5em 0.7em; border-bottom: 1px solid #333; text-align: right; }}
  td:first-child {{ text-align: left; }}
  tr:hover {{ background: #16213e; }}
  .total {{ font-weight: bold; border-top: 2px solid #ff4444; }}
  .bar {{ height: 18px; display: inline-block; border-radius: 2px; }}
  .chart-row {{ display: flex; align-items: center; margin: 4px 0; }}
  .chart-label {{ width: 120px; text-align: right; padding-right: 8px; font-size: 0.85em; color: #aaa; }}
  .chart-value {{ padding-left: 8px; font-size: 0.85em; color: #aaa; }}
  .guilt-saint {{ color: #00cc00; }}
  .guilt-curious {{ color: #00cccc; }}
  .guilt-trimmer {{ color: #cccc00; }}
  .guilt-flattener {{ color: #ff8800; }}
  .guilt-terrorist {{ color: #ff0000; }}
  .guilt-incinerator {{ color: #cc0000; }}
  .guilt-heatdeath {{ color: #cc00cc; }}
  .footer {{ margin-top: 2em; color: #666; font-size: 0.85em; text-align: center; }}
</style>
</head>
<body>
<h1>CLAUDE CODE GUILT TRIP</h1>
<p style="text-align:center;color:#ff6666;">An environmental impact report nobody asked for</p>

<table>
<thead>
<tr><th>Period</th><th>Tokens</th><th>Cost</th><th>Energy</th><th>CO2</th><th>Water</th><th>Trees</th><th>Guilt</th></tr>
</thead>
<tbody>"#
    )?;

    for b in buckets {
        let guilt_class = match b.guilt.level {
            crate::models::GuiltLevel::DigitalSaint => "guilt-saint",
            crate::models::GuiltLevel::CarbonCurious => "guilt-curious",
            crate::models::GuiltLevel::TreeTrimmer => "guilt-trimmer",
            crate::models::GuiltLevel::ForestFlattener => "guilt-flattener",
            crate::models::GuiltLevel::EcoTerrorist => "guilt-terrorist",
            crate::models::GuiltLevel::PlanetIncinerator => "guilt-incinerator",
            crate::models::GuiltLevel::HeatDeathAccelerator => "guilt-heatdeath",
        };
        writeln!(
            writer,
            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td class=\"{}\">{}</td></tr>",
            b.label,
            format_tokens(b.tokens.total_tokens()),
            format_cost(b.cost.total_cost_usd),
            format_energy(b.impact.energy_wh),
            format_co2(b.impact.co2_grams),
            format_water(b.impact.water_ml),
            format_trees(b.impact.trees_destroyed),
            guilt_class,
            b.guilt.title,
        )?;
    }

    // Totals
    if buckets.len() > 1 {
        let total_tokens: u64 = buckets.iter().map(|b| b.tokens.total_tokens()).sum();
        let total_cost: f64 = buckets.iter().map(|b| b.cost.total_cost_usd).sum();
        let total_energy: f64 = buckets.iter().map(|b| b.impact.energy_wh).sum();
        let total_co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
        let total_water: f64 = buckets.iter().map(|b| b.impact.water_ml).sum();
        let total_trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();

        writeln!(
            writer,
            "<tr class=\"total\"><td>TOTAL</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td></td></tr>",
            format_tokens(total_tokens),
            format_cost(total_cost),
            format_energy(total_energy),
            format_co2(total_co2),
            format_water(total_water),
            format_trees(total_trees),
        )?;
    }

    writeln!(writer, "</tbody>\n</table>")?;

    // SVG bar chart
    if max_co2 > 0.0 {
        writeln!(writer, "<h2>CO2 Emissions by Period</h2>")?;
        for b in buckets {
            let pct = (b.impact.co2_grams / max_co2 * 100.0).max(0.5);
            let color = match b.guilt.level {
                crate::models::GuiltLevel::DigitalSaint => "#00cc00",
                crate::models::GuiltLevel::CarbonCurious => "#00cccc",
                crate::models::GuiltLevel::TreeTrimmer => "#cccc00",
                crate::models::GuiltLevel::ForestFlattener => "#ff8800",
                crate::models::GuiltLevel::EcoTerrorist => "#ff0000",
                crate::models::GuiltLevel::PlanetIncinerator => "#cc0000",
                crate::models::GuiltLevel::HeatDeathAccelerator => "#cc00cc",
            };
            writeln!(
                writer,
                "<div class=\"chart-row\"><span class=\"chart-label\">{}</span><span class=\"bar\" style=\"width:{}%;background:{};\"></span><span class=\"chart-value\">{}</span></div>",
                b.label, pct, color, format_co2(b.impact.co2_grams),
            )?;
        }
    }

    writeln!(writer, "<div class=\"footer\">Sources: Jegham et al. 2025, EPA eGRID 2024, Li et al. 2023, Luccioni et al. 2023, USDA Forestry, IEA 2024</div>")?;
    writeln!(writer, "</body>\n</html>")?;
    Ok(())
}

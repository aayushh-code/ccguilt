use colored::*;

use crate::models::{GuiltLevel, UsageBucket};

pub fn render_chart(buckets: &[UsageBucket]) {
    if buckets.is_empty() {
        return;
    }

    render_bar_chart(
        buckets,
        "CO2 Emissions by Period",
        |b| b.impact.co2_grams,
        format_co2_short,
    );

    render_bar_chart(
        buckets,
        "Water Usage by Period",
        |b| b.impact.water_ml,
        format_water_short,
    );

    // Shared legend
    println!();
    print!("  ");
    let levels = [
        ("Saint", Color::Green),
        ("Curious", Color::Cyan),
        ("Trimmer", Color::Yellow),
        ("Flattener", Color::TrueColor { r: 255, g: 165, b: 0 }),
        ("Terrorist", Color::Red),
        ("Incinerator", Color::TrueColor { r: 139, g: 0, b: 0 }),
        ("HeatDeath", Color::Magenta),
    ];
    for (name, color) in levels {
        print!("{} ", "\u{2588}".color(color));
        print!("{} ", name.dimmed());
    }
    println!();
}

fn render_bar_chart(
    buckets: &[UsageBucket],
    title: &str,
    value_fn: fn(&UsageBucket) -> f64,
    format_fn: fn(f64) -> String,
) {
    let max_val = buckets.iter().map(|b| value_fn(b)).fold(0.0_f64, f64::max);

    if max_val == 0.0 {
        return;
    }

    let bar_max_width: usize = 40;
    let label_width = buckets.iter().map(|b| b.label.len()).max().unwrap_or(10);

    println!();
    println!("  {}", title.bold().underline());
    println!();

    for bucket in buckets {
        let val = value_fn(bucket);
        let ratio = val / max_val;
        let bar_len = (ratio * bar_max_width as f64).round() as usize;
        let bar_len = bar_len.max(if val > 0.0 { 1 } else { 0 });

        let bar_char = "\u{2588}";
        let bar = bar_char.repeat(bar_len);

        let colored_bar = match bucket.guilt.level {
            GuiltLevel::DigitalSaint => bar.green(),
            GuiltLevel::CarbonCurious => bar.cyan(),
            GuiltLevel::TreeTrimmer => bar.yellow(),
            GuiltLevel::ForestFlattener => bar.truecolor(255, 165, 0),
            GuiltLevel::EcoTerrorist => bar.red(),
            GuiltLevel::PlanetIncinerator => bar.truecolor(139, 0, 0),
            GuiltLevel::HeatDeathAccelerator => bar.magenta(),
        };

        println!(
            "  {:>width$} {} {}",
            bucket.label.dimmed(),
            colored_bar,
            format_fn(val).dimmed(),
            width = label_width
        );
    }
}

fn format_co2_short(grams: f64) -> String {
    if grams >= 1_000_000.0 {
        format!("{:.1}t", grams / 1_000_000.0)
    } else if grams >= 1000.0 {
        format!("{:.1}kg", grams / 1000.0)
    } else if grams >= 1.0 {
        format!("{:.0}g", grams)
    } else {
        format!("{:.1}mg", grams * 1000.0)
    }
}

fn format_water_short(ml: f64) -> String {
    if ml >= 1_000_000.0 {
        format!("{:.1}m\u{00B3}", ml / 1_000_000.0)
    } else if ml >= 1000.0 {
        format!("{:.1}L", ml / 1000.0)
    } else if ml >= 1.0 {
        format!("{:.0}mL", ml)
    } else {
        format!("{:.1}\u{00B5}L", ml * 1000.0)
    }
}

use chrono::{Datelike, Duration, Local, NaiveDate};
use colored::Colorize;
use std::collections::HashMap;

use crate::display::format;
use crate::models::UsageBucket;

pub fn render_heatmap(buckets: &[UsageBucket], weeks: usize) {
    let today = Local::now().date_naive();
    let start = today - Duration::weeks(weeks as i64);

    // Build date -> CO2 map from daily buckets
    let mut day_co2: HashMap<NaiveDate, f64> = HashMap::new();
    for b in buckets {
        if let Ok(d) = NaiveDate::parse_from_str(&b.label, "%Y-%m-%d") {
            *day_co2.entry(d).or_default() += b.impact.co2_grams;
        }
    }

    let max_co2 = day_co2.values().cloned().fold(0.0_f64, f64::max);
    if max_co2 == 0.0 {
        println!("  No daily data for heatmap.");
        return;
    }

    println!();
    println!("  {}", "CO2 Heatmap (daily emissions)".bold().underline());
    println!();
    println!("  {:>10} Mon Tue Wed Thu Fri Sat Sun", "");

    // Align to Monday of the start week
    let start_weekday = start.weekday().num_days_from_monday();
    let first_monday = start - Duration::days(start_weekday as i64);

    let mut current = first_monday;
    while current <= today {
        // Week label
        let week_label = current.format("%b %d").to_string();
        print!("  {:>10} ", week_label.dimmed());

        for dow in 0..7 {
            let day = current + Duration::days(dow);
            if day < start || day > today {
                print!(" ·  ");
            } else if let Some(&co2) = day_co2.get(&day) {
                let intensity = (co2 / max_co2 * 4.0).round() as usize;
                let ch = match intensity.min(4) {
                    0 => "\u{2591}", // ░
                    1 => "\u{2592}", // ▒
                    2 => "\u{2593}", // ▓
                    3 => "\u{2588}", // █
                    _ => "\u{2588}", // █
                };
                let colored = match intensity.min(4) {
                    0 => format!(" {} ", ch).green().to_string(),
                    1 => format!(" {} ", ch).yellow().to_string(),
                    2 => format!(" {} ", ch).truecolor(255, 165, 0).to_string(),
                    _ => format!(" {} ", ch).red().to_string(),
                };
                print!("{} ", colored);
            } else {
                print!(" \u{00B7}  ");
            }
        }
        println!();

        current += Duration::weeks(1);
    }

    // Legend
    println!();
    println!(
        "  {:>10} {} {} {} {} {}",
        "",
        "\u{00B7} None".dimmed(),
        "\u{2591} Low".green(),
        "\u{2592} Medium".yellow(),
        "\u{2593} High".truecolor(255, 165, 0),
        "\u{2588} Peak".red(),
    );

    // Summary
    let total_co2: f64 = day_co2.values().sum();
    let active_days = day_co2.len();
    println!();
    println!(
        "  {} active days | Total: {} | Daily avg: {}",
        active_days,
        format::format_co2(total_co2),
        format::format_co2(total_co2 / active_days.max(1) as f64),
    );
}

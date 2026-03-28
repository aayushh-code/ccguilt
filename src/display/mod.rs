pub mod guilt;
pub mod json;
pub mod table;

use colored::Colorize;

use crate::data::discovery::ClaudeDataDir;
use crate::models::UsageBucket;

pub fn print_header() {
    let border = "=".repeat(66);
    println!();
    println!("{}", border.bright_red().bold());
    println!("{}", "  CLAUDE CODE GUILT TRIP".bright_red().bold());
    println!(
        "{}",
        "  An environmental impact report nobody asked for".red()
    );
    println!("{}", border.bright_red().bold());
    println!();
}

pub fn print_metadata(
    data_dir: &ClaudeDataDir,
    file_count: usize,
    project_filter: Option<&str>,
    fast: bool,
) {
    let source = if fast {
        "Fast scan (stats-cache.json)".yellow().to_string()
    } else {
        format!(
            "Deep scan of {} session files across {} projects",
            file_count,
            data_dir.project_count()
        )
        .green()
        .to_string()
    };

    println!("  {}: {}", "Data".bold(), source);
    if let Some(filter) = project_filter {
        println!("  {}: {}", "Project filter".bold(), filter);
    }
    println!();
}

pub fn print_summary_footer(buckets: &[UsageBucket], no_guilt: bool) {
    if no_guilt || buckets.is_empty() {
        return;
    }

    // Find overall totals
    let total_co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
    let total_water: f64 = buckets.iter().map(|b| b.impact.water_ml).sum();
    let total_energy: f64 = buckets.iter().map(|b| b.impact.energy_wh).sum();
    let total_trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();

    let total_impact = crate::models::ImpactSummary {
        energy_wh: total_energy,
        co2_grams: total_co2,
        water_ml: total_water,
        trees_destroyed: total_trees,
        trees_dehydrated: buckets.iter().map(|b| b.impact.trees_dehydrated).sum(),
        netflix_hours_equiv: buckets.iter().map(|b| b.impact.netflix_hours_equiv).sum(),
    };

    // Tree progress bar
    println!();
    println!("{}", guilt::tree_progress_bar(total_trees));

    // Satirical comparisons
    println!();
    let comparisons = guilt::generate_comparisons(&total_impact);
    for (i, comp) in comparisons.iter().enumerate() {
        if i >= 3 {
            break; // Max 3 comparisons
        }
        println!("  {}", comp);
    }

    // Random guilt quote
    println!();
    let separator = "-".repeat(66);
    println!("{}", separator.dimmed());
    println!();
    let quote = guilt::random_quote();
    println!("  {}", quote.italic().dimmed());

    // Nihilistic / absurdist remark based on overall guilt level
    let overall_guilt = crate::calc::impact::determine_guilt(&total_impact);
    let remark = guilt::random_remark(overall_guilt.level);
    println!();
    println!("  {}", remark.italic().bright_black());

    // Sources
    println!();
    println!(
        "  {}",
        "Sources: Jegham et al. 2025, EPA eGRID 2024, Li et al. 2023,".dimmed()
    );
    println!(
        "  {}",
        "  Luccioni et al. 2023, USDA Forestry, IEA 2024".dimmed()
    );
    println!();
}

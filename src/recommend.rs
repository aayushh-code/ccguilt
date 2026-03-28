use colored::Colorize;

use crate::calc::cost::calculate_model_cost;
use crate::calc::impact::calculate_impact_with;
#[allow(unused_imports)]
use crate::config;
use crate::display::format;
use crate::models::{ModelTier, ModelTokens, TokenSummary, UsageBucket};

pub fn print_recommendations(buckets: &[UsageBucket], co2_kg_per_kwh: f64, pue: f64) {
    // Aggregate all tokens by model
    let mut total_by_model: std::collections::HashMap<ModelTier, ModelTokens> =
        std::collections::HashMap::new();
    for b in buckets {
        for (tier, tokens) in &b.tokens.by_model {
            let entry = total_by_model.entry(*tier).or_default();
            entry.input_tokens += tokens.input_tokens;
            entry.output_tokens += tokens.output_tokens;
            entry.cache_creation_tokens += tokens.cache_creation_tokens;
            entry.cache_read_tokens += tokens.cache_read_tokens;
        }
    }

    let alternatives = [
        (ModelTier::Opus, ModelTier::Sonnet),
        (ModelTier::Opus, ModelTier::Haiku),
        (ModelTier::Sonnet, ModelTier::Haiku),
    ];

    let mut any_recs = false;

    println!();
    println!("  {}", "Model Recommendations:".bold().underline());
    println!();

    for (from, to) in &alternatives {
        let tokens = match total_by_model.get(from) {
            Some(t) => t,
            None => continue,
        };

        let total = tokens.input_tokens + tokens.output_tokens
            + tokens.cache_creation_tokens + tokens.cache_read_tokens;
        if total == 0 {
            continue;
        }

        let actual_cost = calculate_model_cost(tokens, *from);
        let alt_cost = calculate_model_cost(tokens, *to);

        // Build synthetic TokenSummary for impact calc
        let make_summary = |tier: ModelTier| -> TokenSummary {
            let mut by_model = std::collections::HashMap::new();
            by_model.insert(tier, tokens.clone());
            TokenSummary {
                input_tokens: tokens.input_tokens,
                output_tokens: tokens.output_tokens,
                cache_creation_tokens: tokens.cache_creation_tokens,
                cache_read_tokens: tokens.cache_read_tokens,
                by_model,
            }
        };

        let actual_impact = calculate_impact_with(&make_summary(*from), co2_kg_per_kwh, pue);
        let alt_impact = calculate_impact_with(&make_summary(*to), co2_kg_per_kwh, pue);

        let cost_save = actual_cost.total_cost_usd - alt_cost.total_cost_usd;
        let co2_save = actual_impact.co2_grams - alt_impact.co2_grams;
        let cost_pct = if actual_cost.total_cost_usd > 0.0 {
            cost_save / actual_cost.total_cost_usd * 100.0
        } else {
            0.0
        };
        let co2_pct = if actual_impact.co2_grams > 0.0 {
            co2_save / actual_impact.co2_grams * 100.0
        } else {
            0.0
        };

        if cost_save > 0.01 {
            println!(
                "  {} {} {}  Save {} ({:.0}%) and {} CO2 ({:.0}%)",
                format!("{}", from).bold(),
                "\u{2192}".dimmed(),
                format!("{}", to).green().bold(),
                format::format_cost(cost_save).green(),
                cost_pct,
                format::format_co2(co2_save).green(),
                co2_pct,
            );
            any_recs = true;
        }
    }

    if !any_recs {
        println!(
            "  {}",
            "Already using the cheapest models. The planet is still unimpressed.".dimmed(),
        );
    } else {
        println!();
        println!(
            "  {}",
            "\"But sure, your variable names really need Opus-level intelligence.\""
                .italic()
                .dimmed(),
        );
    }
    println!();
}

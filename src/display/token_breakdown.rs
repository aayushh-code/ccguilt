use colored::Colorize;

use crate::models::UsageBucket;

pub fn render_token_breakdown(buckets: &[UsageBucket]) {
    let input: u64 = buckets.iter().map(|b| b.tokens.input_tokens).sum();
    let output: u64 = buckets.iter().map(|b| b.tokens.output_tokens).sum();
    let cache_create: u64 = buckets.iter().map(|b| b.tokens.cache_creation_tokens).sum();
    let cache_read: u64 = buckets.iter().map(|b| b.tokens.cache_read_tokens).sum();
    let total = input + output + cache_create + cache_read;

    if total == 0 {
        return;
    }

    let bar_width: usize = 40;
    let input_pct = input as f64 / total as f64;
    let output_pct = output as f64 / total as f64;
    let cache_create_pct = cache_create as f64 / total as f64;
    let cache_read_pct = cache_read as f64 / total as f64;

    let input_w = (input_pct * bar_width as f64).round() as usize;
    let output_w = (output_pct * bar_width as f64).round() as usize;
    let cache_create_w = (cache_create_pct * bar_width as f64).round() as usize;
    let cache_read_w = bar_width.saturating_sub(input_w + output_w + cache_create_w);

    let bar = format!(
        "[{}{}{}{}]",
        "\u{2588}".repeat(input_w),
        "\u{2591}".repeat(output_w),
        "\u{2593}".repeat(cache_create_w),
        "\u{2592}".repeat(cache_read_w),
    );

    println!();
    println!(
        "  {} {}",
        "Token mix:".bold(),
        bar,
    );
    println!(
        "    {} {:.0}%  {} {:.0}%  {} {:.0}%  {} {:.0}%",
        "\u{2588} Input".green(),
        input_pct * 100.0,
        "\u{2591} Output".yellow(),
        output_pct * 100.0,
        "\u{2593} Cache Write".cyan(),
        cache_create_pct * 100.0,
        "\u{2592} Cache Read".dimmed(),
        cache_read_pct * 100.0,
    );
}

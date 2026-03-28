mod aggregate;
mod calc;
mod cli;
mod config;
mod data;
mod display;
mod models;

use anyhow::Result;
use chrono::{DateTime, NaiveDate, Utc};
use clap::Parser;
use colored::Colorize;

use cli::{Args, Period};
use data::discovery::ClaudeDataDir;

fn parse_date_arg(s: &str) -> Result<DateTime<Utc>> {
    let naive = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("Invalid date '{}': {}. Use YYYY-MM-DD format.", s, e))?;
    Ok(naive.and_hms_opt(0, 0, 0).unwrap().and_utc())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let claude_home = match &args.claude_home {
        Some(p) => p.clone(),
        None => ClaudeDataDir::default_path()?,
    };

    if !claude_home.exists() {
        eprintln!(
            "{} No Claude Code data directory found at {}",
            "Error:".red().bold(),
            claude_home.display()
        );
        eprintln!("Are you sure you've used Claude Code? Lucky planet.");
        std::process::exit(1);
    }

    let data_dir = ClaudeDataDir::new(claude_home);

    let since = args.since.as_deref().map(parse_date_arg).transpose()?;
    let until = args.until.as_deref().map(parse_date_arg).transpose()?;

    let records = if args.fast {
        // Fast path: read stats-cache.json
        let cache_path = data_dir.stats_cache_path();
        if !cache_path.exists() {
            eprintln!(
                "{} stats-cache.json not found. Run Claude Code first, or use deep scan (remove --fast).",
                "Error:".red().bold()
            );
            std::process::exit(1);
        }

        if args.period == Period::Session {
            eprintln!(
                "{} Session-level breakdown not available in fast mode.",
                "Warning:".yellow().bold()
            );
            eprintln!("Use deep scan (remove --fast) for per-session data.");
            std::process::exit(1);
        }

        eprintln!("  {} Reading stats-cache.json...", ">>".yellow().bold());
        let fast_data = data::cache::parse_stats_cache(&cache_path)?;

        match args.period {
            Period::Total => aggregate::fast_path_total(&fast_data.model_usage),
            _ => aggregate::fast_path_daily(&fast_data.daily_tokens, &fast_data.model_usage),
        }
    } else {
        // Deep scan: parse all JSONL files
        let files = data_dir.jsonl_files(args.project.as_deref());

        if files.is_empty() {
            eprintln!(
                "{} No session files found in {}",
                "Error:".red().bold(),
                data_dir.projects_dir().display()
            );
            eprintln!("Your Claude data directory exists but contains no sessions.");
            eprintln!("Either you're a Digital Saint or something is misconfigured.");
            std::process::exit(1);
        }

        eprintln!(
            "  {} Deep scan: parsing {} session files...",
            ">>".green().bold(),
            files.len()
        );

        let records =
            data::jsonl::parse_jsonl_files(&files, since, until, args.project.as_deref())?;

        // Print scan complete
        let total_records: usize = records.len();
        eprintln!(
            "  {} Found {} token records",
            ">>".green().bold(),
            total_records
        );

        if records.is_empty() {
            eprintln!();
            eprintln!("No data found in the specified range. The planet thanks you.");
            return Ok(());
        }

        records
    };

    if records.is_empty() {
        eprintln!("No data found. Either you're a Digital Saint or your Claude data is elsewhere.");
        return Ok(());
    }

    // Aggregate by period
    let buckets = aggregate::aggregate(records, args.period);

    // Display
    if args.json {
        let json = display::json::render_json(&buckets)?;
        println!("{json}");
    } else {
        let file_count = if args.fast {
            1
        } else {
            data_dir.jsonl_files(args.project.as_deref()).len()
        };

        display::print_header();
        display::print_metadata(&data_dir, file_count, args.project.as_deref(), args.fast);
        display::table::render_table(&buckets, args.no_guilt);
        display::print_summary_footer(&buckets, args.no_guilt);
    }

    Ok(())
}

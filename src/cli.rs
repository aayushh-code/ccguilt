use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "ccguilt",
    about = "Claude Code Guilt Trip — see what your AI habit is doing to the planet",
    long_about = "A satirical environmental impact tracker for Claude Code usage.\n\
                  Reads your local Claude Code data and computes energy, CO2, water,\n\
                  and tree destruction metrics. You monster.\n\n\
                  Sources: Jegham et al. 2025, Luccioni et al. 2023, Li et al. 2023,\n\
                  EPA eGRID 2024, USDA Forestry",
    version
)]
pub struct Args {
    /// Report grouping period
    #[arg(value_enum, default_value = "daily")]
    pub period: Period,

    /// Start date filter (YYYY-MM-DD)
    #[arg(long)]
    pub since: Option<String>,

    /// End date filter (YYYY-MM-DD)
    #[arg(long)]
    pub until: Option<String>,

    /// Output as JSON instead of a table
    #[arg(long)]
    pub json: bool,

    /// Use stats-cache.json for faster but less accurate results
    #[arg(long)]
    pub fast: bool,

    /// Show a bar chart of CO2 emissions per period
    #[arg(long)]
    pub chart: bool,

    /// Hide satirical commentary (coward mode)
    #[arg(long)]
    pub no_guilt: bool,

    /// Filter by project path (substring match)
    #[arg(long)]
    pub project: Option<String>,

    /// Custom Claude data directory (default: ~/.claude)
    #[arg(long, env = "CLAUDE_HOME")]
    pub claude_home: Option<PathBuf>,
}

#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Session,
    Total,
}

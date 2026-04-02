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

    // ── Date & Filter ──
    /// Start date filter (YYYY-MM-DD, 7d, 2w, last-week, yesterday, monday, etc.)
    #[arg(long)]
    pub since: Option<String>,

    /// End date filter (same formats as --since)
    #[arg(long)]
    pub until: Option<String>,

    /// Filter by project path (substring match)
    #[arg(long, group = "project_filter")]
    pub project: Option<String>,

    /// Filter by project path (regex pattern)
    #[arg(long, group = "project_filter")]
    pub project_regex: Option<String>,

    // ── Data Mode ──
    /// Use stats-cache.json for faster but less accurate results
    #[arg(long)]
    pub fast: bool,

    /// Custom Claude data directory (default: ~/.claude)
    #[arg(long, env = "CLAUDE_HOME")]
    pub claude_home: Option<PathBuf>,

    // ── Analysis ──
    /// Show per-model token breakdown within each period
    #[arg(long)]
    pub by_model: bool,

    /// Sort periods by metric (default: chronological)
    #[arg(long, value_enum)]
    pub sort: Option<SortField>,

    /// Show only the top N periods
    #[arg(long)]
    pub top: Option<usize>,

    /// Group by dimension instead of time period
    #[arg(long, value_enum)]
    pub group_by: Option<GroupBy>,

    /// Show efficiency metrics ($/Mtok, gCO2/Mtok)
    #[arg(long)]
    pub efficiency: bool,

    /// Show cumulative running totals
    #[arg(long)]
    pub cumulative: bool,

    /// Hide periods below this CO2 threshold (grams)
    #[arg(long)]
    pub min_co2: Option<f64>,

    /// Hide periods below this cost threshold (USD)
    #[arg(long)]
    pub min_cost: Option<f64>,

    /// Carbon budget (e.g., "50kg", "5000g", "1t") — shows progress toward limit
    #[arg(long)]
    pub budget: Option<String>,

    /// Compare projects side-by-side (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub compare: Option<Vec<String>>,

    // ── Output Format ──
    /// Output as JSON instead of a table
    #[arg(long, group = "output_format")]
    pub json: bool,

    /// Output as CSV
    #[arg(long, group = "output_format")]
    pub csv: bool,

    /// Output as Markdown
    #[arg(long, group = "output_format")]
    pub markdown: bool,

    /// Output as standalone HTML report to file
    #[arg(long)]
    pub html: Option<PathBuf>,

    /// Write output to file (format auto-detected from extension: .csv, .json, .html, .md)
    #[arg(long)]
    pub output: Option<PathBuf>,

    /// Show a bar chart of CO2/water emissions per period
    #[arg(long)]
    pub chart: bool,

    /// Show sparklines in table/footer
    #[arg(long)]
    pub sparkline: bool,

    // ── Display Control ──
    /// Hide satirical commentary (coward mode)
    #[arg(long)]
    pub no_guilt: bool,

    /// Disable colored output (also respects NO_COLOR env var)
    #[arg(long)]
    pub no_color: bool,

    /// Quiet mode: suppress progress messages on stderr
    #[arg(short, long, group = "verbosity")]
    pub quiet: bool,

    /// Verbose mode: show per-file parsing details
    #[arg(short, long, group = "verbosity")]
    pub verbose: bool,

    // ── Modes ──
    /// Launch interactive TUI mode
    #[arg(short, long)]
    pub interactive: bool,

    /// Watch mode: re-run every N seconds (default: 30)
    #[arg(long, default_missing_value = "30", num_args = 0..=1)]
    pub watch: Option<u64>,

    // ── Utility ──
    /// Generate shell completions (bash, zsh, fish, elvish, powershell)
    #[arg(long)]
    pub completions: Option<clap_complete::Shell>,

    /// Install shell completions to the appropriate system location
    #[arg(long, default_missing_value = "auto", num_args = 0..=1)]
    pub setup_completions: Option<String>,

    /// Check for updates and self-update if available
    #[arg(long)]
    pub increase_guilt: bool,

    // ── New Features ──
    /// Compare two time periods (e.g., --diff last-week this-week)
    #[arg(long, num_args = 2)]
    pub diff: Option<Vec<String>>,

    /// Show model cost/CO2 optimization recommendations
    #[arg(long)]
    pub recommend: bool,

    /// Show the Hall of Shame (all achievements)
    #[arg(long)]
    pub achievements: bool,

    /// Show carbon offset options
    #[arg(long)]
    pub offset: bool,

    /// List all projects ranked by environmental impact
    #[arg(long)]
    pub projects: bool,

    /// Show detailed timeline for a session (substring match on ID)
    #[arg(long)]
    pub session: Option<String>,

    /// Output a single compact line (for git hooks)
    #[arg(long, group = "output_format")]
    pub hook_output: bool,

    /// Show a calendar heatmap of daily CO2 emissions
    #[arg(long)]
    pub heatmap: bool,

    /// Skip the SQLite cache, parse JSONL files directly
    #[arg(long)]
    pub no_db: bool,

    /// Rebuild the SQLite database from scratch
    #[arg(long)]
    pub rebuild_db: bool,
}

#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Session,
    Total,
}

#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum SortField {
    Co2,
    Cost,
    Tokens,
    Energy,
    Water,
}

#[derive(ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum GroupBy {
    Project,
    Model,
}

use crate::cli::Args;
use crate::config;
use crate::config_file::UserConfig;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RuntimeConfig {
    pub co2_kg_per_kwh: f64,
    pub pue: f64,
    pub no_color: bool,
    pub no_guilt: bool,
    pub verbose: bool,
    pub quiet: bool,
    pub budget_co2_grams: Option<f64>,
    pub region: Option<String>,
    pub sparklines: bool,
    pub trends: bool,
}

impl RuntimeConfig {
    pub fn from_args_and_config(args: &Args, cfg: &UserConfig) -> Self {
        Self {
            co2_kg_per_kwh: cfg
                .environment
                .grid_co2_kg_per_kwh
                .unwrap_or(config::CO2_KG_PER_KWH),
            pue: cfg.environment.pue.unwrap_or(config::PUE),
            no_color: args.no_color
                || std::env::var("NO_COLOR").is_ok()
                || cfg.defaults.no_color.unwrap_or(false),
            no_guilt: args.no_guilt || cfg.defaults.no_guilt.unwrap_or(false),
            verbose: args.verbose,
            quiet: args.quiet,
            budget_co2_grams: args
                .budget
                .as_ref()
                .and_then(|b| crate::dateparse::parse_co2_budget(b).ok())
                .or(cfg.budget.co2_grams),
            region: cfg.environment.region.clone(),
            sparklines: args.sparkline || cfg.display.sparklines.unwrap_or(false),
            trends: cfg.display.trends.unwrap_or(false),
        }
    }
}

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct UserConfig {
    #[serde(default)]
    pub defaults: DefaultsConfig,
    #[serde(default)]
    pub environment: EnvironmentConfig,
    #[serde(default)]
    pub display: DisplayConfig,
    #[serde(default)]
    pub budget: BudgetConfig,
}

#[derive(Debug, Default, Deserialize, Clone)]
#[allow(dead_code)]
pub struct DefaultsConfig {
    pub period: Option<String>,
    pub no_guilt: Option<bool>,
    pub no_color: Option<bool>,
    pub fast: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct EnvironmentConfig {
    pub grid_co2_kg_per_kwh: Option<f64>,
    pub pue: Option<f64>,
    pub region: Option<String>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct DisplayConfig {
    pub sparklines: Option<bool>,
    pub trends: Option<bool>,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub struct BudgetConfig {
    pub co2_grams: Option<f64>,
}

pub fn config_path() -> Option<PathBuf> {
    dirs::config_dir().map(|d| d.join("ccguilt").join("config.toml"))
}

pub fn load_config() -> UserConfig {
    match config_path() {
        Some(p) if p.exists() => std::fs::read_to_string(&p)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default(),
        _ => UserConfig::default(),
    }
}

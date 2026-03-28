use chrono::{DateTime, Utc};
use indexmap::IndexMap;
use serde::Serialize;
use std::collections::HashMap;

/// One assistant message's token usage from a JSONL line
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TokenRecord {
    pub timestamp: DateTime<Utc>,
    pub session_id: String,
    pub project_name: String,
    pub model: ModelTier,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_input_tokens: u64,
    pub cache_read_input_tokens: u64,
}

/// Normalized model tier for pricing and energy calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum ModelTier {
    Opus,
    Sonnet,
    Haiku,
    Unknown,
}

impl ModelTier {
    pub fn from_model_string(s: &str) -> Option<Self> {
        if s == "<synthetic>" {
            return None;
        }
        let s_lower = s.to_lowercase();
        if s_lower.contains("opus") {
            Some(ModelTier::Opus)
        } else if s_lower.contains("sonnet") {
            Some(ModelTier::Sonnet)
        } else if s_lower.contains("haiku") {
            Some(ModelTier::Haiku)
        } else {
            Some(ModelTier::Unknown)
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ModelTier::Opus => "Opus",
            ModelTier::Sonnet => "Sonnet",
            ModelTier::Haiku => "Haiku",
            ModelTier::Unknown => "Unknown",
        }
    }
}

impl std::fmt::Display for ModelTier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

/// Aggregated token usage for a time bucket
#[derive(Debug, Clone, Default, Serialize)]
pub struct UsageBucket {
    pub label: String,
    pub tokens: TokenSummary,
    pub cost: CostSummary,
    pub impact: ImpactSummary,
    pub guilt: GuiltRating,
    pub models_used: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct TokenSummary {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
    #[serde(skip)]
    pub by_model: HashMap<ModelTier, ModelTokens>,
}

impl TokenSummary {
    pub fn total_tokens(&self) -> u64 {
        self.input_tokens + self.output_tokens + self.cache_creation_tokens + self.cache_read_tokens
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ModelTokens {
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub cache_creation_tokens: u64,
    pub cache_read_tokens: u64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CostSummary {
    pub input_cost_usd: f64,
    pub output_cost_usd: f64,
    pub cache_read_cost_usd: f64,
    pub cache_creation_cost_usd: f64,
    pub total_cost_usd: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct ImpactSummary {
    pub energy_wh: f64,
    pub co2_grams: f64,
    pub water_ml: f64,
    pub trees_destroyed: f64,
    pub trees_dehydrated: f64,
    pub netflix_hours_equiv: f64,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GuiltRating {
    pub level: GuiltLevel,
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize)]
pub enum GuiltLevel {
    #[default]
    DigitalSaint,
    CarbonCurious,
    TreeTrimmer,
    ForestFlattener,
    EcoTerrorist,
    PlanetIncinerator,
    HeatDeathAccelerator,
}

/// Fast-path data from stats-cache.json
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FastPathData {
    pub model_usage: IndexMap<String, CacheModelUsage>,
    pub daily_tokens: Vec<CacheDailyTokens>,
    pub total_sessions: u64,
    pub total_messages: u64,
    pub first_session_date: Option<String>,
}

#[derive(Debug, Clone, Default, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheModelUsage {
    #[serde(default)]
    pub input_tokens: u64,
    #[serde(default)]
    pub output_tokens: u64,
    #[serde(default)]
    pub cache_read_input_tokens: u64,
    #[serde(default)]
    pub cache_creation_input_tokens: u64,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheDailyTokens {
    pub date: String,
    pub tokens_by_model: HashMap<String, u64>,
}

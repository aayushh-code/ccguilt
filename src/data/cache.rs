use anyhow::Result;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

use crate::models::{CacheDailyTokens, CacheModelUsage, FastPathData};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StatsCache {
    #[serde(default)]
    model_usage: HashMap<String, CacheModelUsage>,
    #[serde(default)]
    daily_model_tokens: Vec<CacheDailyTokens>,
    #[serde(default)]
    total_sessions: u64,
    #[serde(default)]
    total_messages: u64,
    #[serde(default)]
    first_session_date: Option<String>,
}

pub fn parse_stats_cache(path: &Path) -> Result<FastPathData> {
    let contents = std::fs::read_to_string(path)?;
    let cache: StatsCache = serde_json::from_str(&contents)?;

    // Preserve insertion order
    let mut model_usage = IndexMap::new();
    for (k, v) in cache.model_usage {
        model_usage.insert(k, v);
    }

    Ok(FastPathData {
        model_usage,
        daily_tokens: cache.daily_model_tokens,
        total_sessions: cache.total_sessions,
        total_messages: cache.total_messages,
        first_session_date: cache.first_session_date,
    })
}

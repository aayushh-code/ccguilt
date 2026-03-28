use crate::models::UsageBucket;
use anyhow::Result;

pub fn render_json(buckets: &[UsageBucket]) -> Result<String> {
    serde_json::to_string_pretty(buckets).map_err(Into::into)
}

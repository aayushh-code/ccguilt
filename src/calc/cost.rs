use crate::config::pricing_profile;
use crate::models::{CostSummary, ModelTier, ModelTokens, TokenSummary};

pub fn calculate_model_cost(tokens: &ModelTokens, tier: ModelTier) -> CostSummary {
    let pricing = pricing_profile(tier);
    let input = tokens.input_tokens as f64 * pricing.input_per_mtok / 1_000_000.0;
    let output = tokens.output_tokens as f64 * pricing.output_per_mtok / 1_000_000.0;
    let cache_read = tokens.cache_read_tokens as f64 * pricing.cache_read_per_mtok / 1_000_000.0;
    let cache_creation =
        tokens.cache_creation_tokens as f64 * pricing.cache_creation_per_mtok / 1_000_000.0;

    CostSummary {
        input_cost_usd: input,
        output_cost_usd: output,
        cache_read_cost_usd: cache_read,
        cache_creation_cost_usd: cache_creation,
        total_cost_usd: input + output + cache_read + cache_creation,
    }
}

pub fn calculate_total_cost(summary: &TokenSummary) -> CostSummary {
    let mut total = CostSummary::default();

    for (tier, model_tokens) in &summary.by_model {
        let c = calculate_model_cost(model_tokens, *tier);
        total.input_cost_usd += c.input_cost_usd;
        total.output_cost_usd += c.output_cost_usd;
        total.cache_read_cost_usd += c.cache_read_cost_usd;
        total.cache_creation_cost_usd += c.cache_creation_cost_usd;
        total.total_cost_usd += c.total_cost_usd;
    }

    total
}

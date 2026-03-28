use chrono::{Datelike, IsoWeek};
use indexmap::IndexMap;

use crate::calc::cost::calculate_total_cost;
use crate::calc::impact::{calculate_impact, determine_guilt};
use crate::cli::Period;
use crate::models::{ModelTier, TokenRecord, UsageBucket};

pub fn aggregate(records: Vec<TokenRecord>, period: Period) -> Vec<UsageBucket> {
    match period {
        Period::Daily => aggregate_by(records, |r| r.timestamp.format("%Y-%m-%d").to_string()),
        Period::Weekly => aggregate_by(records, |r| {
            let w: IsoWeek = r.timestamp.iso_week();
            format!("{}-W{:02}", w.year(), w.week())
        }),
        Period::Monthly => aggregate_by(records, |r| r.timestamp.format("%Y-%m").to_string()),
        Period::Session => aggregate_by(records, |r| r.session_id.clone()),
        Period::Total => {
            let mut bucket = UsageBucket {
                label: "All Time".to_string(),
                ..Default::default()
            };
            for record in &records {
                add_record_to_bucket(&mut bucket, record);
            }
            finalize_bucket(&mut bucket);
            vec![bucket]
        }
    }
}

fn aggregate_by<F: Fn(&TokenRecord) -> String>(
    mut records: Vec<TokenRecord>,
    key_fn: F,
) -> Vec<UsageBucket> {
    records.sort_by_key(|r| r.timestamp);

    let mut buckets: IndexMap<String, UsageBucket> = IndexMap::new();

    for record in &records {
        let key = key_fn(record);
        let bucket = buckets.entry(key.clone()).or_insert_with(|| UsageBucket {
            label: key,
            ..Default::default()
        });
        add_record_to_bucket(bucket, record);
    }

    for bucket in buckets.values_mut() {
        finalize_bucket(bucket);
    }

    buckets.into_values().collect()
}

fn add_record_to_bucket(bucket: &mut UsageBucket, record: &TokenRecord) {
    bucket.tokens.input_tokens += record.input_tokens;
    bucket.tokens.output_tokens += record.output_tokens;
    bucket.tokens.cache_creation_tokens += record.cache_creation_input_tokens;
    bucket.tokens.cache_read_tokens += record.cache_read_input_tokens;

    let model_entry = bucket.tokens.by_model.entry(record.model).or_default();
    model_entry.input_tokens += record.input_tokens;
    model_entry.output_tokens += record.output_tokens;
    model_entry.cache_creation_tokens += record.cache_creation_input_tokens;
    model_entry.cache_read_tokens += record.cache_read_input_tokens;

    // Track models used
    let model_name = record.model.display_name().to_string();
    if !bucket.models_used.contains(&model_name) {
        bucket.models_used.push(model_name);
    }
}

fn finalize_bucket(bucket: &mut UsageBucket) {
    bucket.cost = calculate_total_cost(&bucket.tokens);
    bucket.impact = calculate_impact(&bucket.tokens);
    bucket.guilt = determine_guilt(&bucket.impact);
}

/// Build TokenRecords from fast-path cache data (all-time aggregate only)
pub fn fast_path_total(
    model_usage: &indexmap::IndexMap<String, crate::models::CacheModelUsage>,
) -> Vec<TokenRecord> {
    use chrono::Utc;

    let mut records = Vec::new();
    let now = Utc::now();

    for (model_name, usage) in model_usage {
        let tier = match ModelTier::from_model_string(model_name) {
            Some(t) => t,
            None => continue,
        };

        records.push(TokenRecord {
            timestamp: now,
            session_id: "aggregate".to_string(),
            project_name: "all".to_string(),
            model: tier,
            input_tokens: usage.input_tokens,
            output_tokens: usage.output_tokens,
            cache_creation_input_tokens: usage.cache_creation_input_tokens,
            cache_read_input_tokens: usage.cache_read_input_tokens,
        });
    }

    records
}

/// Build TokenRecords from fast-path daily data
/// Since dailyModelTokens only has total tokens (not split by input/output/cache),
/// we distribute using the all-time ratios from modelUsage
pub fn fast_path_daily(
    daily_tokens: &[crate::models::CacheDailyTokens],
    model_usage: &indexmap::IndexMap<String, crate::models::CacheModelUsage>,
) -> Vec<TokenRecord> {
    let mut records = Vec::new();

    for day in daily_tokens {
        let timestamp = match chrono::NaiveDate::parse_from_str(&day.date, "%Y-%m-%d") {
            Ok(d) => d.and_hms_opt(12, 0, 0).unwrap().and_utc(),
            Err(_) => continue,
        };

        for (model_name, &day_total) in &day.tokens_by_model {
            let tier = match ModelTier::from_model_string(model_name) {
                Some(t) => t,
                None => continue,
            };

            // Get the all-time ratios for this model
            let (input_ratio, output_ratio, cache_create_ratio, cache_read_ratio) =
                if let Some(aggregate) = model_usage.get(model_name) {
                    let total = aggregate.input_tokens
                        + aggregate.output_tokens
                        + aggregate.cache_creation_input_tokens
                        + aggregate.cache_read_input_tokens;
                    if total == 0 {
                        (0.25, 0.25, 0.25, 0.25)
                    } else {
                        let t = total as f64;
                        (
                            aggregate.input_tokens as f64 / t,
                            aggregate.output_tokens as f64 / t,
                            aggregate.cache_creation_input_tokens as f64 / t,
                            aggregate.cache_read_input_tokens as f64 / t,
                        )
                    }
                } else {
                    (0.25, 0.25, 0.25, 0.25)
                };

            records.push(TokenRecord {
                timestamp,
                session_id: format!("fast-{}", day.date),
                project_name: "all".to_string(),
                model: tier,
                input_tokens: (day_total as f64 * input_ratio) as u64,
                output_tokens: (day_total as f64 * output_ratio) as u64,
                cache_creation_input_tokens: (day_total as f64 * cache_create_ratio) as u64,
                cache_read_input_tokens: (day_total as f64 * cache_read_ratio) as u64,
            });
        }
    }

    records
}

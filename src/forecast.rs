use crate::models::UsageBucket;

#[allow(dead_code)]
pub struct Forecast {
    pub annual_co2_grams: f64,
    pub annual_cost_usd: f64,
    pub annual_trees: f64,
    pub daily_avg_co2: f64,
    pub trend: TrendDirection,
}

#[derive(Debug, Clone, Copy)]
pub enum TrendDirection {
    Accelerating,
    Decelerating,
    Stable,
}

pub fn project_annual(buckets: &[UsageBucket]) -> Option<Forecast> {
    if buckets.is_empty() {
        return None;
    }

    let total_co2: f64 = buckets.iter().map(|b| b.impact.co2_grams).sum();
    let total_cost: f64 = buckets.iter().map(|b| b.cost.total_cost_usd).sum();
    let total_trees: f64 = buckets.iter().map(|b| b.impact.trees_destroyed).sum();

    let num_periods = buckets.len() as f64;
    let daily_avg_co2 = total_co2 / num_periods; // approximate: 1 bucket ~ 1 day
    let scale = 365.0 / num_periods;

    // Determine trend from first half vs second half
    let mid = buckets.len() / 2;
    let trend = if mid > 0 && buckets.len() > 2 {
        let first_half_avg: f64 =
            buckets[..mid].iter().map(|b| b.impact.co2_grams).sum::<f64>() / mid as f64;
        let second_half_avg: f64 =
            buckets[mid..].iter().map(|b| b.impact.co2_grams).sum::<f64>()
                / (buckets.len() - mid) as f64;
        let change = (second_half_avg - first_half_avg) / first_half_avg.max(1.0);
        if change > 0.15 {
            TrendDirection::Accelerating
        } else if change < -0.15 {
            TrendDirection::Decelerating
        } else {
            TrendDirection::Stable
        }
    } else {
        TrendDirection::Stable
    };

    Some(Forecast {
        annual_co2_grams: total_co2 * scale,
        annual_cost_usd: total_cost * scale,
        annual_trees: total_trees * scale,
        daily_avg_co2,
        trend,
    })
}

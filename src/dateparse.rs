use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Duration, Local, NaiveDate, Utc, Weekday};

pub fn parse_natural_date(s: &str) -> Result<DateTime<Utc>> {
    let s = s.trim().to_lowercase();
    let today = Local::now().date_naive();

    // Try ISO date first (YYYY-MM-DD)
    if let Ok(naive) = NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
        return Ok(naive.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    // Relative durations: 7d, 2w, 3m, 1y
    if s.len() >= 2 {
        let (num_part, unit) = s.split_at(s.len() - 1);
        if let Ok(n) = num_part.parse::<i64>() {
            let date = match unit {
                "d" => Some(today - Duration::days(n)),
                "w" => Some(today - Duration::weeks(n)),
                "m" => {
                    let target_month = today.month0() as i64 - n;
                    let years_back = if target_month < 0 {
                        ((-target_month - 1) / 12 + 1) as i32
                    } else {
                        0
                    };
                    let month = ((target_month % 12 + 12) % 12 + 1) as u32;
                    NaiveDate::from_ymd_opt(today.year() - years_back, month, today.day().min(28))
                }
                "y" => NaiveDate::from_ymd_opt(
                    today.year() - n as i32,
                    today.month(),
                    today.day().min(28),
                ),
                _ => None,
            };
            if let Some(d) = date {
                return Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
            }
        }
    }

    // Named ranges
    match s.as_str() {
        "today" => return Ok(today.and_hms_opt(0, 0, 0).unwrap().and_utc()),
        "yesterday" => {
            return Ok((today - Duration::days(1))
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc())
        }
        "last-week" | "lastweek" => {
            let days_since_monday = today.weekday().num_days_from_monday();
            let this_monday = today - Duration::days(days_since_monday as i64);
            let last_monday = this_monday - Duration::weeks(1);
            return Ok(last_monday.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        "this-week" | "thisweek" => {
            let days_since_monday = today.weekday().num_days_from_monday();
            let monday = today - Duration::days(days_since_monday as i64);
            return Ok(monday.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        "last-month" | "lastmonth" => {
            let (y, m) = if today.month() == 1 {
                (today.year() - 1, 12)
            } else {
                (today.year(), today.month() - 1)
            };
            let d = NaiveDate::from_ymd_opt(y, m, 1).unwrap();
            return Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        "this-month" | "thismonth" => {
            let d = NaiveDate::from_ymd_opt(today.year(), today.month(), 1).unwrap();
            return Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
        }
        _ => {}
    }

    // Day names: find most recent past occurrence
    if let Some(weekday) = parse_weekday_name(&s) {
        let current_wd = today.weekday().num_days_from_monday();
        let target_wd = weekday.num_days_from_monday();
        let days_back = if current_wd > target_wd {
            current_wd - target_wd
        } else if current_wd < target_wd {
            7 - (target_wd - current_wd)
        } else {
            7 // same day = last week
        };
        let d = today - Duration::days(days_back as i64);
        return Ok(d.and_hms_opt(0, 0, 0).unwrap().and_utc());
    }

    Err(anyhow!(
        "Unrecognized date '{}'. Use YYYY-MM-DD, 7d, 2w, 3m, yesterday, last-week, etc.",
        s
    ))
}

fn parse_weekday_name(s: &str) -> Option<Weekday> {
    match s {
        "monday" | "mon" => Some(Weekday::Mon),
        "tuesday" | "tue" => Some(Weekday::Tue),
        "wednesday" | "wed" => Some(Weekday::Wed),
        "thursday" | "thu" => Some(Weekday::Thu),
        "friday" | "fri" => Some(Weekday::Fri),
        "saturday" | "sat" => Some(Weekday::Sat),
        "sunday" | "sun" => Some(Weekday::Sun),
        _ => None,
    }
}

/// Parse a CO2 budget string like "50kg", "5000g", "1t" into grams
pub fn parse_co2_budget(s: &str) -> Result<f64> {
    let s = s.trim().to_lowercase();
    if let Some(n) = s.strip_suffix("kg") {
        Ok(n.trim().parse::<f64>()? * 1000.0)
    } else if let Some(n) = s.strip_suffix('t') {
        Ok(n.trim().parse::<f64>()? * 1_000_000.0)
    } else if let Some(n) = s.strip_suffix('g') {
        Ok(n.trim().parse::<f64>()?)
    } else {
        // Default to grams
        Ok(s.parse::<f64>()?)
    }
}

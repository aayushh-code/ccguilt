use crate::config::*;
use crate::models::{GuiltLevel, GuiltRating, ImpactSummary, TokenSummary};

pub fn calculate_impact(summary: &TokenSummary) -> ImpactSummary {
    let mut total_energy_wh = 0.0;

    for (tier, model_tokens) in &summary.by_model {
        let profile = energy_profile(*tier);

        // Input tokens: full inference energy
        total_energy_wh += model_tokens.input_tokens as f64 * profile.wh_per_input_token;

        // Output tokens: full inference energy
        total_energy_wh += model_tokens.output_tokens as f64 * profile.wh_per_output_token;

        // Cache creation: same energy as regular input (model still processes it)
        total_energy_wh += model_tokens.cache_creation_tokens as f64
            * profile.wh_per_input_token
            * profile.cache_creation_multiplier;

        // Cache read: ~10% of input energy (memory lookup, not full inference)
        total_energy_wh += model_tokens.cache_read_tokens as f64
            * profile.wh_per_input_token
            * profile.cache_read_multiplier;
    }

    // Apply PUE (data center overhead: cooling, networking, storage, etc.)
    let energy_with_pue = total_energy_wh * PUE;

    // CO2: convert Wh to kWh, multiply by grid carbon intensity
    let co2_kg = (energy_with_pue / 1000.0) * CO2_KG_PER_KWH;
    let co2_grams = co2_kg * 1000.0;

    // Water: convert Wh to kWh, multiply by WUE
    let water_liters = (energy_with_pue / 1000.0) * WATER_LITERS_PER_KWH;
    let water_ml = water_liters * 1000.0;

    // Trees destroyed (by CO2 absorption equivalent, annual)
    let trees_destroyed = co2_kg / TREE_CO2_KG_PER_YEAR;

    // Trees dehydrated (by water consumption equivalent, annual)
    let trees_dehydrated = water_liters / TREE_WATER_LITERS_PER_YEAR;

    // Netflix equivalent hours
    let netflix_hours = energy_with_pue / NETFLIX_WH_PER_HOUR;

    ImpactSummary {
        energy_wh: energy_with_pue,
        co2_grams,
        water_ml,
        trees_destroyed,
        trees_dehydrated,
        netflix_hours_equiv: netflix_hours,
    }
}

pub fn determine_guilt(impact: &ImpactSummary) -> GuiltRating {
    let co2 = impact.co2_grams;

    let (level, title, desc) = if co2 < 10.0 {
        (
            GuiltLevel::DigitalSaint,
            "Digital Saint",
            "Your carbon footprint is basically a carbon toe-print. Are you even trying?",
        )
    } else if co2 < 100.0 {
        (
            GuiltLevel::CarbonCurious,
            "Carbon Curious",
            "Dipping your toes into environmental destruction. Everyone starts somewhere.",
        )
    } else if co2 < 500.0 {
        (
            GuiltLevel::TreeTrimmer,
            "Tree Trimmer",
            "A few branches fell. The forest will recover. Probably.",
        )
    } else if co2 < 2000.0 {
        (
            GuiltLevel::ForestFlattener,
            "Forest Flattener",
            "You can hear the chainsaws from here. The squirrels are filing a class action.",
        )
    } else if co2 < 10000.0 {
        (
            GuiltLevel::EcoTerrorist,
            "Eco-Terrorist",
            "Greenpeace has entered the chat. And they brought lawyers.",
        )
    } else if co2 < 50000.0 {
        (
            GuiltLevel::PlanetIncinerator,
            "Planet Incinerator",
            "Congratulations, you've personally contributed to making Venus look hospitable.",
        )
    } else {
        (
            GuiltLevel::HeatDeathAccelerator,
            "Heat Death Accelerator",
            "The universe was going to end eventually. You're just... helping it along.",
        )
    };

    GuiltRating {
        level,
        title: title.to_string(),
        description: desc.to_string(),
    }
}

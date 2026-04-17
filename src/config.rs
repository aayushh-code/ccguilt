use crate::models::ModelTier;

/// Energy per token in Watt-hours, by model tier
/// Sources:
///   Opus: ~4.05 Wh per ~400 tokens (Jegham et al. 2025 "How Hungry is AI?")
///   Sonnet: ~0.34 Wh per ~1000 tokens (Luccioni et al. 2023 "Power Hungry Processing")
///   Haiku: ~0.22 Wh per ~1000 tokens (Luccioni et al. 2023)
pub struct EnergyProfile {
    pub wh_per_input_token: f64,
    pub wh_per_output_token: f64,
    pub cache_read_multiplier: f64,
    pub cache_creation_multiplier: f64,
}

pub const OPUS_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.010125,
    wh_per_output_token: 0.010125,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const SONNET_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00034,
    wh_per_output_token: 0.00034,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const HAIKU_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00022,
    wh_per_output_token: 0.00022,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const GLM5_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00030,
    wh_per_output_token: 0.00030,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const GLM47_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00040,
    wh_per_output_token: 0.00040,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const DEEPSEEK_R1_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00053,
    wh_per_output_token: 0.00053,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub const UNKNOWN_ENERGY: EnergyProfile = EnergyProfile {
    wh_per_input_token: 0.00034,
    wh_per_output_token: 0.00034,
    cache_read_multiplier: 0.10,
    cache_creation_multiplier: 1.0,
};

pub fn energy_profile(tier: ModelTier) -> &'static EnergyProfile {
    match tier {
        ModelTier::Opus => &OPUS_ENERGY,
        ModelTier::Sonnet => &SONNET_ENERGY,
        ModelTier::Haiku => &HAIKU_ENERGY,
        ModelTier::Glm5 => &GLM5_ENERGY,
        ModelTier::Glm47 => &GLM47_ENERGY,
        ModelTier::DeepSeekReasoner => &DEEPSEEK_R1_ENERGY,
        ModelTier::Unknown => &UNKNOWN_ENERGY,
    }
}

/// Pricing per million tokens (USD) — Anthropic API pricing
/// Source: https://docs.anthropic.com/en/docs/about-claude/models (verified against LiteLLM)
pub struct PricingProfile {
    pub input_per_mtok: f64,
    pub output_per_mtok: f64,
    pub cache_read_per_mtok: f64,
    pub cache_creation_per_mtok: f64,
}

/// Claude Opus 4.5/4.6 pricing (NOT Claude 3 Opus which was $15/$75)
pub const OPUS_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 5.0,
    output_per_mtok: 25.0,
    cache_read_per_mtok: 0.50,
    cache_creation_per_mtok: 6.25,
};

pub const SONNET_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 3.0,
    output_per_mtok: 15.0,
    cache_read_per_mtok: 0.30,
    cache_creation_per_mtok: 3.75,
};

/// Claude Haiku 4.5 pricing (NOT Claude 3 Haiku which was $0.80/$4)
pub const HAIKU_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 1.0,
    output_per_mtok: 5.0,
    cache_read_per_mtok: 0.10,
    cache_creation_per_mtok: 1.25,
};

pub const GLM5_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 0.50,
    output_per_mtok: 1.50,
    cache_read_per_mtok: 0.05,
    cache_creation_per_mtok: 0.63,
};

pub const GLM47_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 0.70,
    output_per_mtok: 2.00,
    cache_read_per_mtok: 0.07,
    cache_creation_per_mtok: 0.88,
};

pub const DEEPSEEK_R1_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 0.70,
    output_per_mtok: 2.50,
    cache_read_per_mtok: 0.14,
    cache_creation_per_mtok: 0.88,
};

pub const UNKNOWN_PRICING: PricingProfile = PricingProfile {
    input_per_mtok: 3.0,
    output_per_mtok: 15.0,
    cache_read_per_mtok: 0.30,
    cache_creation_per_mtok: 3.75,
};

pub fn pricing_profile(tier: ModelTier) -> &'static PricingProfile {
    match tier {
        ModelTier::Opus => &OPUS_PRICING,
        ModelTier::Sonnet => &SONNET_PRICING,
        ModelTier::Haiku => &HAIKU_PRICING,
        ModelTier::Glm5 => &GLM5_PRICING,
        ModelTier::Glm47 => &GLM47_PRICING,
        ModelTier::DeepSeekReasoner => &DEEPSEEK_R1_PRICING,
        ModelTier::Unknown => &UNKNOWN_PRICING,
    }
}

// ── Environmental constants ──────────────────────────────────────

/// US average grid carbon intensity (EPA eGRID 2024)
pub const CO2_KG_PER_KWH: f64 = 0.39;

/// Data center Power Usage Effectiveness overhead multiplier
pub const PUE: f64 = 1.2;

/// Data center Water Usage Effectiveness (Li et al. 2023 "Making AI Less Thirsty")
pub const WATER_LITERS_PER_KWH: f64 = 1.8;

/// One mature tree absorbs ~22 kg CO2/year (EPA)
pub const TREE_CO2_KG_PER_YEAR: f64 = 22.0;

/// One tree consumes ~3,900 liters of water/year (USDA Forestry)
pub const TREE_WATER_LITERS_PER_YEAR: f64 = 3900.0;

/// Netflix HD streaming energy: ~36 Wh per hour (IEA)
pub const NETFLIX_WH_PER_HOUR: f64 = 36.0;

/// Average car CO2: ~210g per km (EU average)
pub const CAR_CO2_G_PER_KM: f64 = 210.0;

/// Hamburger carbon footprint: ~3 kg CO2
pub const HAMBURGER_CO2_KG: f64 = 3.0;

/// Phone charge energy: ~15 Wh
pub const PHONE_CHARGE_WH: f64 = 15.0;

/// Average shower water: ~65 liters
pub const SHOWER_LITERS: f64 = 65.0;

/// LED bulb wattage: 10W
pub const LED_BULB_WATTS: f64 = 10.0;

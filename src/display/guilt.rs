use crate::config::*;
use crate::models::ImpactSummary;
use colored::Colorize;
use rand::seq::SliceRandom;

pub fn generate_comparisons(impact: &ImpactSummary) -> Vec<String> {
    let mut comparisons = Vec::new();
    let water_liters = impact.water_ml / 1000.0;
    let co2_kg = impact.co2_grams / 1000.0;

    // Water comparisons
    if water_liters > 0.001 {
        let glasses = water_liters / 0.25;
        comparisons.push(format!(
            "{} Your AI thirst consumed {:.1}L of water — that's {:.0} glasses that \
             could have gone to someone who isn't arguing with a chatbot about semicolons.",
            "💧", water_liters, glasses
        ));
    }
    if water_liters > 5.0 {
        let showers = water_liters / SHOWER_LITERS;
        comparisons.push(format!(
            "{} Equivalent to {:.1} showers. Except the shower would have been productive.",
            "🚿", showers
        ));
    }
    if water_liters > 100.0 {
        let toilet_flushes = water_liters / 6.0;
        comparisons.push(format!(
            "{} That's {:.0} toilet flushes. At least those serve a purpose.",
            "🚽", toilet_flushes
        ));
    }

    // CO2 comparisons
    if co2_kg > 0.01 {
        let car_km = (co2_kg * 1000.0) / CAR_CO2_G_PER_KM;
        comparisons.push(format!(
            "{} Equivalent to driving {:.1} km. Except the car would have gotten you somewhere.",
            "🚗", car_km
        ));
    }
    if co2_kg > 1.0 {
        let hamburgers = co2_kg / HAMBURGER_CO2_KG;
        comparisons.push(format!(
            "{} The carbon equivalent of {:.1} hamburgers. At least hamburgers taste good.",
            "🍔", hamburgers
        ));
    }
    if co2_kg > 10.0 {
        let flights_pct = (co2_kg / 90.0) * 100.0;
        comparisons.push(format!(
            "{} That's {:.1}% of a transatlantic flight. The flight has free pretzels at least.",
            "✈\u{fe0f}", flights_pct
        ));
    }

    // Energy comparisons
    if impact.energy_wh > 10.0 {
        let phone_charges = impact.energy_wh / PHONE_CHARGE_WH;
        comparisons.push(format!(
            "{} Enough energy to charge your phone {:.0} times. But sure, asking Claude \
             to rename a variable was totally worth it.",
            "🔋", phone_charges
        ));
    }
    if impact.energy_wh > 1000.0 {
        let lightbulb_hours = impact.energy_wh / LED_BULB_WATTS;
        comparisons.push(format!(
            "{} Could power a lightbulb for {:.0} hours. An actual light, illuminating \
             an actual room, for actual humans.",
            "💡", lightbulb_hours
        ));
    }

    // Netflix comparison
    if impact.netflix_hours_equiv > 0.1 {
        comparisons.push(format!(
            "{} You could've watched {:.0} hours of Netflix instead. The guilt would be \
             about your taste in shows, not the environment.",
            "📺", impact.netflix_hours_equiv
        ));
    }

    // Shuffle and return
    let mut rng = rand::thread_rng();
    comparisons.shuffle(&mut rng);
    comparisons
}

pub fn tree_progress_bar(trees_destroyed: f64) -> String {
    let progress = trees_destroyed.fract();
    let whole_trees = trees_destroyed.floor() as u64;
    let bar_width = 30;
    let filled = (progress * bar_width as f64).round() as usize;
    let empty = bar_width - filled;

    let bar = format!(
        "[{}{}] {:.1}%",
        "#".repeat(filled),
        ".".repeat(empty),
        progress * 100.0,
    );

    if whole_trees == 0 {
        format!(
            "  {} {}",
            "Progress to destroying your first tree:".yellow(),
            bar.yellow().bold()
        )
    } else {
        format!(
            "  {} {}  |  {} {}",
            "Trees completely destroyed:".red(),
            whole_trees.to_string().red().bold(),
            "Next victim:".yellow(),
            bar.yellow().bold()
        )
    }
}

pub const GUILT_QUOTES: &[&str] = &[
    "\"Move fast and break things.\" — Mark Zuckerberg, apparently talking about the climate.",
    "Remember: every token you generate is a tiny prayer to the god of entropy.",
    "The good news: AI will solve climate change. The bad news: AI is also causing it. \
     The ugly news: you're the reason.",
    "Fun fact: the dinosaurs went extinct without generating a single token. \
     Something to aspire to.",
    "Your usage today has been reported to Greta Thunberg. She is not amused.",
    "In the time it took to read this report, another ice cap melted. \
     Not because of you specifically. But also not NOT because of you.",
    "Somewhere, a tree is photosynthesizing as hard as it can, desperately trying \
     to offset your Claude Code habit.",
    "If every developer used AI at your rate, we'd need 3.7 Earths. We have 1. \
     Math is hard. Climate change is harder.",
    "Today's session brought to you by fossil fuels, data center cooling systems, \
     and your inability to remember Python syntax.",
    "The planet called. It wants its water back. Also it's filing a restraining order.",
    "\"Surely one more prompt won't hurt,\" you said, for the 47th time today.",
    "Your Claude usage has consumed more water today than a cactus drinks in a month. A CACTUS.",
    "On the bright side, at least you're not training the model. You're just... \
     relentlessly querying it. Like a woodpecker on a redwood.",
    "This report was generated by Claude Code, consuming additional energy. \
     We are the problem reporting on the problem.",
    "Remember when coding meant typing in a text editor and the only environmental cost \
     was the electricity for your monitor? Pepperidge Farm remembers.",
    "AI is the only industry where 'scaling up' means both 'getting better at tasks' and \
     'accelerating planetary destruction.' Neat!",
    "Your tokens are gone but their CO2 is forever. Well, for the next few thousand years. \
     Which is basically forever in human terms.",
    "Congrats! You've outsourced your thinking to a machine that drinks more water than \
     a swimming pool. Efficiency!",
    "A tree just fell in a forest. Nobody heard it. But we know who's responsible.",
    "Your grandchildren will ask what you did during the climate crisis. \
     'I asked an AI to write my unit tests' isn't the flex you think it is.",
    "Data centers now use more water than some small countries. \
     You're basically a colonial power. Congrats.",
    "The carbon in your prompts will outlive your GitHub repos. Think about that.",
    "You've burned more energy asking AI for help than your ancestors used \
     in their entire lifetime. Progress!",
    "Somewhere in Virginia, a river is being diverted to cool a server \
     that's helping you decide between tabs and spaces.",
];

pub fn random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    GUILT_QUOTES.choose(&mut rng).unwrap()
}

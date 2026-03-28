use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL_CONDENSED;
use comfy_table::*;

use crate::display::format::*;
use crate::models::TokenRecord;

pub fn render_session_detail(records: &[TokenRecord], query: &str) {
    let mut matched: Vec<&TokenRecord> = records
        .iter()
        .filter(|r| r.session_id.contains(query))
        .collect();
    matched.sort_by_key(|r| r.timestamp);

    if matched.is_empty() {
        eprintln!("No records found matching session '{}'.", query);
        return;
    }

    let session_id = &matched[0].session_id;
    println!();
    println!("  Session: {}", session_id);
    println!("  Messages: {}", matched.len());
    println!();

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        Cell::new("Time").set_alignment(CellAlignment::Left),
        Cell::new("Model").set_alignment(CellAlignment::Left),
        Cell::new("Input").set_alignment(CellAlignment::Right),
        Cell::new("Output").set_alignment(CellAlignment::Right),
        Cell::new("Cache R").set_alignment(CellAlignment::Right),
        Cell::new("Cache W").set_alignment(CellAlignment::Right),
        Cell::new("Total").set_alignment(CellAlignment::Right),
    ]);

    let mut total_input = 0u64;
    let mut total_output = 0u64;
    let mut total_cache_read = 0u64;
    let mut total_cache_create = 0u64;

    for r in &matched {
        let total = r.input_tokens
            + r.output_tokens
            + r.cache_read_input_tokens
            + r.cache_creation_input_tokens;
        total_input += r.input_tokens;
        total_output += r.output_tokens;
        total_cache_read += r.cache_read_input_tokens;
        total_cache_create += r.cache_creation_input_tokens;

        table.add_row(vec![
            Cell::new(r.timestamp.format("%H:%M:%S").to_string()),
            Cell::new(r.model.display_name()),
            Cell::new(format_tokens(r.input_tokens)),
            Cell::new(format_tokens(r.output_tokens)),
            Cell::new(format_tokens(r.cache_read_input_tokens)),
            Cell::new(format_tokens(r.cache_creation_input_tokens)),
            Cell::new(format_tokens(total)),
        ]);
    }

    let grand_total = total_input + total_output + total_cache_read + total_cache_create;
    table.add_row(vec![
        Cell::new("TOTAL").fg(comfy_table::Color::White),
        Cell::new("").fg(comfy_table::Color::White),
        Cell::new(format_tokens(total_input)).fg(comfy_table::Color::White),
        Cell::new(format_tokens(total_output)).fg(comfy_table::Color::White),
        Cell::new(format_tokens(total_cache_read)).fg(comfy_table::Color::White),
        Cell::new(format_tokens(total_cache_create)).fg(comfy_table::Color::White),
        Cell::new(format_tokens(grand_total)).fg(comfy_table::Color::White),
    ]);

    println!("{table}");
}

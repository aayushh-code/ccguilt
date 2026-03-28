use crossterm::{cursor, execute, style, terminal};
use std::io::Write;

use crate::display::format::*;
use crate::interactive::state::{AppState, View};

pub fn draw(state: &AppState, stdout: &mut impl Write) -> anyhow::Result<()> {
    let (width, height) = terminal::size()?;
    let w = width as usize;
    let h = height as usize;

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;

    // Header bar
    let header = format!(
        " CCGUILT | View: {} | Sort: {} | Model: {} | Depth: {} | [q]uit [s]ort [m]odel [Tab]view [Enter]drill [Bksp]back",
        match state.view { View::Table => "Table", View::Chart => "Chart" },
        state.sort_label(),
        if state.by_model { "ON" } else { "OFF" },
        state.drill_stack.len(),
    );
    let header_padded = format!("{:<width$}", header, width = w);
    execute!(
        stdout,
        style::SetBackgroundColor(style::Color::DarkCyan),
        style::SetForegroundColor(style::Color::Black),
    )?;
    write!(stdout, "{}", header_padded)?;
    execute!(
        stdout,
        style::ResetColor,
    )?;

    if state.buckets.is_empty() {
        execute!(stdout, cursor::MoveTo(2, 2))?;
        write!(stdout, "No data to display.")?;
        stdout.flush()?;
        return Ok(());
    }

    match state.view {
        View::Table => draw_table(state, stdout, w, h)?,
        View::Chart => draw_chart(state, stdout, w, h)?,
    }

    // Status bar
    let total_co2: f64 = state.buckets.iter().map(|b| b.impact.co2_grams).sum();
    let total_cost: f64 = state.buckets.iter().map(|b| b.cost.total_cost_usd).sum();
    let status = format!(
        " Row {}/{} | Total CO2: {} | Total Cost: {}",
        state.selected + 1,
        state.buckets.len(),
        format_co2(total_co2),
        format_cost(total_cost),
    );
    let status_padded = format!("{:<width$}", status, width = w);
    execute!(stdout, cursor::MoveTo(0, height - 1))?;
    execute!(
        stdout,
        style::SetBackgroundColor(style::Color::DarkGrey),
        style::SetForegroundColor(style::Color::White),
    )?;
    write!(stdout, "{}", status_padded)?;
    execute!(stdout, style::ResetColor)?;

    stdout.flush()?;
    Ok(())
}

fn draw_table(state: &AppState, stdout: &mut impl Write, w: usize, h: usize) -> anyhow::Result<()> {
    let max_rows = h.saturating_sub(4); // header + blank + status + margin

    // Column widths
    let col_widths = [14, 10, 10, 12, 10, 10, 8, 24];
    let header_items = ["Period", "Tokens", "Cost", "Energy", "CO2", "Water", "Trees", "Guilt"];

    // Draw column headers
    execute!(stdout, cursor::MoveTo(0, 1))?;
    execute!(
        stdout,
        style::SetForegroundColor(style::Color::Red),
        style::SetAttribute(style::Attribute::Bold),
    )?;
    let mut header_line = String::new();
    for (i, h) in header_items.iter().enumerate() {
        header_line.push_str(&format!("{:>width$} ", h, width = col_widths[i]));
    }
    write!(stdout, " {}", &header_line[..header_line.len().min(w)])?;
    execute!(stdout, style::ResetColor)?;

    // Separator
    execute!(stdout, cursor::MoveTo(0, 2))?;
    let sep = "\u{2500}".repeat(w.min(100));
    execute!(stdout, style::SetForegroundColor(style::Color::DarkGrey))?;
    write!(stdout, " {}", sep)?;
    execute!(stdout, style::ResetColor)?;

    // Compute scroll offset
    let scroll = if state.selected >= max_rows {
        state.selected - max_rows + 1
    } else {
        0
    };

    // Draw rows
    for (i, bucket) in state.buckets.iter().enumerate().skip(scroll).take(max_rows) {
        let row_y = 3 + (i - scroll) as u16;
        execute!(stdout, cursor::MoveTo(0, row_y))?;

        let is_selected = i == state.selected;

        if is_selected {
            execute!(
                stdout,
                style::SetBackgroundColor(style::Color::DarkBlue),
                style::SetForegroundColor(style::Color::White),
            )?;
        }

        let guilt_short = match bucket.guilt.level {
            crate::models::GuiltLevel::DigitalSaint => "Saint",
            crate::models::GuiltLevel::CarbonCurious => "Curious",
            crate::models::GuiltLevel::TreeTrimmer => "Trimmer",
            crate::models::GuiltLevel::ForestFlattener => "Flattener",
            crate::models::GuiltLevel::EcoTerrorist => "Terrorist",
            crate::models::GuiltLevel::PlanetIncinerator => "Incinerator",
            crate::models::GuiltLevel::HeatDeathAccelerator => "HeatDeath",
        };

        let row = format!(
            " {:>width0$} {:>width1$} {:>width2$} {:>width3$} {:>width4$} {:>width5$} {:>width6$} {:>width7$}",
            bucket.label,
            format_tokens(bucket.tokens.total_tokens()),
            format_cost(bucket.cost.total_cost_usd),
            format_energy(bucket.impact.energy_wh),
            format_co2(bucket.impact.co2_grams),
            format_water(bucket.impact.water_ml),
            format_trees(bucket.impact.trees_destroyed),
            guilt_short,
            width0 = col_widths[0],
            width1 = col_widths[1],
            width2 = col_widths[2],
            width3 = col_widths[3],
            width4 = col_widths[4],
            width5 = col_widths[5],
            width6 = col_widths[6],
            width7 = col_widths[7],
        );

        let padded = format!("{:<width$}", row, width = w);
        write!(stdout, "{}", &padded[..padded.len().min(w)])?;

        if is_selected {
            execute!(stdout, style::ResetColor)?;
        }
    }

    Ok(())
}

fn draw_chart(state: &AppState, stdout: &mut impl Write, w: usize, h: usize) -> anyhow::Result<()> {
    let max_rows = h.saturating_sub(4);
    let bar_max = w.saturating_sub(30);

    let max_co2 = state
        .buckets
        .iter()
        .map(|b| b.impact.co2_grams)
        .fold(0.0_f64, f64::max);

    if max_co2 == 0.0 {
        execute!(stdout, cursor::MoveTo(2, 2))?;
        write!(stdout, "No CO2 data to chart.")?;
        return Ok(());
    }

    execute!(stdout, cursor::MoveTo(2, 1))?;
    execute!(
        stdout,
        style::SetForegroundColor(style::Color::Red),
        style::SetAttribute(style::Attribute::Bold),
    )?;
    write!(stdout, "CO2 Emissions by Period")?;
    execute!(stdout, style::ResetColor)?;

    let label_width = state.buckets.iter().map(|b| b.label.len()).max().unwrap_or(10);

    for (i, bucket) in state.buckets.iter().enumerate().take(max_rows) {
        let row_y = 3 + i as u16;
        execute!(stdout, cursor::MoveTo(0, row_y))?;

        let is_selected = i == state.selected;
        let ratio = bucket.impact.co2_grams / max_co2;
        let bar_len = (ratio * bar_max as f64).round() as usize;
        let bar_len = bar_len.max(if bucket.impact.co2_grams > 0.0 { 1 } else { 0 });

        let color = match bucket.guilt.level {
            crate::models::GuiltLevel::DigitalSaint => style::Color::Green,
            crate::models::GuiltLevel::CarbonCurious => style::Color::Cyan,
            crate::models::GuiltLevel::TreeTrimmer => style::Color::Yellow,
            crate::models::GuiltLevel::ForestFlattener => style::Color::DarkYellow,
            crate::models::GuiltLevel::EcoTerrorist => style::Color::Red,
            crate::models::GuiltLevel::PlanetIncinerator => style::Color::DarkRed,
            crate::models::GuiltLevel::HeatDeathAccelerator => style::Color::Magenta,
        };

        if is_selected {
            execute!(stdout, style::SetAttribute(style::Attribute::Bold))?;
        }

        write!(stdout, " {:>width$} ", bucket.label, width = label_width)?;
        execute!(stdout, style::SetForegroundColor(color))?;
        write!(stdout, "{}", "\u{2588}".repeat(bar_len))?;
        execute!(stdout, style::ResetColor)?;
        write!(stdout, " {}", format_co2(bucket.impact.co2_grams))?;

        if is_selected {
            execute!(stdout, style::SetAttribute(style::Attribute::NoBold))?;
        }
    }

    Ok(())
}

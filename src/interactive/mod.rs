pub mod render;
pub mod state;

use crossterm::{
    cursor, execute,
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};

use crate::display::DisplayOptions;
use crate::models::{TokenRecord, UsageBucket};
use crate::runtime::RuntimeConfig;

use state::AppState;

/// RAII guard to restore terminal state on drop (including panics)
struct RawModeGuard;

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            LeaveAlternateScreen,
            cursor::Show
        );
    }
}

pub fn run_interactive(
    records: Vec<TokenRecord>,
    buckets: Vec<UsageBucket>,
    display_opts: DisplayOptions,
    rc: RuntimeConfig,
) -> anyhow::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let _guard = RawModeGuard;

    let mut state = AppState::new(records, buckets, display_opts, rc);

    loop {
        render::draw(&state, &mut stdout)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => break,
                KeyCode::Up | KeyCode::Char('k') => state.move_up(),
                KeyCode::Down | KeyCode::Char('j') => state.move_down(),
                KeyCode::Enter => state.drill_down(),
                KeyCode::Backspace => state.drill_up(),
                KeyCode::Tab => state.cycle_view(),
                KeyCode::Char('s') => state.cycle_sort(),
                KeyCode::Char('m') => state.toggle_by_model(),
                _ => {}
            }
        }
    }

    // Guard handles cleanup in Drop
    Ok(())
}

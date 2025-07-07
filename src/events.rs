use crossterm::event::{self, Event, KeyCode};
use ratatui::Terminal;
use std::time::{Duration, Instant};

use crate::ui;

pub fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>) -> std::io::Result<()> {
    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            ui::draw_ui(f, size);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
    Ok(())
}

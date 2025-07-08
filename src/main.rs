mod events;
mod sysinfo;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Yoinky",
    version = "0.1.0",
    author = "Huseyn",
    about = "TUI tool for monitoring system resources like CPU, RAM, and GPU."
)]
pub struct Cli {
    input: Option<String>,
}

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Uygulama döngüsü başlat
    let result = events::run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        crossterm::terminal::LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;
    result
}

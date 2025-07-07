mod ui;
mod sysinfo;
mod events;

use std::io;
use crossterm::{terminal::{enable_raw_mode, disable_raw_mode}, execute};
use ratatui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Uygulama döngüsü başlat
    let result = events::run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}

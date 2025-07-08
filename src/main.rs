mod events;
mod sysinfo;
mod ui;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{env, io};

use clap::Parser;

#[derive(Parser)]
#[command(
    name = "Yoinky",
    version = env!("CARGO_PKG_VERSION"),
    author = "Huseyn",
    about = "TUI tool for monitoring system resources like CPU, RAM, and GPU."
)]
pub struct Cli {
    #[arg(short, long, help = "Input option for the application")]
    input: Option<String>,
}

fn main() -> Result<(), io::Error> {
    // Parse command line arguments
    let cli = Cli::parse();
    if let Some(input) = cli.input {
        println!("Input option provided: {}", input);
    } else {
        println!("No input option provided.");
    }
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

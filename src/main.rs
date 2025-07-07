use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use procfs::CpuInfo;
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Widget},
    Terminal,
};
use std::{
    io,
    time::{Duration, Instant},
};

fn main() -> Result<(), io::Error> {
    // Terminali ayarla
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().title("CPU Usage").borders(Borders::ALL);

            let cpu_info = CpuInfo::new().unwrap();
            let cpu_count = cpu_info.num_cores();

            let mut cpu_text = String::new();
            cpu_text.push_str(&format!("CPU Core Count: {}\n", cpu_count));

            let paragraph = ratatui::widgets::Paragraph::new(cpu_text).block(block);
            f.render_widget(paragraph, size);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
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

    // Terminali eski haline getir
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Gauge, Paragraph},
};

use crate::sysinfo;

pub fn draw_ui(f: &mut Frame, area: Rect) {
    // --- Styling ---
    let title_style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD);
    let label_style = Style::default().fg(Color::White);
    let value_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let temp_style = Style::default().fg(Color::Yellow);
    let na_style = Style::default().fg(Color::DarkGray);
    let quit_style = Style::default()
        .fg(Color::Red)
        .add_modifier(Modifier::ITALIC);

    // --- Layout ---
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // For Title
            Constraint::Min(0),    // For Content
            Constraint::Length(2), // For Footer
        ])
        .split(area);

    // Create a 2x2 grid layout for the content
    /* let top_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);
    */
    /* let bottom_row = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]); // This is a bit of a hack, we'll split the bottom half
    */
    let content_areas = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    let top_content_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_areas[0]);

    let bottom_content_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_areas[1]);

    // --- Data Fetching ---
    let cpu_count = sysinfo::cpu_core_count();
    let cpu_temp = sysinfo::cpu_temp();
    let cpu_usage = sysinfo::cpu_usage();
    let ram_usage = sysinfo::ram_usage();
    let gpu_info = sysinfo::gpu_info();
    let disk_usage = sysinfo::disk_usage();

    // --- Title Widget ---
    let title = Paragraph::new("📟 YOINKY :: SYSTEM MONITOR 📟")
        .style(title_style)
        .alignment(Alignment::Center);
    f.render_widget(title, main_layout[0]);

    // --- CPU Widget ---
    let cpu_block = Block::default()
        .title(" 💻 CPU ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightBlue));

    let cpu_text = vec![
        Line::from(vec![
            Span::styled("Cores: ", label_style),
            Span::styled(cpu_count.to_string(), value_style),
        ]),
        Line::from(vec![
            Span::styled("Temp:  ", label_style),
            if let Some(temp) = cpu_temp {
                Span::styled(format!("{:.1} °C", temp), temp_style)
            } else {
                Span::styled("N/A", na_style)
            },
        ]),
    ];
    let label = match cpu_usage {
        Some(u) => format!("Usage: {:.1}%", u),
        None => "Usage: N/A".to_string(),
    };

    let cpu_gauge = Gauge::default()
        .label(label)
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(cpu_usage.map_or(0, |u| u as u16));

    f.render_widget(cpu_block, top_content_areas[0]);
    // Inner layout for CPU box
    let cpu_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(2), Constraint::Min(1)])
        .split(top_content_areas[0]);
    f.render_widget(Paragraph::new(cpu_text), cpu_chunks[0]);
    f.render_widget(cpu_gauge, cpu_chunks[1]);

    // --- RAM Widget ---
    let ram_block = Block::default()
        .title(" 🧠 RAM ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Rgb(255, 92, 0)));

    let (ram_percent, ram_text) = if let Some((used, total)) = ram_usage {
        (
            ((used as f64 / total as f64) * 100.0) as u16,
            format!("{:.1}MB / {:.1}MB", used, total),
        )
    } else {
        (0, "N/A".to_string())
    };
    let ram_gauge = Gauge::default()
        .label(ram_text)
        .gauge_style(Style::default().fg(Color::Rgb(255, 92, 0)))
        .percent(ram_percent);
    f.render_widget(ram_block.clone(), top_content_areas[1]);
    f.render_widget(ram_gauge.block(ram_block), top_content_areas[1]);

    // --- GPU Widget ---
    let gpu_block = Block::default()
        .title(" 🎮 GPU ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightGreen));

    let mut gpu_text = vec![];
    if let Some((gpu_type, temp)) = gpu_info {
        gpu_text.push(Line::from(vec![
            Span::styled("Type: ", label_style),
            Span::styled(gpu_type.to_string(), value_style),
        ]));
        gpu_text.push(Line::from(vec![
            Span::styled("Temp: ", label_style),
            Span::styled(format!("{} °C", temp), temp_style),
        ]));
    } else {
        gpu_text.push(Line::from(Span::styled("N/A", na_style)));
    }
    let gpu_paragraph = Paragraph::new(gpu_text)
        .block(gpu_block)
        .alignment(Alignment::Center)
        .wrap(ratatui::widgets::Wrap { trim: true });
    f.render_widget(gpu_paragraph, bottom_content_areas[0]);

    // --- Disk Widget ---
    let disk_block = Block::default()
        .title(" 💾 DISK ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Red));

    let (disk_percent, disk_text) = if let Some((used, total)) = disk_usage {
        (
            ((used / total) * 100.0) as u16,
            format!("{:.1}GB / {:.1}GB", used, total),
        )
    } else {
        (0, "N/A".to_string())
    };
    let disk_gauge = Gauge::default()
        .label(disk_text)
        .gauge_style(Style::default().fg(Color::Red))
        .percent(disk_percent);
    f.render_widget(disk_block.clone(), bottom_content_areas[1]);
    f.render_widget(disk_gauge.block(disk_block), bottom_content_areas[1]);

    // --- Footer Widget ---
    let footer = Paragraph::new("Press 'q' to quit")
        .style(quit_style)
        .alignment(Alignment::Center);
    f.render_widget(footer, main_layout[2]);
}

use ratatui::{
    prelude::*,
    widgets::{Block, BorderType, Borders, Paragraph},
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

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_layout[1]);

    // --- Data Fetching ---
    let cpu_count = sysinfo::cpu_core_count();
    let cpu_temp = sysinfo::cpu_temp();
    let gpu_info = sysinfo::gpu_info();

    // --- Title Widget ---
    let title = Paragraph::new("📟 YOINKY :: SYSTEM MONITOR 📟")
        .style(title_style)
        .alignment(Alignment::Center);
    f.render_widget(title, main_layout[0]);

    // --- CPU Widget ---
    let cpu_block = Block::default()
        .title(" 💻 CPU INFO ")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightBlue));

    let mut cpu_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Cores │ ", label_style),
            Span::styled(cpu_count.to_string(), value_style),
        ]),
        Line::from(""),
    ];
    let cpu_temp_line = if let Some(temp) = cpu_temp {
        Line::from(vec![
            Span::styled("Temp  │ ", label_style),
            Span::styled(format!("{:.1} °C", temp), temp_style),
        ])
    } else {
        Line::from(vec![
            Span::styled("Temp  │ ", label_style),
            Span::styled("N/A", na_style),
        ])
    };
    cpu_text.push(cpu_temp_line);

    let cpu_paragraph = Paragraph::new(cpu_text)
        .block(cpu_block)
        .alignment(Alignment::Center);
    f.render_widget(cpu_paragraph, content_layout[0]);

    // --- GPU Widget ---
    let gpu_block = Block::default()
        .title(" 🎮 GPU INFO ")
        .title_style(Style::default().add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::LightGreen));

    let mut gpu_text = vec![Line::from("")]; // Padding
    if let Some((gpu_type, temp)) = gpu_info {
        gpu_text.push(Line::from(vec![
            Span::styled("Type  │ ", label_style),
            Span::styled(gpu_type.to_string(), value_style),
        ]));
        gpu_text.push(Line::from("")); // Spacer
        gpu_text.push(Line::from(vec![
            Span::styled("Temp  │ ", label_style),
            Span::styled(format!("{} °C", temp), temp_style),
        ]));
    } else {
        gpu_text.push(Line::from(vec![
            Span::styled("Type  │ ", label_style),
            Span::styled("N/A", na_style),
        ]));
        gpu_text.push(Line::from("")); // Spacer
        gpu_text.push(Line::from(vec![
            Span::styled("Temp  │ ", label_style),
            Span::styled("N/A", na_style),
        ]));
    }

    let gpu_paragraph = Paragraph::new(gpu_text)
        .block(gpu_block)
        .alignment(Alignment::Center);
    f.render_widget(gpu_paragraph, content_layout[1]);

    // --- Footer Widget ---
    let footer = Paragraph::new("Press 'q' to quit")
        .style(quit_style)
        .alignment(Alignment::Center);
    f.render_widget(footer, main_layout[2]);
}

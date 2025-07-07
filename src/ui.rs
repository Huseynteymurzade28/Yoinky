use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
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
    let separator_style = Style::default().fg(Color::Gray);

    // --- Data Fetching ---
    let cpu_count = sysinfo::cpu_core_count();
    let cpu_temp = sysinfo::cpu_temp();
    let gpu_temp = sysinfo::gpu_temp();

    // --- Text Assembly ---
    let mut text = vec![
        Line::from(""), // Top padding
        // --- CPU Section ---
        Line::from("💻 CPU INFO").style(
            Style::default()
                .fg(Color::LightBlue)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(vec![
            Span::styled("  Cores │ ", label_style),
            Span::styled(cpu_count.to_string(), value_style),
        ]),
    ];

    let cpu_temp_line = if let Some(temp) = cpu_temp {
        Line::from(vec![
            Span::styled("  Temp  │ ", label_style),
            Span::styled(format!("{:.1} °C", temp), temp_style),
        ])
    } else {
        Line::from(vec![
            Span::styled("  Temp  │ ", label_style),
            Span::styled("N/A", na_style),
        ])
    };
    text.push(cpu_temp_line);

    // --- Separator ---
    text.push(Line::from("")); // Spacer
    text.push(Line::from("──────────────────────────────────").style(separator_style));
    text.push(Line::from("")); // Spacer

    // --- GPU Section ---
    text.push(
        Line::from("🎮 GPU INFO").style(
            Style::default()
                .fg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        ),
    );
    let gpu_temp_line = if let Some(temp) = gpu_temp {
        Line::from(vec![
            Span::styled("  Temp  │ ", label_style),
            Span::styled(format!("{:.1} °C", temp), temp_style),
        ])
    } else {
        Line::from(vec![
            Span::styled("  Temp  │ ", label_style),
            Span::styled("N/A", na_style),
        ])
    };
    text.push(gpu_temp_line);

    // --- Block Creation ---
    let block = Block::default()
        .title(Span::styled(
            " 📟 YOINKY :: SYSTEM MONITOR 📟 ",
            title_style,
        ))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .border_style(Style::default().fg(Color::Blue));

    // --- Paragraph Widget ---
    let para = Paragraph::new(text)
        .block(block)
        .alignment(Alignment::Center);

    // --- Rendering ---
    f.render_widget(para, area);
}

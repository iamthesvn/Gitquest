use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;

pub fn draw_hud(frame: &mut Frame, app: &App, level: usize, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(22),
            Constraint::Length(14),
            Constraint::Length(16),
        ])
        .split(area);

    // Level name + description
    let name = app.level_name(level);
    let desc = app.level_description(level);
    let level_info = Paragraph::new(Line::from(vec![
        Span::styled(
            " GitQuest ",
            Style::default()
                .fg(Color::Rgb(240, 80, 50))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("│ ", Style::default().fg(Color::Rgb(60, 60, 80))),
        Span::styled(
            format!("L{} ", level + 1),
            Style::default()
                .fg(Color::Rgb(240, 80, 50))
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("{name} — {desc}"),
            Style::default().fg(Color::Rgb(180, 180, 180)),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    );
    frame.render_widget(level_info, chunks[0]);

    // Score display
    let score = app.save.total_score;
    let score_para = Paragraph::new(Line::from(vec![
        Span::styled("Score: ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{score:>5}"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    );
    frame.render_widget(score_para, chunks[1]);

    // Progress pips
    let pips: String = (0..app.level_count())
        .map(|i| {
            if i < level {
                "●"
            } else if i == level {
                "◉"
            } else {
                "○"
            }
        })
        .collect::<Vec<_>>()
        .join(" ");

    let progress_para = Paragraph::new(Line::from(vec![Span::styled(
        format!(" {pips} "),
        Style::default()
            .fg(Color::Rgb(240, 80, 50))
            .add_modifier(Modifier::BOLD),
    )]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    );
    frame.render_widget(progress_para, chunks[2]);

    // Music mute status
    let (mute_icon, mute_color) = if app.is_muted() {
        (" ♪ MUTED ", Color::DarkGray)
    } else {
        (" ♪ [M]mute", Color::Rgb(100, 180, 100))
    };

    let mute_para = Paragraph::new(Line::from(Span::styled(
        mute_icon,
        Style::default().fg(mute_color),
    )))
    .block(
        Block::default()
            .borders(Borders::BOTTOM | Borders::LEFT)
            .border_style(Style::default().fg(Color::Rgb(40, 40, 60))),
    );
    frame.render_widget(mute_para, chunks[3]);
}

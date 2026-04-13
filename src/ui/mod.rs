pub mod hud;
pub mod menu;
pub mod summary;
pub mod transition;

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::{App, AppState}, story};

pub fn draw(frame: &mut Frame, app: &App) {
    match &app.state {
        AppState::Menu { selected } => {
            menu::draw_menu(frame, frame.area(), *selected);
        }
        AppState::LevelIntro { level } => {
            draw_level_intro(frame, app, *level);
        }
        AppState::Playing { level } => {
            draw_playing(frame, app, *level);
        }
        AppState::LevelComplete { level, score } => {
            summary::draw_level_complete(frame, app, *level, *score, app.anim_tick);
        }
        AppState::Transition { next_level, frame: anim_frame } => {
            transition::draw_transition(frame, *next_level, *anim_frame);
        }
        AppState::GameComplete { total_score } => {
            summary::draw_game_complete(frame, app, *total_score);
        }
        AppState::Quit => {}
    }
}

pub fn draw_resize_warning(frame: &mut Frame) {
    let area = frame.area();
    let warning = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "  ⚠  Terminal too small!",
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  GitQuest needs at least 80×24.",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        )),
        Line::from(Span::styled(
            "  Please resize your terminal window.",
            Style::default().fg(Color::DarkGray),
        )),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow)));
    frame.render_widget(warning, area);
}

fn draw_level_intro(frame: &mut Frame, app: &App, level: usize) {
    let area = frame.area();

    let story = story::LEVEL_STORIES.get(level);

    // Two-panel layout: left = ASCII art scene, right = story text
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(28), Constraint::Min(40)])
        .split(area);

    // Left panel: ASCII art illustration
    let art = story::LEVEL_ART.get(level).copied().unwrap_or(&[]);
    let art_lines: Vec<Line> = art
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                *l,
                Style::default().fg(Color::Rgb(100, 180, 120)),
            ))
        })
        .collect();

    let art_widget = Paragraph::new(art_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(60, 100, 60)))
            .title(Span::styled(
                " Scene ",
                Style::default().fg(Color::Rgb(100, 180, 120)),
            )),
    );
    frame.render_widget(art_widget, panels[0]);

    // Right panel: story
    let mut lines = vec![Line::from("")];

    if let Some(beat) = story {
        lines.push(Line::from(Span::styled(
            format!("  ── {} ──", beat.title),
            Style::default()
                .fg(Color::Rgb(240, 80, 50))
                .add_modifier(ratatui::style::Modifier::BOLD),
        )));
        lines.push(Line::from(""));
        for story_line in beat.lines {
            lines.push(Line::from(Span::styled(
                *story_line,
                Style::default().fg(Color::Rgb(180, 210, 180)),
            )));
        }
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("  Level {} of {}", level + 1, app.level_count()),
        Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(""));
    // Pulsing Enter prompt
    lines.push(Line::from(vec![
        Span::styled("  ❯ Press ", Style::default().fg(Color::Rgb(180, 180, 180))),
        Span::styled(
            "[Enter]",
            Style::default()
                .fg(Color::Rgb(255, 255, 100))
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        Span::styled(
            " or [Space] to begin the chapter",
            Style::default().fg(Color::Rgb(180, 180, 180)),
        ),
    ]));

    let story_widget = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(60, 80, 60)))
            .title(Span::styled(
                " The Codewright Chronicles ",
                Style::default()
                    .fg(Color::Rgb(240, 80, 50))
                    .add_modifier(ratatui::style::Modifier::BOLD),
            )),
    );
    frame.render_widget(story_widget, panels[1]);
}

fn draw_playing(frame: &mut Frame, app: &App, level: usize) {
    let area = frame.area();

    // Layout: top HUD bar, main game area, bottom key bar
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // HUD top bar
    hud::draw_hud(frame, app, level, chunks[0]);

    // Game content
    let game_area = chunks[1];
    if let Some(lvl) = app.levels.get(level) {
        lvl.render(frame, game_area);
    }

    // Bottom key bar
    draw_key_bar(frame, app, level, chunks[2]);
}

fn draw_key_bar(frame: &mut Frame, app: &App, level: usize, area: Rect) {
    let hint_key = Span::styled("[?] Hint", Style::default().fg(Color::Rgb(180, 180, 180)));
    let quit_key = Span::styled("[Ctrl+Q] Quit", Style::default().fg(Color::Rgb(180, 180, 180)));

    let level_keys = match level {
        1 => "[↑↓] Navigate  [Space] Stage  [A] Stage All  [S] Submit",
        2 => "[Type] Commit  [Enter] Submit  [Backspace] Edit",
        3 => "[↑↓] Navigate  [K/J] Move Item  [Enter] Confirm",
        _ => "[Type] Command  [Enter] Submit  [Backspace] Edit",
    };

    let keys_line = Line::from(vec![
        Span::styled(format!("  {level_keys}  "), Style::default().fg(Color::DarkGray)),
        hint_key,
        Span::styled("  ", Style::default()),
        quit_key,
    ]);

    // Hint overlay if active
    if app.show_hint {
        let hint = app.level_hint(level);
        let hint_para = Paragraph::new(Line::from(Span::styled(
            format!("  💡 {hint}"),
            Style::default().fg(Color::Cyan).add_modifier(ratatui::style::Modifier::BOLD),
        )))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(Span::styled(" Hint ", Style::default().fg(Color::Cyan))));
        frame.render_widget(hint_para, area);
    } else {
        let bar = Paragraph::new(keys_line)
            .block(Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(40, 40, 60))));
        frame.render_widget(bar, area);
    }
}

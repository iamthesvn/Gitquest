use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{Level, LevelStatus};

const TARGET: &str = "git init";

const TREE_STAGES: &[&str] = &[
    "",
    ".",
    ".\n└── [?]",
    ".\n└── src/",
    ".\n├── src/\n└── [?]",
    ".\n├── src/\n└── .git/  ← initializing...",
    ".\n├── src/\n└── .git/\n    ├── HEAD\n    ├── config\n    └── objects/",
];

pub struct LevelInit {
    typed: String,
    wrong_flashes: u8,
    errors: u32,
    start_time: std::time::Instant,
    completed: bool,
    pub flash: bool,
}

impl LevelInit {
    pub fn new() -> Self {
        Self {
            typed: String::new(),
            wrong_flashes: 0,
            errors: 0,
            start_time: std::time::Instant::now(),
            completed: false,
            flash: false,
        }
    }

    fn tree_stage(&self) -> usize {
        let ratio = self.typed.len() as f32 / TARGET.len() as f32;
        let max = TREE_STAGES.len() - 1;
        (ratio * max as f32) as usize
    }
}

impl Level for LevelInit {
    fn name(&self) -> &str {
        "git init"
    }

    fn description(&self) -> &str {
        "Create Your Universe"
    }

    fn hint(&self) -> &str {
        "Type: git init   (exactly)"
    }

    fn score(&self) -> u32 {
        if !self.completed {
            return 0;
        }
        let base = 100u32;
        let penalty = self.errors * 5;
        let elapsed = self.start_time.elapsed().as_secs();
        let time_bonus = if elapsed < 10 { 50 } else { 0 };
        base.saturating_sub(penalty).saturating_add(time_bonus).min(150)
    }

    fn tick(&mut self) -> LevelStatus {
        if self.wrong_flashes > 0 {
            self.wrong_flashes -= 1;
            self.flash = self.wrong_flashes > 0;
        }
        LevelStatus::InProgress
    }

    fn update(&mut self, event: KeyEvent) -> LevelStatus {
        if self.completed {
            return LevelStatus::Completed;
        }

        match event.code {
            KeyCode::Backspace => {
                self.typed.pop();
            }
            KeyCode::Char(c) => {
                let next_pos = self.typed.len();
                if next_pos < TARGET.len() {
                    let Some(expected) = TARGET.chars().nth(next_pos) else {
                        return LevelStatus::InProgress;
                    };
                    if c == expected {
                        self.typed.push(c);
                        if self.typed == TARGET {
                            self.completed = true;
                            return LevelStatus::Completed;
                        }
                    } else {
                        self.errors += 1;
                        self.wrong_flashes = 3;
                        self.flash = true;
                        return LevelStatus::Failed(format!("Wrong key: expected '{expected}'"));
                    }
                }
            }
            _ => {}
        }
        LevelStatus::InProgress
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6),
                Constraint::Min(8),
                Constraint::Length(4),
            ])
            .split(area);

        // Narrative
        let _narrative_style = Style::default().fg(Color::Rgb(180, 180, 180));
        let narrative = if self.typed.is_empty() {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  There is nothing here. No files. No history. No project.",
                    Style::default().fg(Color::DarkGray),
                )),
                Line::from(""),
                // Action line — bright, impossible to miss
                Line::from(vec![
                    Span::styled("  ❯ ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "Type:  git init",
                        Style::default()
                            .fg(Color::Rgb(255, 255, 100))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  and press Enter", Style::default().fg(Color::Rgb(200, 200, 200))),
                ]),
            ]
        } else {
            vec![
                Line::from(""),
                Line::from(Span::styled(
                    "  The void stirs... something is taking shape.",
                    Style::default().fg(Color::Rgb(150, 200, 150)),
                )),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  ❯ ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "Keep typing:  git init",
                        Style::default()
                            .fg(Color::Rgb(255, 255, 100))
                            .add_modifier(Modifier::BOLD),
                    ),
                ]),
            ]
        };

        let narrative_block = Paragraph::new(narrative)
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(narrative_block, chunks[0]);

        // Tree visualization
        let stage = if self.completed { TREE_STAGES.len() - 1 } else { self.tree_stage() };
        let tree_text = TREE_STAGES[stage];
        let tree_color = if self.completed { Color::Green } else { Color::Rgb(100, 180, 255) };

        let tree_lines: Vec<Line> = std::iter::once(Line::from(Span::styled(
            "  File System:",
            Style::default().fg(Color::Rgb(180, 180, 180)).add_modifier(Modifier::BOLD),
        )))
        .chain(tree_text.lines().map(|l| {
            let styled = if l.contains(".git") {
                Span::styled(
                    format!("  {l}"),
                    Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(format!("  {l}"), Style::default().fg(tree_color))
            };
            Line::from(styled)
        }))
        .collect();

        let tree_widget = Paragraph::new(tree_lines)
            .block(Block::default().borders(Borders::LEFT).border_style(
                Style::default().fg(Color::Rgb(60, 60, 80)),
            ));
        frame.render_widget(tree_widget, chunks[1]);

        // Input
        let bg_color = if self.flash { Color::Rgb(80, 20, 20) } else { Color::Rgb(20, 20, 30) };
        let cursor_str = if self.completed { "" } else { "▌" };

        let mut input_spans = vec![Span::styled(
            "  $ ",
            Style::default().fg(Color::Rgb(100, 255, 100)).add_modifier(Modifier::BOLD),
        )];

        // Show typed chars with coloring
        for (i, ch) in TARGET.chars().enumerate() {
            if i < self.typed.len() {
                input_spans.push(Span::styled(
                    ch.to_string(),
                    Style::default().fg(Color::Rgb(100, 255, 100)).add_modifier(Modifier::BOLD),
                ));
            } else if i == self.typed.len() {
                input_spans.push(Span::styled(
                    cursor_str.to_string(),
                    Style::default().fg(Color::White).add_modifier(Modifier::SLOW_BLINK),
                ));
                break;
            }
        }

        if self.completed {
            input_spans.push(Span::styled(
                " ✓  Repository initialized!",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ));
        }

        let input_para = Paragraph::new(Line::from(input_spans))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if self.flash {
                        Color::Red
                    } else {
                        Color::Rgb(60, 60, 80)
                    }))
                    .title(Span::styled(
                        " Terminal ",
                        Style::default().fg(Color::Rgb(240, 80, 50)),
                    ))
                    .style(Style::default().bg(bg_color)),
            );
        frame.render_widget(input_para, chunks[2]);
    }
}

// gitlings/renderer.rs — Minimal rustlings-style rendering for Gitlings mode

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;
// Exercise type used via App reference

const BG: Color = Color::Rgb(10, 10, 18);
const GREEN: Color = Color::Rgb(60, 220, 100);
const RED: Color = Color::Rgb(220, 60, 60);
const GRAY: Color = Color::Rgb(140, 140, 140);
const WHITE: Color = Color::Rgb(220, 220, 220);

// ── Menu ──────────────────────────────────────────────────────────────────────

pub fn draw_gitlings_menu(frame: &mut Frame, app: &App, selected: usize) {
    let area = frame.area();
    let exercises = &app.gitlings_exercises;

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Gitlings",
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  Type real git commands. No hand-holding.",
            Style::default().fg(GRAY).add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
    ];

    for (i, ex) in exercises.iter().enumerate() {
        let is_selected = i == selected;
        let is_done = app.gitlings_progress.get(i).copied().unwrap_or(false);

        let marker = if is_selected { "▶ " } else { "  " };
        let check = if is_done { "✓" } else { " " };
        let name = ex.name;

        let style = if is_selected {
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD)
        } else if is_done {
            Style::default().fg(GREEN)
        } else {
            Style::default().fg(GRAY)
        };

        lines.push(Line::from(Span::styled(
            format!("  {}[{}] {}  {}", marker, check, i + 1, name),
            style,
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [↑↓] Select  [Enter] Start  [Esc] Back",
        Style::default().fg(Color::DarkGray),
    )));

    let p = Paragraph::new(lines)
        .style(Style::default().bg(BG))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(GRAY)));
    frame.render_widget(p, area);
}

// ── Exercise ──────────────────────────────────────────────────────────────────

use crate::git_sandbox::GitSandbox;

pub struct GitlingsExerciseState {
    pub input: String,
    pub completed: bool,
    pub output: String,
    pub output_is_error: bool,
    pub sandbox: Option<GitSandbox>,
}

impl Default for GitlingsExerciseState {
    fn default() -> Self {
        Self {
            input: String::new(),
            completed: false,
            output: String::new(),
            output_is_error: false,
            sandbox: None,
        }
    }
}

impl GitlingsExerciseState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn draw_gitlings_exercise(
    frame: &mut Frame,
    app: &App,
    ex_idx: usize,
) {
    let area = frame.area();
    let exercises = &app.gitlings_exercises;
    let ex = match exercises.get(ex_idx) {
        Some(e) => e,
        None => return,
    };
    let state = &app.gitlings_state;
    let total = exercises.len();

    // Layout: description top, output middle, input bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // ── Top: description ─────────────────────────────────────────────────────
    let mut desc_lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!("  Gitlings — {}/{}  {}", ex_idx + 1, total, ex.name),
            Style::default().fg(GRAY),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", ex.description),
            Style::default().fg(WHITE).add_modifier(Modifier::BOLD),
        )),
    ];

    if state.completed {
        desc_lines.push(Line::from(""));
        desc_lines.push(Line::from(Span::styled(
            "  ✓ Success! Press Enter for next exercise.",
            Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
        )));
    } else if !state.output.is_empty() && state.output_is_error {
        desc_lines.push(Line::from(""));
        desc_lines.push(Line::from(Span::styled(
            "  ✗ Try again.",
            Style::default().fg(RED).add_modifier(Modifier::BOLD),
        )));
    }

    let desc = Paragraph::new(desc_lines)
        .style(Style::default().bg(BG))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(GRAY)));
    frame.render_widget(desc, chunks[0]);

    // ── Middle: terminal output ──────────────────────────────────────────────
    let mut out_lines: Vec<Line> = vec![];
    if !state.output.is_empty() {
        for line in state.output.lines() {
            let color = if state.output_is_error { RED } else { WHITE };
            out_lines.push(Line::from(Span::styled(
                format!("  {}", line),
                Style::default().fg(color),
            )));
        }
    } else {
        out_lines.push(Line::from(Span::styled(
            "  Type your command and press Enter.",
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )));
    }

    let out = Paragraph::new(out_lines)
        .style(Style::default().bg(Color::Rgb(8, 8, 14)))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(GRAY))
            .title(Span::styled(" Output ", Style::default().fg(GRAY))));
    frame.render_widget(out, chunks[1]);

    // ── Bottom: input prompt ─────────────────────────────────────────────────
    let prompt_color = if state.completed { GREEN } else { WHITE };
    let input_line = Line::from(vec![
        Span::styled("  $ ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        Span::styled(state.input.as_str(), Style::default().fg(prompt_color)),
        if state.completed {
            Span::styled("", Style::default())
        } else {
            Span::styled("▌", Style::default().fg(WHITE))
        },
    ]);

    let input_widget = Paragraph::new(input_line)
        .style(Style::default().bg(Color::Rgb(8, 8, 14)))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(GRAY))
            .title(Span::styled(" Terminal ", Style::default().fg(GRAY))));
    frame.render_widget(input_widget, chunks[2]);
}

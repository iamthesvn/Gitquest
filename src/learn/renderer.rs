// learn/renderer.rs — Render the Learn mode screens

use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

const BG: Color = Color::Rgb(10, 10, 18);
const ACCENT: Color = Color::Rgb(255, 120, 40);
const GREEN: Color = Color::Rgb(60, 220, 100);
const GOLD: Color = Color::Rgb(255, 215, 0);

// ── Learn Menu ────────────────────────────────────────────────────────────────

pub fn draw_learn_menu(frame: &mut Frame, app: &App, selected: usize) {
    let area = frame.area();
    let lessons = &app.lessons;

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(60), Constraint::Fill(1)])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(horizontal[1]);

    let menu_area = vertical[1];
    let inner_height = menu_area.height.saturating_sub(2); // minus top/bottom borders

    const HEADER_LINES: u16 = 4;
    const FOOTER_LINES: u16 = 1;
    const LESSON_BLOCK: u16 = 3; // title + tagline + blank

    let content_height = inner_height.saturating_sub(HEADER_LINES + FOOTER_LINES);
    let total_blocks = lessons.len() as u16;

    let (max_blocks, needs_scroll) = if total_blocks * LESSON_BLOCK <= content_height {
        (total_blocks, false)
    } else {
        let mb = content_height.saturating_sub(2) / LESSON_BLOCK;
        (mb.max(1), true)
    };

    let scroll = if !needs_scroll {
        0
    } else {
        let half = (max_blocks / 2) as usize;
        if selected <= half {
            0
        } else if selected >= lessons.len() - half {
            lessons.len() - max_blocks as usize
        } else {
            selected - half
        }
    };

    let end = (scroll + max_blocks as usize).min(lessons.len());

    let mut lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  📚  Learn Git",
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            "  Master the basics before the pressure hits.",
            Style::default().fg(Color::Rgb(140, 140, 140)).add_modifier(Modifier::ITALIC),
        )),
        Line::from(""),
    ];

    if needs_scroll && scroll > 0 {
        lines.push(Line::from(Span::styled(
            "  ▲ ...",
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )));
    }

    for i in scroll..end {
        let lesson = &lessons[i];
        let is_selected = i == selected;
        let marker = if is_selected { "▶ " } else { "  " };
        let style = if is_selected {
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD).bg(Color::Rgb(30, 15, 10))
        } else {
            Style::default().fg(Color::Rgb(180, 180, 180))
        };
        lines.push(Line::from(Span::styled(
            format!("  {}{}", marker, lesson.title),
            style,
        )));
        lines.push(Line::from(Span::styled(
            format!("     {}", lesson.tagline),
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )));
        lines.push(Line::from(""));
    }

    if needs_scroll && end < lessons.len() {
        lines.push(Line::from(Span::styled(
            "  ▼ ...",
            Style::default().fg(Color::Rgb(100, 100, 100)),
        )));
    }

    lines.push(Line::from(Span::styled(
        "  [↑↓] Navigate  [Enter] Start lesson  [Esc] Back",
        Style::default().fg(Color::DarkGray),
    )));

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(ACCENT))
                .title(Span::styled(" GitQuest — Learn Mode ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD)))
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().bg(BG));

    let bg = Paragraph::new("").style(Style::default().bg(BG));
    frame.render_widget(bg, area);
    frame.render_widget(menu, menu_area);
}

// ── Learn Lesson ──────────────────────────────────────────────────────────────

#[derive(Clone)]
pub struct LearnLessonState {
    pub showing_result: bool,
}

impl Default for LearnLessonState {
    fn default() -> Self {
        Self { showing_result: false }
    }
}

impl LearnLessonState {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn draw_learn_lesson(
    frame: &mut Frame,
    app: &App,
    lesson_idx: usize,
    step_idx: usize,
) {
    let area = frame.area();
    let lesson = match app.lessons.get(lesson_idx) {
        Some(l) => l,
        None => return,
    };
    let step = match lesson.steps.get(step_idx) {
        Some(s) => s,
        None => return,
    };
    let state = &app.learn_state;
    let total_steps = lesson.steps.len();
    let anim_tick = app.anim_tick;

    // Determine which frame set to show
    let use_result = state.showing_result && !step.result_frames.is_empty();
    let frames = if use_result { step.result_frames } else { step.art_frames };

    // Determine layout: if there's a command and not yet showing result, reserve bottom space
    let has_pending_command = step.command.is_some() && !state.showing_result;
    let main_area = if has_pending_command || (use_result && step.command.is_some()) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);
        chunks[0]
    } else {
        area
    };

    // Top-level split: info panel left, art panel right
    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_area);

    // Left: explanation text
    let mut left_lines: Vec<Line> = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", lesson.title),
            Style::default().fg(ACCENT).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  Step {} of {}", step_idx + 1, total_steps),
            Style::default().fg(Color::Rgb(140, 140, 140)),
        )),
        Line::from(""),
        Line::from(Span::styled(
            format!("  {}", step.title),
            Style::default().fg(Color::Rgb(255, 200, 80)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Wrap text to fit panel width (roughly 40 chars)
    let wrapped = wrap_text(step.text, 40);
    for line in wrapped {
        left_lines.push(Line::from(Span::styled(
            format!("  {}", line),
            Style::default().fg(Color::Rgb(200, 200, 210)),
        )));
    }

    left_lines.push(Line::from(""));

    if has_pending_command {
        left_lines.push(Line::from(Span::styled(
            "  Press Enter to run the command.",
            Style::default().fg(GOLD).add_modifier(Modifier::BOLD),
        )));
    } else if use_result {
        left_lines.push(Line::from(Span::styled(
            "  ✓ Command executed. Press Enter to continue.",
            Style::default().fg(GREEN).add_modifier(Modifier::BOLD),
        )));
    } else {
        left_lines.push(Line::from(Span::styled(
            "  [Enter] Next step   [Esc] Back to menu",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let left_block = Paragraph::new(left_lines)
        .style(Style::default().bg(BG))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(50, 50, 80)))
                .title(Span::styled(" Lesson ", Style::default().fg(ACCENT).add_modifier(Modifier::BOLD))),
        );
    frame.render_widget(left_block, panels[0]);

    // Right: animated ASCII art
    let frame_idx = if frames.len() > 1 {
        (anim_tick / 10) % frames.len()
    } else {
        0
    };
    let art = frames[frame_idx];

    let mut art_lines: Vec<Line> = vec![Line::from("")];
    for line in art.lines() {
        art_lines.push(Line::from(Span::styled(
            format!("  {}", line),
            Style::default().fg(Color::Rgb(100, 200, 130)),
        )));
    }



    let right_block = Paragraph::new(art_lines)
        .style(Style::default().bg(BG))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(40, 80, 40)))
                .title(Span::styled(" Demo ", Style::default().fg(Color::Rgb(100, 200, 130)))),
        );
    frame.render_widget(right_block, panels[1]);

    // Bottom command display (when command is present and not yet run)
    if has_pending_command {
        let bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);
        let cmd_area = bottom[1];

        let cmd_line = Line::from(vec![
            Span::styled("  $ ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(step.command.unwrap(), Style::default().fg(Color::White)),
        ]);

        let cmd_widget = Paragraph::new(cmd_line)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(ACCENT))
                    .title(Span::styled(" Command ", Style::default().fg(ACCENT)))
                    .style(Style::default().bg(Color::Rgb(8, 8, 14))),
            );
        frame.render_widget(cmd_widget, cmd_area);
    }

    // Bottom result banner (after command has run)
    if use_result && step.command.is_some() {
        let bottom = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .split(area);
        let cmd_area = bottom[1];

        let cmd_line = Line::from(vec![
            Span::styled("  $ ", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
            Span::styled(step.command.unwrap(), Style::default().fg(Color::White)),
            Span::styled("   ✓", Style::default().fg(GREEN).add_modifier(Modifier::BOLD)),
        ]);

        let cmd_widget = Paragraph::new(cmd_line)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(GREEN))
                    .title(Span::styled(" Command ", Style::default().fg(GREEN)))
                    .style(Style::default().bg(Color::Rgb(8, 14, 8))),
            );
        frame.render_widget(cmd_widget, cmd_area);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn wrap_text(text: &str, width: usize) -> Vec<String> {
    let mut result = Vec::new();
    for paragraph in text.split('\n') {
        let words: Vec<&str> = paragraph.split_whitespace().collect();
        let mut current_line = String::new();
        for word in words {
            if current_line.len() + word.len() + 1 > width {
                result.push(current_line.trim().to_string());
                current_line = word.to_string();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            }
        }
        if !current_line.is_empty() {
            result.push(current_line.trim().to_string());
        }
    }
    result
}

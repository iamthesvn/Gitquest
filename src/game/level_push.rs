use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::{Level, LevelStatus};

#[derive(Debug, Clone, PartialEq)]
pub enum PushPhase {
    Typing,
    Launching { frame: u8 },
    Done,
}

pub struct LevelPush {
    pub typed: String,
    pub phase: PushPhase,
    pub errors: u32,
    pub first_try_correct: bool,
    pub feedback: Option<String>,
    pub completed: bool,
    pub flash: bool,
    pub flash_timer: u8,
}

impl LevelPush {
    pub fn new() -> Self {
        Self {
            typed: String::new(),
            phase: PushPhase::Typing,
            errors: 0,
            first_try_correct: true,
            feedback: None,
            completed: false,
            flash: false,
            flash_timer: 0,
        }
    }
}

const LOCAL_COMMITS: &[&str] = &[
    "a1b2c3d  Add user authentication module",
    "7f8e9d0  Fix login form validation",
    "4c5d6e1  Initial commit",
];

impl Level for LevelPush {
    fn name(&self) -> &str { "git push" }
    fn description(&self) -> &str { "Launch to Orbit" }
    fn hint(&self) -> &str {
        "Type the full command: git push origin main"
    }

    fn score(&self) -> u32 {
        if !self.completed { return 0; }
        let base = 100u32;
        let penalty = self.errors * 15;
        let first_try_bonus = if self.first_try_correct { 25 } else { 0 };
        base.saturating_sub(penalty).saturating_add(first_try_bonus).min(125)
    }

    fn tick(&mut self) -> LevelStatus {
        if let PushPhase::Launching { frame } = &self.phase.clone() {
            let new_frame = frame + 1;
            if new_frame >= 20 {
                self.phase = PushPhase::Done;
                self.completed = true;
                return LevelStatus::Completed;
            } else {
                self.phase = PushPhase::Launching { frame: new_frame };
            }
        }
        LevelStatus::InProgress
    }

    fn update(&mut self, event: KeyEvent) -> LevelStatus {
        if self.completed { return LevelStatus::Completed; }

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
            self.flash = self.flash_timer > 0;
        }

        match &self.phase.clone() {
            PushPhase::Typing => {
                match event.code {
                    KeyCode::Backspace => {
                        self.typed.pop();
                        self.feedback = None;
                    }
                    KeyCode::Enter => {
                        let cmd = self.typed.trim().to_string();
                        if cmd == "git push origin main" {
                            self.phase = PushPhase::Launching { frame: 0 };
                            self.feedback = Some("Initiating launch sequence...".into());
                        } else if cmd == "git push" {
                            self.errors += 1;
                            self.first_try_correct = false;
                            self.flash = true;
                            self.flash_timer = 4;
                            self.feedback = Some(
                                "Push where? Git needs: git push origin main".into()
                            );
                            self.typed.clear();
                            return LevelStatus::Failed("Incomplete push command".into());
                        } else if cmd.starts_with("git push") {
                            self.errors += 1;
                            self.first_try_correct = false;
                            self.flash = true;
                            self.flash_timer = 4;
                            self.feedback = Some(
                                "Close! But use: git push origin main".into()
                            );
                            self.typed.clear();
                            return LevelStatus::Failed("Wrong push target".into());
                        } else {
                            self.errors += 1;
                            self.first_try_correct = false;
                            self.flash = true;
                            self.flash_timer = 4;
                            self.feedback = Some(format!("Unknown command: '{cmd}'"));
                            self.typed.clear();
                            return LevelStatus::Failed("Unknown command".into());
                        }
                    }
                    KeyCode::Char(c) => {
                        self.typed.push(c);
                        self.feedback = None;
                    }
                    _ => {}
                }
            }
            PushPhase::Launching { .. } => {
                // Animation is driven by app tick, keypresses are absorbed
            }
            PushPhase::Done => {
                self.completed = true;
                return LevelStatus::Completed;
            }
        }

        LevelStatus::InProgress
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(50),
                Constraint::Percentage(50),
            ])
            .split(area);

        // Calculate rocket position based on frame
        let launch_frame = match &self.phase {
            PushPhase::Launching { frame } => *frame as usize,
            PushPhase::Done => 20,
            _ => 0,
        };

        // Top: Remote (orbit / cloud)
        let remote_block_style = if launch_frame >= 18 {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::Rgb(60, 60, 80))
        };

        let remote_commits = if launch_frame >= 18 {
            LOCAL_COMMITS.to_vec()
        } else {
            vec![]
        };

        let mut remote_lines = vec![
            Line::from(Span::styled(
                "  ☁  origin/main (Remote)",
                Style::default().fg(if launch_frame >= 18 { Color::Green } else { Color::DarkGray })
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        if remote_commits.is_empty() {
            remote_lines.push(Line::from(Span::styled(
                "  [ empty — no commits yet ]",
                Style::default().fg(Color::DarkGray),
            )));
        } else {
            for c in &remote_commits {
                remote_lines.push(Line::from(vec![
                    Span::styled("  ● ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled(*c, Style::default().fg(Color::Rgb(150, 200, 150))),
                ]));
            }
        }

        // Animated packet dots (commits traveling up)
        if launch_frame > 5 && launch_frame < 18 {
            let dot_pos = (launch_frame - 5) / 3;
            let dots = "·".repeat(dot_pos + 1);
            remote_lines.push(Line::from(""));
            remote_lines.push(Line::from(Span::styled(
                format!("  {dots}↑ pushing..."),
                Style::default().fg(Color::Rgb(100, 200, 255)).add_modifier(Modifier::BOLD),
            )));
        }

        let remote = Paragraph::new(remote_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(remote_block_style)
                .title(Span::styled(
                    " 🛸 GitHub (origin) ",
                    Style::default().fg(Color::Rgb(100, 150, 255)).add_modifier(Modifier::BOLD),
                )));
        frame.render_widget(remote, chunks[0]);

        // Bottom: Local + Rocket + Input
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
            .split(chunks[1]);

        // Local commits
        let mut local_lines = vec![
            Line::from(Span::styled("  Local commits:", Style::default().fg(Color::Rgb(180, 180, 180)).add_modifier(Modifier::BOLD))),
            Line::from(""),
        ];
        for c in LOCAL_COMMITS {
            local_lines.push(Line::from(vec![
                Span::styled("  ● ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                Span::styled(*c, Style::default().fg(Color::Rgb(200, 200, 200))),
            ]));
        }
        local_lines.push(Line::from(""));

        // Input or status
        let bg = if self.flash { Color::Rgb(80, 20, 20) } else { Color::Rgb(20, 20, 30) };
        match &self.phase {
            PushPhase::Typing => {
                // Highlighted action prompt
                local_lines.push(Line::from(vec![
                    Span::styled("  ❯ ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                    Span::styled(
                        "git push origin main",
                        Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("  then press [Enter]", Style::default().fg(Color::Rgb(180, 180, 180))),
                ]));
                let cursor = "▌";
                local_lines.push(Line::from(vec![
                    Span::styled("  $ ", Style::default().fg(Color::Rgb(100, 255, 100)).add_modifier(Modifier::BOLD)),
                    Span::styled(format!("{}{cursor}", self.typed), Style::default().fg(Color::White)),
                ]));
                if let Some(fb) = &self.feedback {
                    local_lines.push(Line::from(""));
                    local_lines.push(Line::from(Span::styled(
                        format!("  ⚠  {fb}"),
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
                    )));
                }
            }
            PushPhase::Launching { frame } => {
                let countdown = 3u8.saturating_sub(frame / 2);
                if countdown > 0 {
                    local_lines.push(Line::from(Span::styled(
                        format!("  🚀 Launching in {countdown}..."),
                        Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD),
                    )));
                } else {
                    local_lines.push(Line::from(Span::styled(
                        "  🚀 Uploading commits to origin...",
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
                    )));
                }
            }
            PushPhase::Done => {
                local_lines.push(Line::from(Span::styled(
                    "  ✓  Push complete! Remote is up to date.",
                    Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                )));
            }
        }

        let local = Paragraph::new(local_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(if self.flash { Color::Red } else { Color::Rgb(60, 60, 80) }))
                .title(Span::styled(" 🖥  Local Repository ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)))
                .style(Style::default().bg(bg)));
        frame.render_widget(local, bottom_chunks[0]);

        // Rocket animation panel
        let rocket_y = if launch_frame == 0 {
            6usize
        } else {
            6usize.saturating_sub(launch_frame.min(6))
        };

        let rocket_art = &[
            "    /\\   ",
            "   /  \\  ",
            "  | 🔥 | ",
            "  |    | ",
            " /|    |\\ ",
            "/ |    | \\",
            "  | ▓▓ |  ",
            "  |    |  ",
            " /======\\ ",
            "/  PUSH  \\",
        ];

        let mut rocket_lines = vec![Line::from("")];
        for (i, line) in rocket_art.iter().enumerate() {
            let visible = i >= rocket_y.min(rocket_art.len().saturating_sub(1));
            let color = if launch_frame >= 18 {
                Color::DarkGray // landed at top
            } else if visible || launch_frame == 0 {
                Color::Rgb(240, 80, 50)
            } else {
                Color::Reset
            };
            rocket_lines.push(Line::from(Span::styled(*line, Style::default().fg(color).add_modifier(Modifier::BOLD))));
        }

        if launch_frame > 0 && launch_frame < 18 {
            rocket_lines.push(Line::from(Span::styled(
                "  ~  ~  ~  ~",
                Style::default().fg(Color::Rgb(255, 140, 0)),
            )));
        }

        let rocket_widget = Paragraph::new(rocket_lines)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
                .title(Span::styled(" Launchpad ", Style::default().fg(Color::DarkGray))));
        frame.render_widget(rocket_widget, bottom_chunks[1]);
    }
}

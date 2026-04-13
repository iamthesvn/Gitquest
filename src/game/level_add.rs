use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use super::{Level, LevelStatus};

#[derive(Clone, Debug)]
pub struct FileEntry {
    pub name: &'static str,
    pub is_junk: bool,
    pub staged: bool,
}

pub struct LevelAdd {
    pub files: Vec<FileEntry>,
    pub cursor: usize,
    pub submitted: bool,
    pub warning: Option<String>,
    pub errors: u32,
    pub completed: bool,
}

impl LevelAdd {
    pub fn new() -> Self {
        Self {
            files: vec![
                FileEntry { name: "src/main.rs", is_junk: false, staged: false },
                FileEntry { name: "src/lib.rs", is_junk: false, staged: false },
                FileEntry { name: "Cargo.toml", is_junk: false, staged: false },
                FileEntry { name: "README.md", is_junk: false, staged: false },
                FileEntry { name: ".DS_Store", is_junk: true, staged: false },
                FileEntry { name: "thumbs.db", is_junk: true, staged: false },
                FileEntry { name: ".env", is_junk: true, staged: false },
                FileEntry { name: "src/utils.rs", is_junk: false, staged: false },
            ],
            cursor: 0,
            submitted: false,
            warning: None,
            errors: 0,
            completed: false,
        }
    }

    fn staged_junk(&self) -> Vec<&str> {
        self.files
            .iter()
            .filter(|f| f.staged && f.is_junk)
            .map(|f| f.name)
            .collect()
    }

    fn unstaged_source(&self) -> Vec<&str> {
        self.files
            .iter()
            .filter(|f| !f.staged && !f.is_junk)
            .map(|f| f.name)
            .collect()
    }

    fn check_complete(&self) -> bool {
        self.staged_junk().is_empty() && self.unstaged_source().is_empty()
    }
}

impl Level for LevelAdd {
    fn name(&self) -> &str { "git add" }
    fn description(&self) -> &str { "Stage the Cargo" }
    fn hint(&self) -> &str {
        "[↑↓] Navigate  [Space] Stage/Unstage  [A] Stage all  [S] Submit"
    }

    fn score(&self) -> u32 {
        if !self.completed { return 0; }
        let base = 100u32;
        let junk_penalty = self.staged_junk().len() as u32 * 10;
        let miss_penalty = self.unstaged_source().len() as u32 * 5;
        let err_penalty = self.errors * 5;

        // Check alphabetical order bonus
        let staged_names: Vec<&str> = self.files.iter().filter(|f| f.staged).map(|f| f.name).collect();
        let mut sorted = staged_names.clone();
        sorted.sort();
        let alpha_bonus = if staged_names == sorted { 30 } else { 0 };

        (base + alpha_bonus).saturating_sub(junk_penalty + miss_penalty + err_penalty)
    }

    fn update(&mut self, event: KeyEvent) -> LevelStatus {
        if self.completed { return LevelStatus::Completed; }
        self.warning = None;

        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                if self.cursor > 0 { self.cursor -= 1; }
            }
            KeyCode::Down | KeyCode::Char('j') => {
                if self.cursor < self.files.len().saturating_sub(1) {
                    self.cursor += 1;
                }
            }
            KeyCode::Char(' ') | KeyCode::Enter => {
                if let Some(f) = self.files.get_mut(self.cursor) {
                    f.staged = !f.staged;
                }
            }
            KeyCode::Char('a') => {
                // Stage all
                for f in &mut self.files {
                    f.staged = true;
                }
                self.warning = Some("Warning: .env and junk files staged! Use .gitignore in real projects.".into());
            }
            KeyCode::Char('u') => {
                // Unstage all
                for f in &mut self.files {
                    f.staged = false;
                }
            }
            KeyCode::Char('s') => {
                // Submit — collect strings first to avoid borrow conflict
                let junk: Vec<String> = self.files
                    .iter()
                    .filter(|f| f.staged && f.is_junk)
                    .map(|f| f.name.to_string())
                    .collect();
                let missing: Vec<String> = self.files
                    .iter()
                    .filter(|f| !f.staged && !f.is_junk)
                    .map(|f| f.name.to_string())
                    .collect();

                if !junk.is_empty() {
                    self.errors += 1;
                    self.warning = Some(format!(
                        "Staged junk! Unstage: {}  [Tip: use .gitignore]",
                        junk.join(", ")
                    ));
                    return LevelStatus::Failed("Junk files staged".into());
                }
                if !missing.is_empty() {
                    self.errors += 1;
                    self.warning = Some(format!(
                        "Missing source files: {}",
                        missing.join(", ")
                    ));
                    return LevelStatus::Failed("Source files not staged".into());
                }

                self.completed = true;
                return LevelStatus::Completed;
            }
            _ => {}
        }

        if self.check_complete() && !self.submitted {
            // hint the user to submit
        }

        LevelStatus::InProgress
    }

    fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(4),  // 2 instruction lines + bottom border
                Constraint::Min(10),
                Constraint::Length(3),
            ])
            .split(area);

        // Instruction bar — two lines: action keys + what to stage
        let instr = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("  ❯ ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                Span::styled("[Space]", Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
                Span::styled(" stage/unstage  ", Style::default().fg(Color::Rgb(200, 200, 200))),
                Span::styled("[A]", Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
                Span::styled(" stage-all  ", Style::default().fg(Color::Rgb(200, 200, 200))),
                Span::styled("[U]", Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
                Span::styled(" unstage-all  ", Style::default().fg(Color::Rgb(200, 200, 200))),
                Span::styled("[S]", Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
                Span::styled(" submit", Style::default().fg(Color::Rgb(200, 200, 200))),
            ]),
            Line::from(vec![
                Span::styled("    Stage: ", Style::default().fg(Color::Rgb(140, 140, 140))),
                Span::styled("source files (.rs .toml .md)", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled("   Skip: ", Style::default().fg(Color::Rgb(140, 140, 140))),
                Span::styled(".DS_Store  thumbs.db  .env", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
            ]),
        ])
        .block(Block::default().borders(Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
        frame.render_widget(instr, chunks[0]);

        // Split files and staging area
        let panels = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        // Left: working directory
        let working_items: Vec<ListItem> = self.files
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let icon = if f.is_junk { "🗑  " } else { "📄 " };
                let color = if f.is_junk { Color::Red } else { Color::Rgb(100, 200, 100) };
                let prefix = if i == self.cursor { "▶ " } else { "  " };
                let staged_mark = if f.staged { " [staged]" } else { "" };

                let style = if i == self.cursor {
                    Style::default().fg(color).add_modifier(Modifier::BOLD).bg(Color::Rgb(30, 30, 50))
                } else if f.staged {
                    Style::default().fg(Color::DarkGray)
                } else {
                    Style::default().fg(color)
                };

                ListItem::new(Line::from(vec![
                    Span::styled(format!("{prefix}{icon}{}{staged_mark}", f.name), style),
                ]))
            })
            .collect();

        let working_list = List::new(working_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80)))
                .title(Span::styled(" 📁 Working Directory ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD))));
        frame.render_widget(working_list, panels[0]);

        // Right: staging area
        let staged_items: Vec<ListItem> = self.files
            .iter()
            .filter(|f| f.staged)
            .map(|f| {
                let icon = if f.is_junk { "⚠  " } else { "✓  " };
                let color = if f.is_junk { Color::Yellow } else { Color::Green };
                ListItem::new(Line::from(Span::styled(
                    format!("  {icon}{}", f.name),
                    Style::default().fg(color).add_modifier(Modifier::BOLD),
                )))
            })
            .collect();

        let staged_list = List::new(staged_items)
            .block(Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
                .title(Span::styled(" 📦 Staging Area ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))));
        frame.render_widget(staged_list, panels[1]);

        // Bottom: warning / status
        let status_text = if let Some(w) = &self.warning {
            Line::from(Span::styled(format!("  ⚠  {w}"), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)))
        } else if self.completed {
            Line::from(Span::styled("  ✓  Files staged correctly! Proceeding...", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)))
        } else {
            let staged_count = self.files.iter().filter(|f| f.staged && !f.is_junk).count();
            let source_count = self.files.iter().filter(|f| !f.is_junk).count();
            Line::from(vec![
                Span::styled("  Progress: ", Style::default().fg(Color::Rgb(180, 180, 180))),
                Span::styled(format!("{staged_count}/{source_count}"), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" source files staged  ", Style::default().fg(Color::Rgb(180, 180, 180))),
                Span::styled("[S]", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                Span::styled(" to submit", Style::default().fg(Color::Rgb(180, 180, 180))),
            ])
        };

        let status = Paragraph::new(status_text)
            .block(Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
        frame.render_widget(status, chunks[2]);
    }
}

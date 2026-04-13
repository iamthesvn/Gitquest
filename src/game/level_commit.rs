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
pub enum MessageQuality {
    Good,
    Okay,
    Bad,
}

pub struct LevelCommit {
    pub input: String,
    pub quality: Option<MessageQuality>,
    pub fake_sha: String,
    pub vault_anim: u8,
    pub retries: u32,
    pub completed: bool,
    pub feedback: Option<String>,
}

impl LevelCommit {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            quality: None,
            fake_sha: String::new(),
            vault_anim: 0,
            retries: 0,
            completed: false,
            feedback: None,
        }
    }

    fn evaluate_message(msg: &str) -> (MessageQuality, String) {
        let trimmed = msg.trim();

        if trimmed.is_empty() || trimmed.len() < 5 {
            return (MessageQuality::Bad, "Empty or too short. Be descriptive!".into());
        }
        if trimmed.len() > 72 {
            return (
                MessageQuality::Bad,
                format!("Too long! {} chars (max 72). Keep it concise.", trimmed.len()),
            );
        }
        if ["asdf", "test", "fix", "wip", "xxx", "update stuff", "changes", "stuff"].contains(&trimmed.to_lowercase().as_str()) {
            return (MessageQuality::Bad, "This commit message will haunt your git log. Be specific!".into());
        }

        // Check for imperative mood (starts with common verb)
        let verbs = ["Add", "Fix", "Update", "Remove", "Refactor", "Implement",
                     "Change", "Delete", "Create", "Rename", "Move", "Improve",
                     "Extract", "Merge", "Revert", "Test", "Configure", "Enable",
                     "Disable", "Upgrade", "Bump", "Set", "Use", "Replace", "Build"];

        let first_word = trimmed.split_whitespace().next().unwrap_or("");
        let starts_with_verb = verbs.iter().any(|v| first_word.eq_ignore_ascii_case(v));
        let is_descriptive = trimmed.split_whitespace().count() >= 3;

        if starts_with_verb && is_descriptive {
            (MessageQuality::Good, "Excellent commit message! Future you says thank you.".into())
        } else if is_descriptive {
            (
                MessageQuality::Okay,
                "Your message works, but future-you might not understand it. Start with a verb!".into(),
            )
        } else {
            (MessageQuality::Bad, "Too vague. Add more detail and start with an action verb.".into())
        }
    }

    fn gen_sha() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().subsec_nanos();
        format!("{:07x}", t & 0xFFFFFFF)
    }
}

const VAULT_OPEN: &str = r#"
      ╔═══════════════╗
      ║  ┌─────────┐  ║
      ║  │  FILES  │  ║
      ║  │ staged  │  ║
      ║  └─────────┘  ║
      ╠═══════════════╣
     /                 \
    /    (  OPEN  )     \
   ╔═══════════════════╗
"#;

const VAULT_CLOSED: &str = r#"
      ╔═══════════════╗
      ║███████████████║
      ║█  S E A L E  █║
      ║█      D      █║
      ║███████████████║
      ╠═══╦═══════╦═══╣
      ║ ● ║ ████  ║ ● ║
      ╚═══╩═══════╩═══╝
"#;

impl Level for LevelCommit {
    fn name(&self) -> &str { "git commit" }
    fn description(&self) -> &str { "Seal the Vault" }
    fn hint(&self) -> &str {
        "Start with a verb: Add/Fix/Update/Remove. Keep under 72 chars. Be specific."
    }

    fn score(&self) -> u32 {
        if !self.completed { return 0; }
        let base = match &self.quality {
            Some(MessageQuality::Good) => 100u32,
            Some(MessageQuality::Okay) => 70,
            _ => 50,
        };
        let retry_penalty = self.retries * 20;
        base.saturating_sub(retry_penalty)
    }

    fn tick(&mut self) -> LevelStatus {
        if self.vault_anim > 0 {
            self.vault_anim -= 1;
            if self.vault_anim == 0 {
                self.completed = true;
                return LevelStatus::Completed;
            }
        }
        LevelStatus::InProgress
    }

    fn update(&mut self, event: KeyEvent) -> LevelStatus {
        if self.completed { return LevelStatus::Completed; }

        // While vault animation is playing, absorb keypresses
        if self.vault_anim > 0 {
            return LevelStatus::InProgress;
        }

        match event.code {
            KeyCode::Backspace => {
                self.input.pop();
                self.quality = None;
                self.feedback = None;
            }
            KeyCode::Enter => {
                let (quality, feedback) = Self::evaluate_message(&self.input);
                self.feedback = Some(feedback);
                match quality {
                    MessageQuality::Bad => {
                        self.quality = Some(quality);
                        self.retries += 1;
                        self.input.clear();
                        return LevelStatus::Failed("Bad commit message".into());
                    }
                    _ => {
                        self.quality = Some(quality);
                        self.fake_sha = Self::gen_sha();
                        self.vault_anim = 15; // 1.5s animation
                    }
                }
            }
            KeyCode::Char(c) => {
                if self.input.len() < 100 {
                    self.input.push(c);
                    self.quality = None;
                    self.feedback = None;
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
                Constraint::Length(5),
                Constraint::Min(10),
                Constraint::Length(5),
            ])
            .split(area);

        // Staged files receipt
        let receipt_lines = vec![
            Line::from(Span::styled("  ┌─ Staged Files (ready to commit) ─┐", Style::default().fg(Color::Rgb(100, 200, 100)))),
            Line::from(Span::styled("  │  📄 src/main.rs                  │", Style::default().fg(Color::Rgb(150, 150, 150)))),
            Line::from(Span::styled("  │  📄 src/lib.rs                   │", Style::default().fg(Color::Rgb(150, 150, 150)))),
            Line::from(Span::styled("  │  📄 Cargo.toml  README.md +more  │", Style::default().fg(Color::Rgb(150, 150, 150)))),
            Line::from(Span::styled("  └────────────────────────────────────┘", Style::default().fg(Color::Rgb(100, 200, 100)))),
        ];
        let receipt = Paragraph::new(receipt_lines);
        frame.render_widget(receipt, chunks[0]);

        // Vault + input
        let vault_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(60)])
            .split(chunks[1]);

        // Vault animation
        let vault_text = if self.vault_anim > 7 {
            VAULT_OPEN
        } else if self.vault_anim > 0 {
            VAULT_CLOSED
        } else if self.completed {
            VAULT_CLOSED
        } else {
            VAULT_OPEN
        };

        let vault_color = if self.completed || self.vault_anim <= 7 && self.vault_anim > 0 {
            Color::Green
        } else {
            Color::Rgb(100, 150, 200)
        };

        let vault = Paragraph::new(vault_text)
            .style(Style::default().fg(vault_color).add_modifier(Modifier::BOLD));
        frame.render_widget(vault, vault_chunks[0]);

        // Commit input area
        let input_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(4)])
            .split(vault_chunks[1]);

        // SHA display if sealed
        if !self.fake_sha.is_empty() {
            let sha_line = Line::from(vec![
                Span::styled("  Commit: ", Style::default().fg(Color::Rgb(180, 180, 180))),
                Span::styled(
                    format!("[{}]", self.fake_sha),
                    Style::default().fg(Color::Rgb(255, 165, 0)).add_modifier(Modifier::BOLD),
                ),
                Span::styled("  ← your commit hash", Style::default().fg(Color::DarkGray)),
            ]);
            let sha = Paragraph::new(sha_line);
            frame.render_widget(sha, input_chunks[0]);
        } else {
            let prompt_lines = vec![
                Line::from(vec![
                    Span::styled("  ❯ ", Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD)),
                    Span::styled("Type your commit message, then press ", Style::default().fg(Color::Rgb(200, 200, 200))),
                    Span::styled("[Enter]", Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Good: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                    Span::styled("\"Add user authentication module\"", Style::default().fg(Color::Rgb(150, 220, 150))),
                ]),
                Line::from(vec![
                    Span::styled("  Bad:  ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                    Span::styled("\"fix stuff\"  \"wip\"  empty messages", Style::default().fg(Color::DarkGray)),
                ]),
            ];
            let prompt = Paragraph::new(prompt_lines);
            frame.render_widget(prompt, input_chunks[0]);
        }

        // Input field
        let char_count = self.input.len();
        let count_color = if char_count > 72 { Color::Red } else if char_count > 50 { Color::Yellow } else { Color::Green };
        let cursor = if self.vault_anim == 0 && !self.completed { "▌" } else { "" };

        let input_style = match &self.quality {
            Some(MessageQuality::Good) => Style::default().fg(Color::Green),
            Some(MessageQuality::Okay) => Style::default().fg(Color::Yellow),
            Some(MessageQuality::Bad) => Style::default().fg(Color::Red),
            None => Style::default().fg(Color::White),
        };

        let input_para = Paragraph::new(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(format!("{}{cursor}", self.input), input_style),
        ]))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(match &self.quality {
                Some(MessageQuality::Good) => Color::Green,
                Some(MessageQuality::Okay) => Color::Yellow,
                Some(MessageQuality::Bad) => Color::Red,
                None => Color::Rgb(60, 60, 80),
            }))
            .title(Span::styled(
                format!(" Commit Message ({char_count}/72) "),
                Style::default().fg(count_color),
            )));
        frame.render_widget(input_para, input_chunks[1]);

        // Feedback
        if let Some(fb) = &self.feedback {
            let (icon, color) = match &self.quality {
                Some(MessageQuality::Good) => ("✓ ", Color::Green),
                Some(MessageQuality::Okay) => ("⚠ ", Color::Yellow),
                Some(MessageQuality::Bad) => ("✗ ", Color::Red),
                None => ("  ", Color::White),
            };
            let feedback_para = Paragraph::new(Line::from(vec![
                Span::styled(format!("  {icon}"), Style::default().fg(color).add_modifier(Modifier::BOLD)),
                Span::styled(fb.as_str(), Style::default().fg(color)),
            ]))
            .block(Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
            frame.render_widget(feedback_para, chunks[2]);
        } else {
            let teach = Paragraph::new(Line::from(Span::styled(
                "  Press [Enter] to seal the commit. Bad messages will be rejected.",
                Style::default().fg(Color::DarkGray),
            )))
            .block(Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
            frame.render_widget(teach, chunks[2]);
        }
    }
}

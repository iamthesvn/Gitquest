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
pub enum BranchPhase {
    // Player must type "git branch feature"
    CreateBranch,
    // Player must type "git checkout feature" or "git switch feature"
    CheckoutBranch,
    // Player must reorder 3 items [2, 0, 1] → [0, 1, 2]
    ReorderTask,
    // Player must type "git checkout main" or "git switch main"
    ReturnToMain,
    Done,
}

pub struct LevelBranch {
    pub phase: BranchPhase,
    pub typed: String,
    pub errors: u32,
    pub reorder: Vec<usize>,     // indices representing items
    pub reorder_cursor: usize,
    pub on_feature: bool,
    pub feature_commit: bool,
    pub completed: bool,
    pub start_time: std::time::Instant,
    pub flash: bool,
    pub flash_timer: u8,
}

impl LevelBranch {
    pub fn new() -> Self {
        Self {
            phase: BranchPhase::CreateBranch,
            typed: String::new(),
            errors: 0,
            reorder: vec![2, 0, 1], // scrambled order
            reorder_cursor: 0,
            on_feature: false,
            feature_commit: false,
            completed: false,
            start_time: std::time::Instant::now(),
            flash: false,
            flash_timer: 0,
        }
    }

    fn phase_target(&self) -> &'static str {
        match self.phase {
            BranchPhase::CreateBranch => "git branch feature",
            BranchPhase::CheckoutBranch => "git checkout feature",
            BranchPhase::ReturnToMain => "git checkout main",
            _ => "",
        }
    }

    fn check_typed(&self) -> bool {
        let typed = self.typed.trim();
        match self.phase {
            BranchPhase::CheckoutBranch => {
                typed == "git checkout feature" || typed == "git switch feature"
            }
            BranchPhase::ReturnToMain => {
                typed == "git checkout main" || typed == "git switch main"
            }
            _ => typed == self.phase_target(),
        }
    }

    fn reorder_correct(&self) -> bool {
        self.reorder == [0, 1, 2]
    }

    fn swap_with_above(&mut self) {
        if self.reorder_cursor > 0 {
            self.reorder.swap(self.reorder_cursor - 1, self.reorder_cursor);
            self.reorder_cursor -= 1;
        }
    }

    fn swap_with_below(&mut self) {
        if self.reorder_cursor < self.reorder.len().saturating_sub(1) {
            self.reorder.swap(self.reorder_cursor, self.reorder_cursor + 1);
            self.reorder_cursor += 1;
        }
    }
}

const ITEMS: &[&str] = &[
    "1. Create project structure",
    "2. Write feature logic",
    "3. Add unit tests",
];

impl Level for LevelBranch {
    fn name(&self) -> &str { "git branch" }
    fn description(&self) -> &str { "Fork in the Road" }
    fn hint(&self) -> &str {
        "Create: git branch feature  |  Switch: git checkout feature  |  Return: git checkout main"
    }

    fn score(&self) -> u32 {
        if !self.completed { return 0; }
        let base = 100u32;
        let penalty = self.errors * 10;
        let elapsed = self.start_time.elapsed().as_secs();
        let time_bonus = if elapsed < 30 { 40 } else { 0 };
        base.saturating_sub(penalty).saturating_add(time_bonus).min(140)
    }

    fn update(&mut self, event: KeyEvent) -> LevelStatus {
        if self.completed {
            return LevelStatus::Completed;
        }

        if self.flash_timer > 0 {
            self.flash_timer -= 1;
            self.flash = self.flash_timer > 0;
        }

        // Clone phase to avoid borrow-checker conflicts when mutating self inside match arms.
        let phase = self.phase.clone();

        match phase {
            BranchPhase::CreateBranch
            | BranchPhase::CheckoutBranch
            | BranchPhase::ReturnToMain => {
                match event.code {
                    KeyCode::Backspace => {
                        self.typed.pop();
                    }
                    KeyCode::Enter => {
                        if self.check_typed() {
                            match phase {
                                BranchPhase::CreateBranch => {
                                    self.phase = BranchPhase::CheckoutBranch;
                                }
                                BranchPhase::CheckoutBranch => {
                                    self.on_feature = true;
                                    self.phase = BranchPhase::ReorderTask;
                                }
                                BranchPhase::ReturnToMain => {
                                    self.on_feature = false;
                                    self.phase = BranchPhase::Done;
                                    self.completed = true;
                                    self.typed.clear();
                                    return LevelStatus::Completed;
                                }
                                _ => {}
                            }
                            self.typed.clear();
                        } else {
                            self.errors += 1;
                            self.flash = true;
                            self.flash_timer = 4;
                            let current = self.typed.clone();
                            self.typed.clear();
                            return LevelStatus::Failed(format!("Wrong: '{current}' — try again"));
                        }
                    }
                    KeyCode::Char(c) => {
                        self.typed.push(c);
                    }
                    _ => {}
                }
            }
            BranchPhase::ReorderTask => {
                match event.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        if self.reorder_cursor > 0 {
                            self.reorder_cursor -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if self.reorder_cursor < self.reorder.len().saturating_sub(1) {
                            self.reorder_cursor += 1;
                        }
                    }
                    KeyCode::Char('K') | KeyCode::Char('u') => {
                        self.swap_with_above();
                    }
                    KeyCode::Char('J') | KeyCode::Char('d') => {
                        self.swap_with_below();
                    }
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        if self.reorder_correct() {
                            self.feature_commit = true;
                            self.phase = BranchPhase::ReturnToMain;
                            self.typed.clear();
                        } else {
                            self.errors += 1;
                            self.flash = true;
                            self.flash_timer = 4;
                            return LevelStatus::Failed("Items not in correct order yet".into());
                        }
                    }
                    _ => {}
                }
            }
            BranchPhase::Done => {
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
                Constraint::Length(10),
                Constraint::Min(6),
                Constraint::Length(4),
            ])
            .split(area);

        // Git graph visualization
        let head_on_main = !self.on_feature;
        let has_feature = self.phase != BranchPhase::CreateBranch;
        let has_feature_commit = self.feature_commit;

        let main_color = if head_on_main { Color::Rgb(240, 80, 50) } else { Color::Rgb(100, 200, 100) };
        let feat_color = if !head_on_main { Color::Rgb(240, 80, 50) } else { Color::Rgb(100, 150, 255) };

        let mut graph_lines = vec![
            Line::from(Span::styled("  Git Graph:", Style::default().fg(Color::Rgb(180, 180, 180)).add_modifier(Modifier::BOLD))),
            Line::from(""),
        ];

        // main branch commits
        let head_main = if head_on_main { " ← HEAD" } else { "" };
        graph_lines.push(Line::from(vec![
            Span::styled("  ●─────●─────●", Style::default().fg(main_color).add_modifier(Modifier::BOLD)),
            Span::styled(format!("  main{head_main}"), Style::default().fg(main_color).add_modifier(Modifier::BOLD)),
        ]));

        if has_feature {
            graph_lines.push(Line::from(vec![
                Span::styled("        └─────", Style::default().fg(feat_color)),
                if has_feature_commit {
                    Span::styled("●", Style::default().fg(feat_color).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled("◌", Style::default().fg(feat_color))
                },
                Span::styled(
                    format!("  feature{}", if !head_on_main { " ← HEAD" } else { "" }),
                    Style::default().fg(feat_color).add_modifier(Modifier::BOLD),
                ),
            ]));
        }

        graph_lines.push(Line::from(""));

        let phase_desc = match &self.phase {
            BranchPhase::CreateBranch => "Type: git branch feature",
            BranchPhase::CheckoutBranch => "Type: git checkout feature  (or: git switch feature)",
            BranchPhase::ReorderTask => "You're on 'feature'. Arrange the tasks in order. [K/J] move, [Enter] confirm",
            BranchPhase::ReturnToMain => "Type: git checkout main  (or: git switch main)",
            BranchPhase::Done => "Done!",
        };

        // Action instruction — bright yellow, impossible to miss
        graph_lines.push(Line::from(vec![
            Span::styled(
                "  ❯ ",
                Style::default().fg(Color::Rgb(240, 80, 50)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                phase_desc,
                Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD),
            ),
        ]));

        let graph = Paragraph::new(graph_lines)
            .block(Block::default().borders(Borders::BOTTOM)
                .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
        frame.render_widget(graph, chunks[0]);

        // Input / reorder area
        match &self.phase {
            BranchPhase::ReorderTask => {
                let bg = if self.flash { Color::Rgb(60, 20, 20) } else { Color::Reset };
                let mut lines = vec![
                    Line::from(Span::styled(
                        "  Arrange feature tasks in order: [↑↓] move cursor  [K/J] move item  [Enter] confirm",
                        Style::default().fg(Color::DarkGray),
                    )),
                    Line::from(""),
                ];
                for (i, &item_idx) in self.reorder.iter().enumerate() {
                    let is_cursor = i == self.reorder_cursor;
                    let prefix = if is_cursor { "▶ " } else { "  " };
                    let style = if is_cursor {
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD).bg(Color::Rgb(30, 30, 50))
                    } else {
                        Style::default().fg(Color::Rgb(180, 180, 180))
                    };
                    lines.push(Line::from(Span::styled(
                        format!("  {prefix}{}", ITEMS[item_idx]),
                        style,
                    )));
                }
                if self.reorder_correct() {
                    lines.push(Line::from(""));
                    lines.push(Line::from(Span::styled(
                        "  ✓ Correct order! Press [Enter] to commit this change.",
                        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
                    )));
                }
                let p = Paragraph::new(lines)
                    .block(Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(if self.flash { Color::Red } else { Color::Rgb(60, 60, 80) }))
                        .title(Span::styled(" Feature Branch Task ", Style::default().fg(Color::Rgb(100, 150, 255))))
                        .style(Style::default().bg(bg)));
                frame.render_widget(p, chunks[1]);
            }
            _ => {
                let bg = if self.flash { Color::Rgb(80, 20, 20) } else { Color::Rgb(20, 20, 30) };
                let cursor = "▌";
                let typed_display = format!("{}{cursor}", self.typed);

                let p = Paragraph::new(Line::from(vec![
                    Span::styled("  $ ", Style::default().fg(Color::Rgb(100, 255, 100)).add_modifier(Modifier::BOLD)),
                    Span::styled(typed_display, Style::default().fg(Color::White)),
                ]))
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(if self.flash { Color::Red } else { Color::Rgb(60, 60, 80) }))
                    .title(Span::styled(" Terminal ", Style::default().fg(Color::Rgb(240, 80, 50))))
                    .style(Style::default().bg(bg)));
                frame.render_widget(p, chunks[1]);
            }
        }

        // Info bar
        let info = match &self.phase {
            BranchPhase::CreateBranch => "Branches are cheap in Git. They're just a pointer to a commit.",
            BranchPhase::CheckoutBranch => "HEAD moves to track your current branch. 'git switch' is the modern command.",
            BranchPhase::ReorderTask => "On your feature branch, you can work without affecting main!",
            BranchPhase::ReturnToMain => "Switch back to main — your feature work is safely on its own branch.",
            BranchPhase::Done => "Perfect! You branched, worked, and returned. That's the workflow.",
        };
        let info_para = Paragraph::new(Line::from(Span::styled(
            format!("  💡 {info}"),
            Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
        )))
        .block(Block::default().borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(60, 60, 80))));
        frame.render_widget(info_para, chunks[2]);
    }
}

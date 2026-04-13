pub mod level_add;
pub mod level_branch;
pub mod level_commit;
pub mod level_init;
pub mod level_push;

use crossterm::event::KeyEvent;
use ratatui::{layout::Rect, Frame};

#[derive(Debug, Clone, PartialEq)]
pub enum LevelStatus {
    InProgress,
    Completed,
    Failed(String),
}

pub trait Level {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn update(&mut self, event: KeyEvent) -> LevelStatus;
    fn tick(&mut self) -> LevelStatus { LevelStatus::InProgress }
    fn render(&self, frame: &mut Frame, area: Rect);
    fn score(&self) -> u32;
    fn hint(&self) -> &str;
}

pub fn rank_title(score: u32) -> &'static str {
    match score {
        460..=500 => "Linus Himself",
        400..=459 => "Rebase Legend",
        330..=399 => "Merge Master",
        250..=329 => "Branch Manager",
        _ => "Git Novice",
    }
}

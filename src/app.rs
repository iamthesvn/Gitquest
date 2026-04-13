use std::{
    io,
    time::{Duration, Instant},
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{backend::Backend, Terminal};

use crate::{
    audio::{MusicPlayer, Sound, SoundManager},
    game::{
        level_add::LevelAdd,
        level_branch::LevelBranch,
        level_commit::LevelCommit,
        level_init::LevelInit,
        level_push::LevelPush,
        Level, LevelStatus,
    },
    ui,
};

const TICK_RATE: Duration = Duration::from_millis(100);

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Menu { selected: usize },
    LevelIntro { level: usize },  // player must press Enter/Space to start
    Playing { level: usize },
    LevelComplete { level: usize, score: u32 },
    Transition { next_level: usize, frame: usize },
    GameComplete { total_score: u32 },
    Quit,
}

pub struct SaveData {
    pub current_level: usize,
    pub scores: Vec<u32>,
    pub total_score: u32,
}

impl SaveData {
    pub fn load() -> Self {
        if let Some(path) = save_path() {
            if let Ok(data) = std::fs::read_to_string(&path) {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&data) {
                    let level = json["current_level"].as_u64().unwrap_or(0) as usize;
                    let scores = json["scores"]
                        .as_array()
                        .map(|a| a.iter().map(|v| v.as_u64().unwrap_or(0) as u32).collect())
                        .unwrap_or_else(|| vec![0; 5]);
                    let total = json["total_score"].as_u64().unwrap_or(0) as u32;
                    return Self { current_level: level, scores, total_score: total };
                }
            }
        }
        Self { current_level: 0, scores: vec![0; 5], total_score: 0 }
    }

    pub fn save(&self) {
        if let Some(path) = save_path() {
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let json = serde_json::json!({
                "current_level": self.current_level,
                "scores": self.scores,
                "total_score": self.total_score,
            });
            let _ = std::fs::write(path, json.to_string());
        }
    }
}

fn save_path() -> Option<std::path::PathBuf> {
    dirs::home_dir().map(|h| h.join(".gitquest").join("save.json"))
}

pub struct App {
    pub state: AppState,
    pub save: SaveData,
    pub sound: SoundManager,
    pub music: MusicPlayer,
    pub levels: Vec<Box<dyn Level>>,
    pub show_hint: bool,
    pub hint_timer: u8,
    pub music_tick_counter: u8, // tick every ~500ms (5 ticks of 100ms)
    pub anim_tick: usize,       // global animation frame counter, increments every tick
}

impl App {
    pub fn new() -> Self {
        let save = SaveData::load();
        Self {
            state: AppState::Menu { selected: 0 },
            save,
            sound: SoundManager::new(),
            music: MusicPlayer::new(),
            levels: Self::build_levels(),
            show_hint: false,
            hint_timer: 0,
            music_tick_counter: 0,
            anim_tick: 0,
        }
    }

    pub fn toggle_mute(&mut self) {
        self.music.toggle_mute();
    }

    pub fn is_muted(&self) -> bool {
        self.music.is_muted()
    }

    fn build_levels() -> Vec<Box<dyn Level>> {
        vec![
            Box::new(LevelInit::new()),
            Box::new(LevelAdd::new()),
            Box::new(LevelCommit::new()),
            Box::new(LevelBranch::new()),
            Box::new(LevelPush::new()),
        ]
    }

    #[allow(dead_code)]
    pub fn current_level(&self) -> Option<&dyn Level> {
        match &self.state {
            AppState::Playing { level } | AppState::LevelComplete { level, .. } => {
                self.levels.get(*level).map(|l| l.as_ref())
            }
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn current_level_index(&self) -> Option<usize> {
        match &self.state {
            AppState::Playing { level } => Some(*level),
            _ => None,
        }
    }

    pub fn level_count(&self) -> usize {
        self.levels.len()
    }

    pub fn level_name(&self, idx: usize) -> &str {
        self.levels.get(idx).map(|l| l.name()).unwrap_or("")
    }

    pub fn level_hint(&self, idx: usize) -> &str {
        self.levels.get(idx).map(|l| l.hint()).unwrap_or("")
    }

    pub fn level_description(&self, idx: usize) -> &str {
        self.levels.get(idx).map(|l| l.description()).unwrap_or("")
    }

    pub fn tick(&mut self) {
        self.anim_tick = self.anim_tick.wrapping_add(1);
        if self.hint_timer > 0 {
            self.hint_timer -= 1;
        }
        match &self.state.clone() {
            AppState::Transition { next_level, frame } => {
                let new_frame = frame + 1;
                // 30 frames total: 0-14 flood, 15-19 hold, 20-29 drain
                if new_frame >= 30 {
                    if *next_level >= self.levels.len() {
                        let total = self.save.total_score;
                        self.sound.play(Sound::GameComplete);
                        self.state = AppState::GameComplete { total_score: total };
                    } else {
                        self.levels[*next_level] = match *next_level {
                            0 => Box::new(LevelInit::new()),
                            1 => Box::new(LevelAdd::new()),
                            2 => Box::new(LevelCommit::new()),
                            3 => Box::new(LevelBranch::new()),
                            4 => Box::new(LevelPush::new()),
                            _ => return,
                        };
                        self.state = AppState::LevelIntro { level: *next_level };
                    }
                } else {
                    self.state = AppState::Transition { next_level: *next_level, frame: new_frame };
                }
            }
            _ => {}
        }
    }

    /// Called when player presses Enter/Space on the intro screen
    pub fn handle_intro_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
            if let AppState::LevelIntro { level } = &self.state {
                let level = *level;
                self.sound.play(Sound::Correct);
                self.state = AppState::Playing { level };
            }
        }
    }

    pub fn handle_menu_key(&mut self, key: KeyEvent) {
        match &self.state.clone() {
            AppState::Menu { selected } => {
                match key.code {
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.sound.play(Sound::KeyPress);
                        let s = selected.saturating_sub(1);
                        self.state = AppState::Menu { selected: s };
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.sound.play(Sound::KeyPress);
                        let s = (selected + 1).min(2);
                        self.state = AppState::Menu { selected: s };
                    }
                    KeyCode::Enter => {
                        self.sound.play(Sound::Correct);
                        match selected {
                            0 => {
                                // New game
                                self.save = SaveData { current_level: 0, scores: vec![0; 5], total_score: 0 };
                                self.levels = Self::build_levels();
                                self.state = AppState::LevelIntro { level: 0 };
                            }
                            1 => {
                                // Continue
                                let lvl = self.save.current_level.min(self.levels.len().saturating_sub(1));
                                self.state = AppState::LevelIntro { level: lvl };
                            }
                            2 => self.state = AppState::Quit,
                            _ => {}
                        }
                    }
                    KeyCode::Char('q') => self.state = AppState::Quit,
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn handle_playing_key(&mut self, key: KeyEvent) {
        let idx = match &self.state {
            AppState::Playing { level } => *level,
            _ => return,
        };

        if key.code == KeyCode::Char('?') {
            self.show_hint = !self.show_hint;
            self.sound.play(Sound::KeyPress);
            return;
        }
        if key.code == KeyCode::Char('q') && key.modifiers.contains(KeyModifiers::CONTROL) {
            self.state = AppState::Quit;
            return;
        }

        self.sound.play(Sound::KeyPress);

        let status = {
            let level = self.levels[idx].as_mut();
            level.update(key)
        };

        match status {
            LevelStatus::InProgress => {}
            LevelStatus::Completed => {
                let score = self.levels[idx].score();
                self.sound.play(Sound::LevelComplete);
                self.save.scores[idx] = score;
                self.save.total_score = self.save.scores.iter().sum();
                self.save.current_level = (idx + 1).min(self.levels.len());
                self.save.save();
                self.state = AppState::LevelComplete { level: idx, score };
            }
            LevelStatus::Failed(msg) => {
                self.sound.play(Sound::Error);
                let _ = msg;
            }
        }
    }

    pub fn handle_level_complete_key(&mut self, key: KeyEvent) {
        match &self.state.clone() {
            AppState::LevelComplete { level, .. } => {
                if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                    self.sound.play(Sound::Transition);
                    self.state = AppState::Transition { next_level: level + 1, frame: 0 };
                }
            }
            _ => {}
        }
    }

    pub fn handle_game_complete_key(&mut self, key: KeyEvent) {
        // 'm' is handled globally (mute toggle); only Enter/q return to menu
        if key.code == KeyCode::Enter || key.code == KeyCode::Char('q') {
            self.sound.play(Sound::KeyPress);
            self.state = AppState::Menu { selected: 0 };
        }
    }
}

pub fn run<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), Box<dyn std::error::Error>>
where
    io::Error: From<B::Error>,
    B::Error: 'static,
{
    let mut app = App::new();
    let mut last_tick = Instant::now();

    loop {
        let size = terminal.size()?;
        if size.width < 80 || size.height < 24 {
            terminal.draw(|f| {
                ui::draw_resize_warning(f);
            })?;
        } else {
            terminal.draw(|f| {
                ui::draw(f, &app);
            })?;
        }

        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::ZERO);

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
                    break;
                }
                // [M] toggles music — but ONLY on non-playing screens so 'm' can
                // be typed freely in commit messages, branch commands, etc.
                // On playing screens the HUD shows "[M] mute" but it only works
                // when not in a text-input level (use Ctrl+M instead there).
                let is_playing = matches!(&app.state, AppState::Playing { .. });
                if !is_playing
                    && (key.code == KeyCode::Char('m') || key.code == KeyCode::Char('M'))
                    && !matches!(&app.state, AppState::Menu { .. })
                {
                    app.toggle_mute();
                }
                match &app.state {
                    AppState::Menu { .. } => app.handle_menu_key(key),
                    AppState::LevelIntro { .. } => app.handle_intro_key(key),
                    AppState::Playing { .. } => app.handle_playing_key(key),
                    AppState::LevelComplete { .. } => app.handle_level_complete_key(key),
                    AppState::GameComplete { .. } => app.handle_game_complete_key(key),
                    AppState::Quit => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            // Tick the current level (for animations)
            if let AppState::Playing { level } = &app.state.clone() {
                let idx = *level;
                let status = app.levels[idx].tick();
                if status == LevelStatus::Completed {
                    let score = app.levels[idx].score();
                    app.sound.play(Sound::LevelComplete);
                    app.save.scores[idx] = score;
                    app.save.total_score = app.save.scores.iter().sum();
                    app.save.current_level = (idx + 1).min(app.levels.len());
                    app.save.save();
                    app.state = AppState::LevelComplete { level: idx, score };
                }
            }
            // Tick music player every 5 ticks (~500ms)
            app.music_tick_counter = app.music_tick_counter.wrapping_add(1);
            if app.music_tick_counter % 5 == 0 {
                app.music.tick();
            }
            app.tick();
            last_tick = Instant::now();
        }

        if app.state == AppState::Quit {
            break;
        }
    }

    Ok(())
}

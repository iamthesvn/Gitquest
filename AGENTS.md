# AGENTS.md — Coding Agent Guide for GitQuest

## Project Overview

GitQuest is a Rust terminal UI game that teaches Git through three modes:
- **Learn** — 11 guided lessons with animated ASCII art demos
- **Game** — 20-chapter narrative across 4 volumes (Halcyon saga)
- **Gitlings** — 5 rustlings-style exercises with real git execution in isolated sandboxes

## Architecture

- **State machine**: `AppState` enum drives the entire UI flow
- **Event loop**: `App::tick()` every 100ms handles animations + input
- **Save system**: `SaveData` persists to `~/.gitquest/save.json` via serde_json
- **Audio**: Synthesised at runtime via rodio — no bundled audio files
- **Git execution**: `GitSandbox` creates temp repos in `/tmp/gitquest-sandbox/`

## Key Conventions

- All user-facing text uses `&'static str` (lessons, dialogue, hints)
- Colors are `Color::Rgb(r, g, b)` — accent is orange `(255, 120, 40)`
- Menu handlers use wrap-around: `(selected + 1) % (max + 1)` for Down, `checked_sub(1).unwrap_or(max)` for Up
- Global mute (`m`) is blocked in `Playing`, `LearnLesson`, and `GitlingsExercise` states so users can type commands

## Testing

- Gitlings exercises have unit tests in `src/gitlings/exercises.rs`
- `GitSandbox` tests verify init + branch creation
- Run `cargo test` before any commit

## Common Tasks

| Task | File(s) |
|------|---------|
| Add a Learn lesson | `src/learn/lessons.rs` |
| Add a Gitlings exercise | `src/gitlings/exercises.rs` |
| Add a story chapter | `src/volumes/story.rs` |
| Change menu behaviour | `src/app.rs` + `src/ui/menu.rs` |
| Change colours/theme | `src/ui/*.rs` (search for `Color::`) |

## Dependencies

- `ratatui 0.30` + `crossterm 0.29` — TUI framework
- `rodio 0.19` — audio synthesis
- `serde` + `serde_json` + `dirs` — save system
- `animate` (git) + `tui-overlay 0.1.2` — animations

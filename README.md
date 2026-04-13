# GitQuest

> Learn Git, one command at a time — in your terminal.

A Ratatui TUI game that teaches the 5 most essential Git commands through
hands-on, interactive levels. No quizzes. No slides. Just do the thing.

```
  ██████╗ ██╗████████╗  ██████╗ ██╗   ██╗███████╗███████╗████████╗
 ██╔════╝ ██║╚══██╔══╝ ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
 ██║  ███╗██║   ██║    ██║   ██║██║   ██║█████╗  ███████╗   ██║
 ██║   ██║██║   ██║    ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║
 ╚██████╔╝██║   ██║    ╚██████╔╝╚██████╔╝███████╗███████║   ██║
  ╚═════╝ ╚═╝   ╚═╝     ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝
```

## Quick start

```bash
git clone https://github.com/your-username/gitquest
cd gitquest
cargo install --path .
gitquest
```

Requires Rust stable. No other dependencies — audio is synthesised at runtime.

## The levels

| # | Command | What you do |
|---|---------|-------------|
| 1 | `git init` | Type the command as a starfield materialises around you |
| 2 | `git add` | Select which files to stage — and learn why `.gitignore` exists |
| 3 | `git commit` | Write a real commit message; a vault door seals your work |
| 4 | `git branch` | Fork the timeline, do work on a feature branch, switch back |
| 5 | `git push` | Launch a rocket — watch commits travel to the remote |

Each level ends with a short "what you just learned" panel, then a Git-logo
flood animation transitions you to the next chapter.

## Controls

| Key | Action |
|-----|--------|
| `Type` | Enter commands / commit messages |
| `↑ ↓` / `j k` | Navigate lists |
| `Enter` | Confirm |
| `Space` | Stage/unstage (Level 2) |
| `M` | Mute / unmute background music |
| `?` | Toggle hint |
| `Ctrl+C` | Quit |

## Features

- State machine architecture — easy to add new levels
- Synthesised ambient music + SFX (no audio files bundled)
- Save / continue via `~/.gitquest/save.json`
- Git-logo flood transition animation between levels
- Score and rank system (Git Novice → Linus Himself)
- Runs on macOS, Linux, and Windows

## Tech

Rust 2021 · [ratatui](https://github.com/ratatui/ratatui) · crossterm · rodio · serde_json · dirs

## Adding a level

1. Create `src/game/level_foo.rs` and implement the `Level` trait
2. Register it in `src/game/mod.rs`
3. Add it to `App::build_levels()` in `src/app.rs`

That's it — the engine is level-agnostic.

## License

MIT

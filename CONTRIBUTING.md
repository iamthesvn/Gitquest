# Contributing to GitQuest

Thanks for your interest in GitQuest! This document helps both human contributors and AI coding agents understand the project.

## Project Structure

```
src/
├── app.rs              # State machine, event loop, save system
├── main.rs             # Entry point, module declarations
├── git_sandbox.rs      # Isolated temp repos for real git execution
├── audio.rs            # Synthesised music + sound effects
├── anim.rs             # Animation state (borders, transitions, typewriter)
├── ui/
│   ├── mod.rs          # Main render dispatcher
│   ├── menu.rs         # Main menu renderer
│   ├── chapter.rs      # In-game chapter screen
│   ├── transition.rs   # Level-complete flood animation
│   ├── toast.rs        # Toast overlay system
│   └── renderer.rs     # Learn mode renderer
├── gitlings/
│   ├── exercises.rs    # Exercise definitions + verify functions
│   └── renderer.rs     # Gitlings menu + exercise screen
├── learn/
│   ├── lessons.rs      # Lesson content (title, text, ASCII art)
│   └── renderer.rs     # Learn menu + lesson screen
└── volumes/
    └── story.rs        # 20 chapters across 4 volumes
```

## Adding a Gitlings Exercise

1. Add `ex0N()` to `src/gitlings/exercises.rs` with `setup` + `verify` functions
2. Append it to `all_exercises()`
3. Run `cargo test` to verify

## Adding a Learn Lesson

1. Add a `Lesson` entry to `src/learn/lessons.rs`
2. Include `art_frames` and `result_frames` as `&'static [&'static str]`
3. Append to `all_lessons()`

## Adding a Story Chapter

1. Add a `Chapter` entry to the relevant volume in `src/volumes/story.rs`
2. Fill: `title`, `scene_art`, `npc_name`, `npc_dialogue`, `task_prompt`, `accepted_answers`, `hints`, `success_message`, `xp`

## Code Style

- Rust 2024 edition
- 4-space indentation
- `snake_case` for functions/variables, `PascalCase` for types
- Prefer `&'static str` for all user-facing text

## Running Tests

```bash
cargo test          # Unit tests
cargo run           # Debug build
cargo run --release # Optimised build
```

## Release Checklist

1. Bump `version` in `Cargo.toml`
2. Update version string in `src/ui/menu.rs`
3. Update `CHANGELOG.md` (if it exists)
4. Tag: `git tag -a vX.Y.Z -m "Release message"`
5. Push tag: `git push origin vX.Y.Z`
6. Create GitHub Release with notes

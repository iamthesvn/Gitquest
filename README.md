# GitQuest

> Learn Git by living it. A terminal game where every command is a real work scenario.

```
  ██████╗ ██╗████████╗  ██████╗ ██╗   ██╗███████╗███████╗████████╗
 ██╔════╝ ██║╚══██╔══╝ ██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝
 ██║  ███╗██║   ██║    ██║   ██║██║   ██║█████╗  ███████╗   ██║
 ██║   ██║██║   ██║    ██║▄▄ ██║██║   ██║██╔══╝  ╚════██║   ██║
 ╚██████╔╝██║   ██║    ╚██████╔╝╚██████╔╝███████╗███████║   ██║
  ╚═════╝ ╚═╝   ╚═╝     ╚══▀▀═╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝
```

You are **Alex Chen**, a new hire at Halcyon. Your teammates are waiting.
Your laptop is blank. Production is about to break. Type the right git
command or fall behind.

---

## Quick start

```bash
git clone https://github.com/your-username/gitquest
cd gitquest
cargo install --path .
gitquest
```

Requires Rust stable (1.85+). No external dependencies — audio is synthesised at runtime, no files to bundle.

---

## The story — Halcyon

GitQuest is structured as **4 volumes** of 5 chapters each (20 chapters total).
Each chapter drops you into a real office scenario. A colleague briefs you,
gives you context, and then you type the git command that solves the problem.

### Volume 1 — First Day at Halcyon
*"You got the job. Now prove you deserve it."*

| Ch | Title | Command |
|----|-------|---------|
| 1 | The Empty Desk | `git init` |
| 2 | The Config Chaos | `git config --global user.name` |
| 3 | The Missing Files | `git add` |
| 4 | The First Commit | `git commit -m` |
| 5 | The Repo Goes Remote | `git push origin main` |

### Volume 2 — The Production Crisis
*"Something is broken in prod. All eyes are on you."*

| Ch | Title | Command |
|----|-------|---------|
| 1 | The Hotfix Branch | `git checkout -b` |
| 2 | The Stash | `git stash` |
| 3 | The Log Detective | `git log` |
| 4 | The Revert | `git revert HEAD` |
| 5 | The Merge | `git merge` |

### Volume 3 — The Politics of Code
*"You've survived the crisis. Now survive the team."*

| Ch | Title | Command |
|----|-------|---------|
| 1 | The Conflict | `git pull origin main` |
| 2 | The Blame Game | `git blame` |
| 3 | The Cherry Pick | `git cherry-pick` |
| 4 | The Cleanup | `git branch -d` |
| 5 | The Tag | `git tag -a` |

### Volume 4 — The Review Room
*"Before it ships, someone has to actually read it."*

| Ch | Title | Command |
|----|-------|---------|
| 1 | The Midnight Edit | `git diff` |
| 2 | The Staged Mistake | `git diff --cached` |
| 3 | The Wall of Text | `git diff --stat` |
| 4 | The Scope Creep | `git diff --name-only` |
| 5 | The Audit Trail | `git diff --name-status` |

---

## The cast

| Character | Role | Personality |
|-----------|------|-------------|
| **Hyett** | Senior dev / your mentor | Calm, precise, quietly rooting for you |
| **Preston** | Product manager | Anxious, dramatic, one deploy from a breakdown |
| **Chacon** | Senior engineer | Blunt, sarcastic, respects competence and nothing else |
| **Wanstrath** | CEO | Visionary, oblivious, occasionally causes the fires he asks you to fight |

---

## Hint system

Every chapter has **3 tiered hints** — you decide how much help you want.

| Key | Action |
|-----|--------|
| `?` | Open / close the hint panel |
| `Shift+H` | Reveal the next hint tier (only when panel is open) |

Hints go from vague nudge → command category → near-exact answer.
Each hint used costs a small XP penalty. You always earn at least 25% of the base XP.

---

## Controls

| Key | Action |
|-----|--------|
| `↑ ↓` / `j k` | Navigate menus |
| `Enter` | Confirm / submit command |
| `Backspace` | Edit your command |
| `?` | Toggle hint panel |
| `Shift+H` | Reveal next hint (panel must be open) |
| `M` | Mute / unmute background music |
| `Esc` | Back / main menu |
| `Ctrl+C` | Quit |

---

## Scoring

- Each chapter has a base XP value (10–40 XP depending on difficulty)
- **-2 XP** per extra attempt after the first
- **-3 XP** per hint tier revealed
- Score floors at **25%** of base — you always earn something
- Commands are matched case-insensitively with collapsed whitespace, so minor formatting differences don't count as wrong

**Ranks** (total XP across all 20 chapters):

| XP | Rank |
|----|------|
| 0–49 | Intern |
| 50–149 | Junior Dev |
| 150–299 | Mid-Level Engineer |
| 300–499 | Senior Dev |
| 500+ | Principal Engineer |

---

## Features

- Narrative-driven gameplay — real office characters, real tension
- 4 volumes × 5 chapters = 20 scenarios covering essential git workflows
- 3-tier hint system with XP penalty — not hand-holding, just scaffolding
- Synthesised ambient music (3 tracks) + sound effects via `rodio` — no audio files
- Git-logo flood transition animation between chapters
- Save / continue system at `~/.gitquest/save.json`
- Terminal minimum 80×24 — shows resize warning if too small
- Runs on macOS, Linux, and Windows

---

## Tech

| | |
|---|---|
| Language | Rust 2024 |
| TUI | [ratatui](https://github.com/ratatui/ratatui) 0.30 + crossterm 0.29 |
| Audio | rodio 0.19 (synthesised — no bundled files) |
| Save | serde_json + dirs |

---

## Adding a chapter

1. Add a new `Chapter { .. }` entry to the relevant volume in `src/volumes/story.rs`
2. Fill in: `title`, `scene_art`, `npc_name`, `npc_dialogue`, `task_prompt`, `accepted_answers`, `hints` (3 items), `success_message`, `xp`
3. That's it — the engine picks it up automatically

To add a whole new volume, append a new `Volume { .. }` to the `vec![]` in `all_volumes()`.

---

## License

MIT

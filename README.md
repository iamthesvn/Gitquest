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

## Three ways to play

### 1. Learn — Guided lessons, zero pressure
11 bite-sized lessons covering the essentials: `init`, `config`, `add`, `commit`, `push`, `branch`, `switch`, `merge`, `status`, `log`, and `pull`. Each lesson shows you what the command does with animated ASCII art and a live demo. No typing required — just read, watch, and absorb.

### 2. Game — The Halcyon saga
20 chapters across 4 volumes. Real office scenarios, real characters, real stakes. Type the right git command to advance. Wrong answers cost XP. Hints are available but penalised. Save progress lives at `~/.gitquest/save.json`.

| Vol | Theme | Commands |
|-----|-------|----------|
| 1 | First Day at Halcyon | `init`, `config`, `add`, `commit`, `push` |
| 2 | The Production Crisis | `checkout -b`, `stash`, `log`, `revert`, `merge` |
| 3 | The Politics of Code | `pull`, `blame`, `cherry-pick`, `branch -d`, `tag` |
| 4 | The Review Room | `diff`, `diff --cached`, `diff --stat`, `diff --name-only`, `diff --name-status` |

### 3. Gitlings — Hands-on, no hand-holding
5 rustlings-style exercises (`init`, `config`, `add`, `commit`, `branch`). You type real git commands into isolated temporary repositories. Commands are actually executed and verified. No story, no XP — just you, a terminal, and git.

---

## Why this structure?

Most Git tutorials fall into one of two traps: **dry reference docs** that list flags in alphabetical order, or **contrived examples** (`git add file.txt`) with no stakes and no memory. You read them, nod, and forget everything the moment a real merge conflict explodes across your terminal.

GitQuest was built on three beliefs:

**1. Context beats syntax.**  
You don't learn `git revert` by reading its man page. You learn it when production is down, your CEO is in the Slack channel, and someone just pushed a broken commit. The 20 chapters are real office scenarios with real characters because that's how human memory works — we remember stories, not bullet points.

**2. The terminal is the classroom.**  
Git is a terminal tool. Teaching it inside a browser with clickable buttons teaches you to click buttons, not to type commands under pressure. GitQuest runs in your terminal because that's where you'll actually use Git.

**3. Scaffolding, not spoon-feeding.**  
The 3-tier hint system exists because getting stuck is frustrating, but being told the answer is useless. Hints cost XP because learning happens in the gap between "I don't know" and "I figured it out." The Learn mode exists for the same reason — watch the concept first, then solve the problem.

The three modes serve different learning moments. **Learn** when you want to see how something works without consequence. **Game** when you want context and stakes. **Gitlings** when you want reps. The four volumes mirror how Git knowledge actually accumulates in a real job: init → config → add → commit → push on day one, then branches, stashes, reverts, and merge conflicts once the codebase catches fire.

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

- **3 modes**: Learn (guided lessons), Game (20-chapter story), Gitlings (hands-on exercises)
- Narrative-driven gameplay — real office characters, real tension
- 4 volumes × 5 chapters = 20 scenarios covering essential git workflows
- 11 Learn lessons with animated ASCII art demonstrations
- 5 Gitlings exercises with real git execution in isolated sandboxes
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

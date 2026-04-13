use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, game::rank_title, story};

// ─── Level-complete scene renderers ──────────────────────────────────────────
// Each fn builds the animated Lines for the left-side "scene" panel.
// `tick` is the global anim_tick counter (increments 10×/sec).

/// Level 1 — git init: stars appear one-by-one, then the .git/ tree materialises.
fn scene_init(tick: usize) -> Vec<Line<'static>> {
    // Stars fill in over 20 ticks; tree appears after tick 20.
    let stars: &[(u16, u16)] = &[
        (2, 4), (5, 1), (8, 7), (1, 10), (6, 3), (3, 14), (9, 6),
        (4, 9), (7, 2), (2, 12), (10, 5), (5, 11), (8, 1), (1, 8),
        (6, 15), (11, 3), (3, 6), (9, 13), (4, 16), (7, 10),
    ];

    let mut grid = vec![vec![' '; 22]; 18];

    let visible = tick.min(stars.len());
    for &(col, row) in &stars[..visible] {
        let r = row as usize;
        let c = col as usize;
        if r < grid.len() && c < grid[r].len() {
            let ch = if tick > stars.len() + 5 { '✦' } else { '·' };
            grid[r][c] = ch;
        }
    }

    let mut lines: Vec<Line<'static>> = vec![
        Line::from(Span::styled(
            " ★  Starfield Initialised",
            Style::default().fg(Color::Rgb(240, 200, 80)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for row in &grid {
        let s: String = row.iter().collect();
        lines.push(Line::from(Span::styled(
            s,
            Style::default().fg(Color::Rgb(80, 120, 200)),
        )));
    }

    // .git/ directory tree fades in after tick 22
    if tick >= 22 {
        let tree_lines = [
            "  project/",
            "  └── .git/",
            "      ├── HEAD",
            "      ├── config",
            "      ├── objects/",
            "      └── refs/",
        ];
        let reveal = (tick - 22).min(tree_lines.len());
        lines.push(Line::from(""));
        for tl in &tree_lines[..reveal] {
            lines.push(Line::from(Span::styled(
                *tl,
                Style::default().fg(Color::Rgb(100, 220, 140)).add_modifier(Modifier::BOLD),
            )));
        }
    }

    lines
}

/// Level 2 — git add: cargo crates slide from left to right staging zone.
fn scene_add(tick: usize) -> Vec<Line<'static>> {
    let files = ["main.rs", "lib.rs", "Cargo.toml", "README.md"];
    // Each file needs 8 ticks to cross; stagger start by 5 ticks each.
    let track_width = 20usize;

    let mut lines = vec![
        Line::from(Span::styled(
            " ⬆  Staging the Cargo",
            Style::default().fg(Color::Rgb(60, 210, 80)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "  Working Dir     Staging",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(Span::styled(
            "  ─────────────── ────────",
            Style::default().fg(Color::Rgb(40, 60, 40)),
        )),
    ];

    for (i, name) in files.iter().enumerate() {
        let start_tick = i * 6;
        let pos = if tick >= start_tick {
            ((tick - start_tick) * 2).min(track_width)
        } else {
            0
        };
        let staged = pos >= track_width;

        let crate_icon = if staged { "📦" } else { "🗃 " };
        let track: String = if staged {
            format!("{:width$}→ {crate_icon} {name}", "", width = track_width)
        } else {
            let spaces = " ".repeat(pos);
            format!("  {spaces}{crate_icon} {name}")
        };

        let color = if staged {
            Color::Rgb(100, 220, 100)
        } else {
            Color::Rgb(200, 180, 100)
        };
        lines.push(Line::from(Span::styled(track, Style::default().fg(color))));
    }

    lines
}

/// Level 3 — git commit: vault door swings shut, hash materialises.
fn scene_commit(tick: usize) -> Vec<Line<'static>> {
    // Vault door frames: door opens wide, swings closed over ~15 ticks
    let door_frames: &[&[&str]] = &[
        &["  ╔══════════╗  ", "  ║          ║  ", "  ║  STAGED  ║  ", "  ║          ║  ", "  ╚══════════╝  "],
        &["  ╔═════════╗   ", "  ║         ║   ", "  ║ STAGING ║   ", "  ║         ║   ", "  ╚═════════╝   "],
        &["  ╔════════╗    ", "  ║        ║    ", "  ║COMMITING║    ", "  ║        ║    ", "  ╚════════╝    "],
        &["  ╔══════╗      ", "  ║      ║      ", "  ║ VAULT ║      ", "  ║      ║      ", "  ╚══════╝      "],
        &["  ╔════╗        ", "  ║    ║        ", "  ║ 🔒 ║        ", "  ║    ║        ", "  ╚════╝        "],
        &["  ╔══╗          ", "  ║  ║          ", "  ║██║          ", "  ║  ║          ", "  ╚══╝          "],
        &["  ╔╗            ", "  ║║            ", "  ██            ", "  ║║            ", "  ╚╝            "],
        &["  ▐             ", "  ▐             ", "  ▐             ", "  ▐             ", "  ▐             "],
        &["                ", "                ", "  ▓ SEALED ▓    ", "                ", "                "],
    ];

    let frame_idx = (tick / 2).min(door_frames.len() - 1);
    let door = door_frames[frame_idx];

    let mut lines = vec![
        Line::from(Span::styled(
            " 🔒  Vault Sealing…",
            Style::default().fg(Color::Rgb(80, 150, 255)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    let door_color = if frame_idx >= door_frames.len() - 2 {
        Color::Rgb(100, 200, 100)
    } else {
        Color::Rgb(120, 160, 240)
    };

    for row in door {
        lines.push(Line::from(Span::styled(
            *row,
            Style::default().fg(door_color).add_modifier(Modifier::BOLD),
        )));
    }

    // SHA hash types in after vault closes
    if tick >= 16 {
        let hash = "a1b2c3d";
        let revealed = (tick - 16).min(hash.len());
        let partial = &hash[..revealed];
        lines.push(Line::from(""));
        lines.push(Line::from(vec![
            Span::styled("  SHA: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                partial,
                Style::default().fg(Color::Rgb(255, 200, 60)).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                if revealed < hash.len() { "█" } else { "" },
                Style::default().fg(Color::Rgb(255, 200, 60)),
            ),
        ]));
    }

    lines
}

/// Level 4 — git branch: ASCII git graph grows a fork then cursor jumps.
fn scene_branch(tick: usize) -> Vec<Line<'static>> {
    // Phases: 0-5 show main only, 6-12 branch forks off, 13+ cursor moves
    let show_branch = tick >= 6;
    let branch_len = if show_branch { (tick - 6).min(4) } else { 0 };
    let on_feature = tick >= 14;
    let new_commit = tick >= 20;

    let main_indicator = if on_feature { "  " } else { "◉ " };
    let feat_indicator = if on_feature { "◉ " } else { "  " };

    let mut lines = vec![
        Line::from(Span::styled(
            " ⑂  Branching the Timeline",
            Style::default().fg(Color::Rgb(240, 200, 40)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled("  ● ─── ● ─── ●", Style::default().fg(Color::Rgb(120, 180, 255)))),
    ];

    if show_branch {
        let branch_arm = "─── ●".repeat(1);
        let partial = &branch_arm[..((branch_len * 5) / 4).min(branch_arm.len())];
        lines.push(Line::from(Span::styled("              \\", Style::default().fg(Color::Rgb(240, 200, 40)))));
        lines.push(Line::from(Span::styled(
            format!("               {partial}"),
            Style::default().fg(Color::Rgb(240, 200, 40)).add_modifier(Modifier::BOLD),
        )));
    }

    if new_commit {
        lines.push(Line::from(Span::styled(
            "               ─── ◉  ← new commit!",
            Style::default().fg(Color::Rgb(100, 220, 100)).add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  main:    ", Style::default().fg(Color::Rgb(120, 180, 255))),
        Span::styled(main_indicator, Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
        Span::styled("HEAD", Style::default().fg(Color::Rgb(120, 180, 255))),
    ]));

    if show_branch {
        lines.push(Line::from(vec![
            Span::styled("  feature: ", Style::default().fg(Color::Rgb(240, 200, 40))),
            Span::styled(feat_indicator, Style::default().fg(Color::Rgb(255, 255, 100)).add_modifier(Modifier::BOLD)),
            Span::styled("HEAD", Style::default().fg(Color::Rgb(240, 200, 40))),
        ]));
    }

    lines
}

/// Level 5 — git push: rocket climbs from launchpad to orbit.
fn scene_push(tick: usize) -> Vec<Line<'static>> {
    // Sky height = 12 rows; rocket starts at bottom, reaches orbit by tick 20.
    let sky_rows = 10usize;
    let rocket_row = sky_rows.saturating_sub((tick * sky_rows) / 22);

    let rocket = ["  /\\  ", " /  \\ ", " |  | ", " \\__/ ", " |  | "];
    let flame_frames = ["  ▲▲  ", "  ▲▲  ", " ▲▲▲▲ "];
    let flame = flame_frames[tick % flame_frames.len()];

    let orbit_reached = tick >= 20;

    let mut lines = vec![
        Line::from(Span::styled(
            " 🚀  Launching to Orbit",
            Style::default().fg(Color::Rgb(200, 80, 255)).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    // Remote (top)
    if orbit_reached {
        lines.push(Line::from(Span::styled(
            "  ┌─ origin/main ─┐",
            Style::default().fg(Color::Rgb(100, 220, 140)).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(Span::styled(
            "  │  ● a1b2c3d    │",
            Style::default().fg(Color::Rgb(100, 220, 140)),
        )));
        lines.push(Line::from(Span::styled(
            "  └─────────────-─┘",
            Style::default().fg(Color::Rgb(100, 220, 140)),
        )));
    } else {
        lines.push(Line::from(Span::styled(
            "  ┌─ origin/main ─┐",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  │    (empty)    │",
            Style::default().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            "  └────────────────┘",
            Style::default().fg(Color::DarkGray),
        )));
    }

    // Sky (empty rows above rocket)
    for row in 0..sky_rows {
        if row == rocket_row {
            for r in &rocket {
                lines.push(Line::from(Span::styled(
                    *r,
                    Style::default().fg(Color::Rgb(200, 200, 220)).add_modifier(Modifier::BOLD),
                )));
            }
        } else if row == rocket_row + rocket.len() {
            lines.push(Line::from(Span::styled(
                flame,
                Style::default().fg(Color::Rgb(255, 140, 20)).add_modifier(Modifier::BOLD),
            )));
        }
    }

    // Launchpad
    lines.push(Line::from(Span::styled(
        "  ═══════════════════",
        Style::default().fg(Color::Rgb(120, 120, 140)),
    )));
    lines.push(Line::from(Span::styled(
        "  LOCAL  main ●",
        Style::default().fg(Color::Rgb(100, 150, 255)),
    )));

    lines
}

// ─── Level name lookup ───────────────────────────────────────────────────────

const LEVEL_NAMES: &[&str] = &[
    "git init  — Create Your Universe",
    "git add   — Stage the Cargo",
    "git commit— Seal the Vault",
    "git branch— Fork in the Road",
    "git push  — Launch to Orbit",
];

const TROPHY: &[&str] = &[
    r"      ___________      ",
    r"     '._==_==_=_.'     ",
    r"     .-\:      /-.     ",
    r"    | (|:.     |) |    ",
    r"     '-|:.     |-'     ",
    r"       \::.    /       ",
    r"        '::. .'        ",
    r"          ) (          ",
    r"        _.' '._        ",
    r"       '-------'       ",
];

// ─── Public draw functions ────────────────────────────────────────────────────

pub fn draw_level_complete(
    frame: &mut Frame,
    app: &App,
    level: usize,
    score: u32,
    tick: usize,
) {
    let area = frame.area();

    let level_name = app.level_name(level);
    let teach: &[&str] = story::LEVEL_EPILOGUES.get(level).copied().unwrap_or(&[]);

    // Two-column layout: left = animated scene, right = score + teaching text
    let cols = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(28), Constraint::Min(40)])
        .split(area);

    // ── Left: animated scene ──────────────────────────────────────────────────
    let scene_lines: Vec<Line<'static>> = match level {
        0 => scene_init(tick),
        1 => scene_add(tick),
        2 => scene_commit(tick),
        3 => scene_branch(tick),
        4 => scene_push(tick),
        _ => vec![Line::from("")],
    };

    let scene_color = match level {
        0 => Color::Rgb(80, 120, 200),
        1 => Color::Rgb(60, 210, 80),
        2 => Color::Rgb(80, 150, 255),
        3 => Color::Rgb(240, 200, 40),
        4 => Color::Rgb(200, 80, 255),
        _ => Color::Rgb(240, 80, 50),
    };

    let scene_block = Paragraph::new(scene_lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(scene_color))
            .title(Span::styled(
                " Scene ",
                Style::default().fg(scene_color).add_modifier(Modifier::BOLD),
            )),
    );
    frame.render_widget(scene_block, cols[0]);

    // ── Right: score + teaching + continue prompt ─────────────────────────────
    draw_right_panel(frame, cols[1], level, level_name, score, app, teach, tick);
}

fn draw_right_panel(
    frame: &mut Frame,
    area: Rect,
    level: usize,
    level_name: &str,
    score: u32,
    app: &App,
    teach: &[&str],
    tick: usize,
) {
    // Score bar: fill proportionally (max ~150 with bonuses)
    let bar_max = 150usize;
    let bar_width = 24usize;
    let filled = ((score as usize * bar_width) / bar_max).min(bar_width);
    let bar = "▓".repeat(filled) + &"░".repeat(bar_width - filled);

    // Pulsing continue prompt
    let prompt_visible = (tick / 5) % 2 == 0;
    let prompt_text = if prompt_visible {
        "  ▶ Press [Enter] or [Space] →"
    } else {
        "  ▷ Press [Enter] or [Space] →"
    };

    let accent = Color::Rgb(240, 80, 50);

    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            format!("  ✓  Level {} Complete!", level + 1),
            Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
        )),
        Line::from(Span::styled(
            format!("  {level_name}"),
            Style::default().fg(accent).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Score  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{score:>3} pts"),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(Span::styled(
            format!("  [{bar}]"),
            Style::default().fg(Color::Rgb(100, 200, 100)),
        )),
        Line::from(vec![
            Span::styled("  Total  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} pts", app.save.total_score),
                Style::default().fg(Color::Rgb(180, 180, 180)),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  ─── What you learned ─────────────────",
            Style::default().fg(Color::Rgb(50, 60, 80)),
        )),
        Line::from(""),
    ];

    // Reveal teaching lines one-by-one (1 per 4 ticks, after tick 6)
    let reveal_count = if tick >= 6 { (tick - 6) / 4 } else { 0 };
    for line in teach.iter().take(reveal_count) {
        lines.push(Line::from(Span::styled(
            *line,
            Style::default().fg(Color::Rgb(150, 200, 150)),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        prompt_text,
        Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
    )));

    let border_color = Color::Green;
    let p = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(Span::styled(
                " Level Complete ",
                Style::default().fg(border_color).add_modifier(Modifier::BOLD),
            )),
    );
    frame.render_widget(p, area);
}

pub fn draw_game_complete(frame: &mut Frame, app: &App, total_score: u32) {
    let area = frame.area();

    let rank = rank_title(total_score);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Length(60), Constraint::Fill(1)])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(30), Constraint::Fill(1)])
        .split(horizontal[1]);

    let game_area = vertical[1];

    let mut lines = vec![Line::from("")];

    // Trophy
    for tl in TROPHY {
        lines.push(Line::from(Span::styled(
            *tl,
            Style::default()
                .fg(Color::Rgb(255, 215, 0))
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "        GitQuest Complete!",
        Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Score breakdown
    lines.push(Line::from(Span::styled(
        "  ─── Score Breakdown ───────────────────────────",
        Style::default().fg(Color::Rgb(60, 60, 80)),
    )));
    lines.push(Line::from(""));

    for (i, name) in LEVEL_NAMES.iter().enumerate() {
        let score = app.save.scores.get(i).copied().unwrap_or(0);
        let bar_filled = (score * 20 / 150).min(20) as usize;
        let bar = "▓".repeat(bar_filled) + &"░".repeat(20 - bar_filled);
        lines.push(Line::from(vec![
            Span::styled("  L", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} ", i + 1),
                Style::default().fg(Color::Rgb(240, 80, 50)),
            ),
            Span::styled(
                format!("{:<33}", name),
                Style::default().fg(Color::Rgb(150, 150, 150)),
            ),
            Span::styled(
                format!("[{bar}]"),
                Style::default().fg(Color::Rgb(100, 180, 100)),
            ),
            Span::styled(
                format!(" {score:>3}"),
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Total Score: ", Style::default().fg(Color::Rgb(180, 180, 180))),
        Span::styled(
            format!("{total_score}"),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled("  Rank: ", Style::default().fg(Color::Rgb(180, 180, 180))),
        Span::styled(
            rank,
            Style::default()
                .fg(Color::Rgb(255, 215, 0))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    // Story ending
    lines.push(Line::from(""));
    for &story_line in story::GAME_COMPLETE_STORY {
        lines.push(Line::from(Span::styled(
            story_line,
            Style::default().fg(Color::Rgb(150, 200, 150)),
        )));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  [Enter] Back to Menu  [Q] Quit  [M] Toggle Music",
        Style::default().fg(Color::DarkGray),
    )));

    let bg = Paragraph::new("").style(Style::default().bg(Color::Rgb(10, 10, 15)));
    frame.render_widget(bg, area);

    let p = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 215, 0)))
            .title(Span::styled(
                " GitQuest Complete ",
                Style::default()
                    .fg(Color::Rgb(255, 215, 0))
                    .add_modifier(Modifier::BOLD),
            ))
            .title_alignment(Alignment::Center),
    );
    frame.render_widget(p, game_area);
}

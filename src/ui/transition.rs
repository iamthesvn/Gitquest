use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

// The git diamond logo — rendered as a repeating tile.
// Each row stored as a Vec<char> so we can index by char position safely,
// avoiding any UTF-8 byte-slice panics with box-drawing characters.
//
//   ╔══╗
//   ║  ║
// ╔═╩══╩═╗
// ║  GIT ║
// ╚═╦══╦═╝
//   ║  ║
//   ╚══╝

const LOGO_ROWS: &[&str] = &[
    "  ╔══╗  ",
    "  ║  ║  ",
    "╔═╩══╩═╗",
    "║  GIT ║",
    "╚═╦══╦═╝",
    "  ║  ║  ",
    "  ╚══╝  ",
];

// Tile dimensions in characters (not bytes!)
const TILE_H: usize = 7;

// Pre-compute each row as a Vec<char> at runtime to avoid byte-index panics.
// We cache this as a lazy static or just build it per-frame (small, fast enough).
fn logo_row_chars() -> Vec<Vec<char>> {
    LOGO_ROWS.iter().map(|s| s.chars().collect()).collect()
}

pub fn draw_transition(frame: &mut Frame, next_level: usize, anim_frame: usize) {
    let area = frame.area();
    let width = area.width as usize;
    let height = area.height as usize;

    if width == 0 || height == 0 {
        return;
    }

    // Build char rows once
    let logo_chars: Vec<Vec<char>> = logo_row_chars();
    // All rows should have the same char-width; use the longest as tile width
    let tile_w = logo_chars.iter().map(|r| r.len()).max().unwrap_or(8);

    let cols = (width + tile_w - 1) / tile_w;
    let rows = (height + TILE_H - 1) / TILE_H;
    let total_tiles = cols * rows;

    if total_tiles == 0 {
        return;
    }

    // Deterministic pseudo-random flood order
    let mut order: Vec<(usize, usize)> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .collect();
    let len = order.len();
    for i in 0..len {
        let j = (i.wrapping_mul(6271).wrapping_add(1337)) % len;
        order.swap(i, j);
    }

    // Phase: 30 ticks total
    //  0-12  → flood in
    //  13-19 → hold
    //  20-29 → drain out
    let visible_count = if anim_frame <= 12 {
        if anim_frame == 0 { 0 } else { (anim_frame * total_tiles) / 12 }
    } else if anim_frame <= 19 {
        total_tiles
    } else {
        let drain = anim_frame - 20;
        total_tiles.saturating_sub((drain + 1) * total_tiles / 10)
    }
    .min(total_tiles);

    let visible: std::collections::HashSet<(usize, usize)> =
        order.iter().take(visible_count).copied().collect();

    // Accent colour per next level
    let accent = match next_level {
        0 => Color::Rgb(240, 80, 50),
        1 => Color::Rgb(60, 210, 80),
        2 => Color::Rgb(80, 150, 255),
        3 => Color::Rgb(240, 200, 40),
        4 => Color::Rgb(200, 80, 255),
        _ => Color::Rgb(240, 80, 50),
    };
    let border_color = Color::Rgb(240, 80, 50); // git orange for structural chars

    let mut lines: Vec<Line> = Vec::with_capacity(height);

    for screen_row in 0..height {
        let tile_row = screen_row / TILE_H;
        let local_row = screen_row % TILE_H;

        // Char row for this logo row (safe char-indexed)
        let row_chars: &[char] = &logo_chars[local_row.min(logo_chars.len() - 1)];

        let mut spans: Vec<Span> = Vec::new();
        let mut screen_col = 0usize;

        while screen_col < width {
            let tile_col = screen_col / tile_w;
            let local_col = screen_col % tile_w;
            let take = (tile_w - local_col).min(width - screen_col);

            // Build the chunk string by char — zero panic risk
            let chunk: String = (local_col..local_col + take)
                .map(|ci| {
                    if ci < row_chars.len() {
                        row_chars[ci]
                    } else {
                        ' '
                    }
                })
                .collect();

            let style = if visible.contains(&(tile_row, tile_col)) {
                // "GIT" letters → accent; structural box chars → git orange; spaces → dim
                let all_structural = chunk
                    .chars()
                    .all(|c| matches!(c, '╔'|'╗'|'╚'|'╝'|'╦'|'╩'|'╣'|'╠'|'═'|'║'|' '));
                let has_letter = chunk.chars().any(|c| c.is_ascii_alphabetic());

                let color = if has_letter {
                    accent
                } else if all_structural {
                    border_color
                } else {
                    Color::Rgb(200, 200, 200)
                };
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Rgb(12, 12, 18))
            };

            spans.push(Span::styled(chunk, style));
            screen_col += take;
        }

        lines.push(Line::from(spans));
    }

    let p = Paragraph::new(lines).style(Style::default().bg(Color::Rgb(8, 8, 14)));
    frame.render_widget(p, area);
}

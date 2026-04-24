use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

// "GIT" in block letters — git orange
const GIT_LOGO: &[&str] = &[
    r"  ██████╗ ██╗████████╗",
    r"  ██╔════╝ ██║╚══██╔══╝",
    r"  ██║  ███╗██║   ██║   ",
    r"  ██║   ██║██║   ██║   ",
    r"  ╚██████╔╝██║   ██║   ",
    r"   ╚═════╝ ╚═╝   ╚═╝   ",
];

// "QUEST" in block letters — same style as GIT, pure Unicode block
const QUEST_LOGO: &[&str] = &[
    r" ██████╗ ██╗   ██╗███████╗███████╗████████╗",
    r"██╔═══██╗██║   ██║██╔════╝██╔════╝╚══██╔══╝",
    r"██║   ██║██║   ██║█████╗  ███████╗   ██║   ",
    r"██║   ██║██║   ██║██╔══╝  ╚════██║   ██║   ",
    r"╚██████╔╝╚██████╔╝███████╗███████║   ██║   ",
    r" ╚═════╝  ╚═════╝ ╚══════╝╚══════╝   ╚═╝   ",
];

const MENU_ITEMS: &[&str] = &["     Learn", "     New Game", "     Continue", "     Quit"];

pub fn draw_menu(frame: &mut Frame, area: Rect, selected: usize, border_breathe: Color) {
    // Wide enough for QUEST logo (44 chars) + padding
    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(54),
            Constraint::Fill(1),
        ])
        .split(area);

    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(28),
            Constraint::Fill(1),
        ])
        .split(horizontal[1]);

    let menu_area = vertical[1];

    let mut lines: Vec<Line> = vec![Line::from("")];

    // GIT in orange block letters
    for logo_line in GIT_LOGO {
        lines.push(Line::from(Span::styled(
            *logo_line,
            Style::default()
                .fg(Color::Rgb(240, 80, 50))
                .add_modifier(Modifier::BOLD),
        )));
    }

    // QUEST in white block letters — same visual weight
    for quest_line in QUEST_LOGO {
        lines.push(Line::from(Span::styled(
            *quest_line,
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    // Menu items
    for (i, item) in MENU_ITEMS.iter().enumerate() {
        let is_selected = i == selected;
        let display = if is_selected {
            format!("  ▶  {}", item.trim())
        } else {
            item.to_string()
        };
        let style = if is_selected {
            Style::default()
                .fg(Color::Rgb(240, 80, 50))
                .add_modifier(Modifier::BOLD)
                .bg(Color::Rgb(30, 15, 10))
        } else {
            Style::default().fg(Color::Rgb(180, 180, 180))
        };
        lines.push(Line::from(Span::styled(display, style)));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "   Learn Git. One command at a time.",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));

    lines.push(Line::from(""));

    lines.push(Line::from(Span::styled(
        "   [↑↓] Navigate  [Enter] Select  [Q] Quit",
        Style::default().fg(Color::Rgb(80, 80, 80)),
    )));

    let border_color = border_breathe;
    let title_color = border_breathe;

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(Span::styled(
                    " GitQuest v1.0 ",
                    Style::default()
                        .fg(title_color)
                        .add_modifier(Modifier::BOLD),
                ))
                .title_alignment(Alignment::Center),
        )
        .alignment(Alignment::Left);

    let bg = Paragraph::new("").style(Style::default().bg(Color::Rgb(10, 10, 15)));
    frame.render_widget(bg, area);
    frame.render_widget(menu, menu_area);
}

use crate::{
    domain::{Position, World},
    tui::{Effect, EffectType, Effects, menu::Menu, syntax, theme::*, viewport::Viewport},
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

/// Renders the main game world (the editor) and visual effects.
pub fn render_world(
    f: &mut Frame,
    world: &World,
    effects: &Effects,
    viewport: &Viewport,
    area: Rect,
) {
    let cursor = world.cursor();
    let enemies = world.enemies().position_set();
    let buffer = world.buffer();
    let mut lines = vec![];

    // Calculate visible area
    let visible_height = area.height.saturating_sub(BORDER_LENGTH) as usize;
    let viewport_line_start = viewport.visible_line_start();
    let end_row = (viewport_line_start + visible_height).min(buffer.rows());

    // Only render visible lines
    for row in viewport_line_start..end_row {
        if let Some(line_content) = buffer.get_line(row) {
            let mut spans = vec![];

            // Tokenize line for syntax highlighting
            let tokens = syntax::tokenize_line(line_content);
            let mut col = 0;

            // Draw each token with appropriate style
            for token in tokens {
                for ch in token.text.chars() {
                    let pos = Position { row, col };

                    // Handle game elements rendering with a hierarchy
                    let (display_ch, style) = if let Some(effect) = effects.get(&pos) {
                        // Render effect
                        draw_effect(effect)
                    } else if pos == cursor.pos() {
                        // Render cursor
                        (
                            PLAYER_CHAR.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        )
                    } else if enemies.contains(&pos) {
                        // Render enemies
                        (
                            ENEMY_CHAR.to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        )
                    } else {
                        // Render text with syntax highlighting
                        (ch.to_string(), token.token_type.style())
                    };

                    spans.push(Span::styled(display_ch, style));
                    col += 1;
                }
            }

            // Add trailing space for empty line handling
            if line_content.is_empty() {
                let pos = Position { row, col };
                let (ch, style) = if pos == cursor.pos() {
                    (
                        PLAYER_CHAR.to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )
                } else if enemies.contains(&pos) {
                    (
                        ENEMY_CHAR.to_string(),
                        Style::default().add_modifier(Modifier::BOLD),
                    )
                } else {
                    (" ".to_string(), Style::default())
                };
                spans.push(Span::styled(ch, style));
            }

            lines.push(Line::from(spans));
        }
    }

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(GAME_TITLE));

    f.render_widget(paragraph, area);
}

/// Renders the status bar at the bottom of the UI.
pub fn render_status_bar<'a>(
    f: &mut Frame,
    game: &World,
    keys_iter: impl Iterator<Item = &'a KeyEvent>,
    area: Rect,
) {
    let cursor = game.cursor().pos();
    let recent_pressed = recent_pressed(keys_iter);
    let status_text = format!(
        "Score: {} | Position: {}:{} | Recent Keys: [{}] | {}",
        game.score(),
        cursor.row,
        cursor.col,
        recent_pressed,
        STATUS_INSTRUCTIONS
    );

    let status = Paragraph::new(status_text)
        .style(Style::default().bg(STATUS_BG_COLOR).fg(STATUS_FG_COLOR))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

/// Renders the file selection UI
pub fn render_file_select(f: &mut Frame, input: &str, error: &Option<String>) {
    let area = f.area();
    let dialog_area = centered_rect(FILE_SELECTION_SIZE.0, FILE_SELECTION_SIZE.1, area);

    let mut text = vec![
        Line::from(Span::styled(
            FILE_SELECTION_TITLE,
            Style::default()
                .fg(MENU_TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(FILE_SELECTION_INSTRUCTION),
        Line::from(""),
        Line::from(vec![
            Span::raw("Path: "),
            Span::styled(input, Style::default().fg(FILE_SELECTION_INPUT_COLOR)),
            Span::styled("_", Style::default().fg(Color::Gray)),
        ]),
    ];

    if let Some(err) = error {
        text.push(Line::from(""));
        text.push(Line::from(Span::styled(
            format!("❌ {}", err),
            Style::default().fg(Color::Red),
        )));
    }

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MENU_LINE_COLOR)),
        )
        .alignment(Alignment::Center);

    f.render_widget(paragraph, dialog_area);
}

/// Renders the main menu UI.
pub fn render_menu(f: &mut Frame, menu: &Menu) {
    let area = f.area();

    let menu_area = centered_rect(MENU_SIZE.0, MENU_SIZE.1, area);

    let title = Paragraph::new(MENU_TITLE)
        .style(
            Style::default()
                .fg(MENU_TITLE_COLOR)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    let items: Vec<ListItem> = menu
        .options()
        .iter()
        .enumerate()
        .map(|(i, opt)| {
            let prefix = if i == menu.selected_idx() { "> " } else { "  " };
            let style = if i == menu.selected_idx() {
                Style::default()
                    .fg(MENU_SELECTED_COLOR)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Line::from(Span::styled(
                format!("{}{}", prefix, opt),
                style,
            )))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(MENU_LINE_COLOR)),
        )
        .style(Style::default());

    let chunks = Layout::default()
        .constraints([Constraint::Length(1), Constraint::Min(0)])
        .split(menu_area);

    f.render_widget(title, chunks[0]);
    f.render_widget(list, chunks[1]);
}

/// Draws a visual effect based on its type and elapsed time.
fn draw_effect(effect: &Effect) -> (String, Style) {
    let elapsed = effect.percentage_elapsed();

    let (ch, color) = match effect.ty {
        EffectType::Collision => {
            if elapsed < 0.25 {
                ("●", Color::White)
            } else if elapsed < 0.5 {
                ("◉", Color::LightYellow)
            } else if elapsed < 0.75 {
                ("○", Color::Yellow)
            } else if elapsed < 1.0 {
                ("∘", Color::DarkGray)
            } else {
                ("·", Color::DarkGray)
            }
        }
        EffectType::Trailing => {
            let brightness = ((1.0 - elapsed) * 255.0) as u8;
            let ch = if elapsed < 0.25 {
                "▓"
            } else if elapsed < 0.5 {
                "▒"
            } else if elapsed < 0.75 {
                "░"
            } else {
                "·"
            };
            (ch, Color::Rgb(brightness, brightness, brightness))
        }
    };

    (
        ch.to_string(),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}

/// Calculates a centered rectangle within a given area.
fn centered_rect(width: u16, height: u16, area: Rect) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

/// Gets a string of recently pressed keys from an iterator.
fn recent_pressed<'a>(keys_iter: impl Iterator<Item = &'a KeyEvent>) -> String {
    let mut keys = vec![];
    for key in keys_iter.take(5) {
        keys.push(key.code.to_string());
    }
    keys.reverse();
    keys.join(" ")
}

use crate::{
    app::GameState,
    ui::{constants::*, viewport::Viewport},
};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Renders the text buffer with the cursor highlighted.
pub fn render_buffer(f: &mut Frame, game: &GameState, viewport: &Viewport, area: Rect) {
    let cursor = game.cursor();
    let buffer = game.buffer();
    let mut lines = vec![];

    // Calculate visible area
    let visible_height = area.height.saturating_sub(BORDER_LENGTH) as usize;
    let viewport_line_start = viewport.visible_line_start();
    let end_row = (viewport_line_start + visible_height).min(buffer.rows());

    // Only render visible lines
    for row in viewport_line_start..end_row {
        if let Some(line_content) = buffer.get_line(row) {
            let mut spans = vec![];

            if row == cursor.row {
                let chars: Vec<char> = line_content.chars().collect();

                for (col, ch) in chars.iter().enumerate() {
                    // Highlight cursor position
                    let style = if col == cursor.col {
                        Style::default()
                            .bg(CURSOR_BG_COLOR)
                            .fg(CURSOR_FG_COLOR)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    spans.push(Span::styled(ch.to_string(), style));
                }

                // Cursor at end of line
                if cursor.col >= chars.len() {
                    // Add a space with highlight - this handles the empty line case
                    spans.push(Span::styled(
                        " ",
                        Style::default()
                            .bg(CURSOR_BG_COLOR)
                            .fg(CURSOR_FG_COLOR)
                            .add_modifier(Modifier::BOLD),
                    ));
                }
            } else {
                // Normal line without cursor
                spans.push(Span::raw(line_content.to_string()));
            }

            lines.push(Line::from(spans));
        }
    }

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(TITLE));

    f.render_widget(paragraph, area);
}

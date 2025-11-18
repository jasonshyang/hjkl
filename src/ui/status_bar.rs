use crate::{app::GameState, ui::constants::*};
use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

/// Renders the status bar at the bottom of the UI.
pub fn render_status_bar(f: &mut Frame, game: &GameState, area: Rect) {
    let cursor = game.cursor();
    let status_text = match game.last_pressed() {
        Some(last_pressed) => format!(
            "Position: {}:{} | Last Pressed: {} | {}",
            cursor.row, cursor.col, last_pressed, STATUS_INSTRUCTIONS
        ),
        None => format!(
            "Position: {}:{} | {}",
            cursor.row, cursor.col, STATUS_INSTRUCTIONS
        ),
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().bg(STATUS_BG_COLOR).fg(STATUS_FG_COLOR))
        .block(Block::default().borders(Borders::ALL));

    f.render_widget(status, area);
}

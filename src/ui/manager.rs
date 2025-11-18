use crate::{
    app::GameState,
    ui::{buffer, constants::STATUS_BAR_HEIGHT, status_bar, viewport::Viewport},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

/// UI Manager handling rendering of different UI components.
#[derive(Default)]
pub struct UiManager {
    viewport: Viewport,
}

impl UiManager {
    pub fn reset(&mut self) {
        self.viewport = Viewport::default();
    }

    /// Renders the entire UI by composing the buffer and status bar.
    ///
    /// Status bar is fixed height at bottom, text buffer takes all available space left.
    ///
    /// ┌─────────────────────┐
    /// │                     │
    /// │  Main buffer view   │
    /// │                     │
    /// ├─────────────────────┤
    /// │  Status bar         │
    /// └─────────────────────┘
    pub fn render(&mut self, f: &mut Frame, game: &GameState) {
        // Define layout chunks
        let chunks = Layout::default()
            .constraints([Constraint::Min(0), Constraint::Length(STATUS_BAR_HEIGHT)])
            .split(f.area());

        // Update viewport based on available height (subtract 2 for borders)
        let visible_height = chunks[0].height.saturating_sub(2) as usize;

        // Updates the viewport based on cursor position and visible area height.
        self.viewport
            .adjust_for_cursor(game.cursor(), game.buffer_lines(), visible_height);

        buffer::render_buffer(f, game, &self.viewport, chunks[0]);
        status_bar::render_status_bar(f, game, chunks[1]);
    }
}

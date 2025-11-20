use crate::{domain::Position, tui::theme::VIEWPORT_PADDING};

/// Manages the viewport offset for scrolling through the buffer.
///
/// The viewport tracks which portion of the buffer is visible on screen.
/// When the cursor moves outside the visible area, the viewport adjusts
/// to keep the cursor in view with some padding.
#[derive(Debug, Clone, Copy)]
pub struct Viewport {
    /// The first line of the buffer that's visible
    visible_line_start: usize,
    /// Number of lines from top/bottom edge before scrolling
    scroll_padding: usize,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            visible_line_start: 0,
            scroll_padding: VIEWPORT_PADDING,
        }
    }
}

impl Viewport {
    /// Returns the current viewport first visible line
    pub fn visible_line_start(&self) -> usize {
        self.visible_line_start
    }

    /// Updates the viewport to keep the cursor visible within the given height.
    pub fn adjust_for_cursor(
        &mut self,
        cursor: Position,
        buffer_lines: usize,
        visible_height: usize,
    ) {
        // No visible area
        if visible_height == 0 {
            return;
        }

        // If cursor is above the viewport, scroll up
        if cursor.row < self.top_threshold() {
            // Set first line so cursor is at padding line
            self.visible_line_start = cursor.row.saturating_sub(self.scroll_padding);
        }

        // If cursor is below the viewport, scroll down
        if cursor.row > self.bottom_threshold(visible_height) {
            // Set first line so cursor is at bottom padding line
            self.visible_line_start = cursor.row + self.scroll_padding + 1 - visible_height;
        }

        // Prevent scrolling pass the last line of the buffer
        // we want to ensure that there are enough lines to fill the visible area
        let remaining_lines = buffer_lines.saturating_sub(visible_height);
        self.visible_line_start = self.visible_line_start.min(remaining_lines);
    }

    /// Returns the top threshold row for scrolling.
    fn top_threshold(&self) -> usize {
        self.visible_line_start + self.scroll_padding
    }

    /// Returns the bottom threshold row for scrolling.
    ///
    /// The cursor must exceed this row to trigger scrolling down.
    ///
    /// e.g.
    /// when self.offset = 10, this means we are viewing lines 10..(10 + visible_height)
    ///
    fn bottom_threshold(&self, visible_height: usize) -> usize {
        self.visible_line_start + visible_height.saturating_sub(self.scroll_padding + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_stays_at_top_initially() {
        let mut viewport = Viewport::default();
        let cursor = Position { row: 2, col: 0 };

        // Starting at row 2, with 100 total lines and 20 rows visible we should not scroll
        viewport.adjust_for_cursor(cursor, 100, 20);

        // Viewport should remain at 0
        assert_eq!(viewport.visible_line_start(), 0);
    }

    #[test]
    fn test_viewport_scrolls_down_when_cursor_near_bottom() {
        let mut viewport = Viewport::default();
        let cursor = Position { row: 18, col: 0 };

        // For 20 rows visible (100 total lines) with initial 0 line start and 3 padding,
        // we expect to start scrolling when cursor exceeds row 16 (0-indexed)
        viewport.adjust_for_cursor(cursor, 100, 20);

        // because cursor is at 18, our first line should now be 2
        assert_eq!(viewport.visible_line_start(), 2);
    }

    #[test]
    fn test_viewport_doesnt_scroll_past_end() {
        let mut viewport = Viewport::default();
        let cursor = Position { row: 98, col: 0 };

        // With 100 lines and visible_height=20, max offset is 80
        viewport.adjust_for_cursor(cursor, 100, 20);
        assert_eq!(viewport.visible_line_start(), 80);
    }
}

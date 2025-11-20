use std::time::Instant;

use crate::domain::{Buffer, Position, motions::Motion, types::BoundedQueue};

/// Memory associated with the cursor, containing stateful information
/// used for various cursor behaviors.
#[derive(Default)]
struct CursorMemory {
    /// Used in vertical motions to remember the target column.
    target_col: Option<usize>,
    /// Track last position
    position_history: BoundedQueue<(Instant, Position)>,
}

/// A cursor within the text buffer.
#[derive(Default)]
pub struct Cursor {
    position: Position,
    memory: CursorMemory,
}

impl Cursor {
    pub fn reset(&mut self) {
        self.position = Position::default();
        self.memory = CursorMemory::default();
    }

    pub fn pos(&self) -> Position {
        self.position
    }

    /// Returns the last `n` recorded cursor positions with their timestamps.
    pub fn last_x_positions(&self, n: usize) -> Vec<(Instant, Position)> {
        let total = self.memory.position_history.len();

        if total <= n {
            self.memory.position_history.iter().cloned().collect()
        } else {
            self.memory
                .position_history
                .iter()
                .skip(total - n)
                .cloned()
                .collect()
        }
    }

    /// Applies the given motion to the cursor position within the provided buffer.
    pub fn apply_motion(&mut self, buffer: &Buffer, motion: Motion, count: Option<usize>) {
        self.memory
            .position_history
            .push((Instant::now(), self.position));

        let count = count.unwrap_or(1);

        let is_vertical = motion.is_vertical();
        self.position = motion.apply(buffer, self.position, count);

        if is_vertical {
            self.position.col = match self.memory.target_col {
                Some(col) => col.min(buffer.get_line_len(self.position.row)),
                None => self.position.col,
            };
        } else {
            self.memory.target_col = Some(self.position.col);
        }
    }
}

#[cfg(test)]
mod cursor_tests {
    use super::*;

    #[test]
    fn test_cursor_last_x_positions() {
        let mut cursor = Cursor::default();
        cursor.position = Position { row: 0, col: 0 };

        for i in 1..=5 {
            cursor
                .memory
                .position_history
                .push((Instant::now(), Position { row: 0, col: i }));
        }

        let last_positions = cursor.last_x_positions(3);
        assert_eq!(last_positions.len(), 3);
        assert_eq!(last_positions[0].1, Position { row: 0, col: 3 });
        assert_eq!(last_positions[1].1, Position { row: 0, col: 4 });
    }
}

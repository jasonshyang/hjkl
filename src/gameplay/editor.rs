use crate::core::{Buffer, Position, generate_random_rust_code_buffer};
use crate::motions::Motion;

#[derive(Default)]
struct EditorMemory {
    /// Used in vertical motions to remember the target column.
    target_col: Option<usize>,
}

pub struct Editor {
    /// The text buffer being viewed.
    buffer: Buffer,
    /// The current cursor position in the buffer.
    cursor: Position,
    /// Editor memory for storing states
    memory: EditorMemory,
}

impl Default for Editor {
    fn default() -> Self {
        let buffer = generate_random_rust_code_buffer();

        Self {
            buffer,
            cursor: Position::default(),
            memory: EditorMemory::default(),
        }
    }
}

impl Editor {
    pub fn reset(&mut self) {
        self.buffer = generate_random_rust_code_buffer();
        self.cursor = Position::default();
        self.memory = EditorMemory::default();
    }

    /// Returns a reference to the current text buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns the number of lines in the buffer.
    pub fn buffer_lines(&self) -> usize {
        self.buffer.rows()
    }

    /// Returns the current cursor position.
    pub fn cursor(&self) -> Position {
        self.cursor
    }

    /// Applies the given motion to the cursor, updating its position.
    pub fn apply_motion(&mut self, motion: Motion, count: Option<usize>) {
        let count = count.unwrap_or(1);

        let is_vertical = motion.is_vertical();
        self.cursor = motion.apply(&self.buffer, self.cursor, count);

        if is_vertical {
            self.cursor.col = match self.memory.target_col {
                Some(col) => col.min(self.buffer.get_line_len(self.cursor.row)),
                None => self.cursor.col,
            };
        } else {
            self.memory.target_col = Some(self.cursor.col);
        }
    }
}

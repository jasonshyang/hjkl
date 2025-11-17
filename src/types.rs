/// Represents different Vim Motions
pub enum Motion {
    Left,               // h
    Down,               // j
    Up,                 // k
    Right,              // l
    WordStart,          // w - start of next word
    WordEnd,            // e - end of current/next word
    WordBackward,       // b - start of previous word
    WORDStart,          // W - start of next WORD
    WORDEnd,            // E - end of current/next WORD
    WORDBackward,       // B - start of previous WORD
}

/// Represents a text buffer
pub struct Buffer(Vec<String>);

impl Buffer {
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn get_line(&self, row: usize) -> Option<&String> {
        self.0.get(row)
    }

    pub fn get_char(&self, pos: &Position) -> Option<char> {
        self.get_line(pos.row)
            .and_then(|line| line.chars().nth(pos.col))
    }

    pub fn is_space(&self, pos: &Position) -> bool {
        match self.get_char(pos) {
            Some(c) => c.is_whitespace(),
            None => true,
        }
    }

    pub fn insert_line(&mut self, row: usize, line: String) {
        self.0.insert(row, line);
    }

    pub fn insert_char(&mut self, pos: Position, c: char) {
        if let Some(line) = self.0.get_mut(pos.row) {
            line.insert(pos.col, c);
        }
    }

    pub fn delete_char(&mut self, pos: Position) {
        if let Some(line) = self.0.get_mut(pos.row) {
            if pos.col < line.len() {
                line.remove(pos.col);
            }
        }
    }
}

/// Represents a position in the text buffer
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Moves the position one character to the right.
    /// 
    /// If at the end of a line, moves to the start of the next line.
    /// 
    /// Returns true if the move was successful
    pub fn move_right(&mut self, buffer: &Buffer) -> bool {
        if let Some(line) = buffer.get_line(self.row) {
            if self.col + 1 < line.len() {
                self.col += 1;
                true
            } else if self.row + 1 < buffer.rows() {
                self.row += 1;
                self.col = 0;
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(lines: Vec<String>) -> Self {
        Buffer(lines)
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Buffer(Vec::new())
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { row: 0, col: 0 }
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let buffer = Buffer(vec![String::from("Hello"), String::from("World")]);

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.move_right(&buffer));
        assert_eq!(pos, Position { row: 0, col: 1 });

        pos.col = 4; // Move to end of "Hello"
        assert!(pos.move_right(&buffer));
        assert_eq!(pos, Position { row: 1, col: 0 }); // Should move to start of "World"

        pos.col = 4; // Move to end of "World"
        assert!(!pos.move_right(&buffer)); // No more lines to move to
    }
}

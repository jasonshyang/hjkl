use std::fmt::Display;

use rand::Rng;

use crate::domain::Position;

/// Represents a text buffer
#[derive(Default, Debug)]
pub struct Buffer(Vec<String>);

impl Buffer {
    /// Returns the number of rows in the buffer including empty lines
    pub fn rows(&self) -> usize {
        self.0.len()
    }

    /// Returns the line at the specified row, or None if out of bounds
    pub fn get_line(&self, row: usize) -> Option<&String> {
        self.0.get(row)
    }

    /// Returns the length of the line at the specified row, or 0 if out of bounds
    pub fn get_line_len(&self, row: usize) -> usize {
        self.get_line(row).map_or(0, |line| line.len())
    }

    /// Returns the character at the specified position, or None if out of bounds
    pub fn get_char(&self, pos: &Position) -> Option<char> {
        self.get_line(pos.row)
            .and_then(|line| line.chars().nth(pos.col))
    }

    /// Returns true if the character at the specified position is whitespace
    ///
    /// Empty line is not considered whitespace
    pub fn is_space(&self, pos: &Position) -> bool {
        match self.get_char(pos) {
            Some(c) => c.is_whitespace(),
            None => false, // empty is not space
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if the line at the specified position is empty
    ///
    /// An empty line is considered one with zero length
    pub fn is_empty_line(&self, pos: &Position) -> bool {
        match self.get_line(pos.row) {
            Some(line) => line.is_empty(),
            None => false,
        }
    }

    /// Inserts a new line at the specified row
    ///
    /// This shifts existing lines down
    pub fn insert_line(&mut self, row: usize, line: String) {
        self.0.insert(row, line);
    }

    /// Inserts a character at the specified position
    ///
    /// This shifts existing characters to the right
    pub fn insert_char(&mut self, pos: Position, c: char) {
        if let Some(line) = self.0.get_mut(pos.row) {
            line.insert(pos.col, c);
        }
    }

    /// Pushes a new line at the end of the buffer
    pub fn push_line(&mut self, line: String) {
        self.0.push(line);
    }

    pub fn delete_char(&mut self, pos: Position) {
        if let Some(line) = self.0.get_mut(pos.row)
            && pos.col < line.len()
        {
            line.remove(pos.col);
        }
    }

    /// Return a random position on the buffer
    pub fn random_position(&self, allow_space: bool) -> Option<Position> {
        if self.is_empty() {
            return None;
        }

        loop {
            let mut rng = rand::rng();
            let row = rng.random_range(0..self.rows());
            let line_len = self.get_line_len(row);
            if line_len == 0 {
                continue;
            }
            let col = rng.random_range(0..=line_len);
            let pos = Position { row, col };
            if allow_space || !self.is_space(&pos) {
                return Some(pos);
            }
        }
    }

    pub fn random_position_from(
        &self,
        start: Position,
        radius: usize,
        allow_space: bool,
    ) -> Option<Position> {
        if self.is_empty() {
            return None;
        }

        let start_row = start.row.saturating_sub(radius);
        let end_row = (start.row + radius).min(self.rows() - 1);
        let start_col = start.col.saturating_sub(radius);
        let end_col = start.col + radius;

        loop {
            let mut rng = rand::rng();
            let row = rng.random_range(start_row..=end_row);
            let line_len = self.get_line_len(row);
            if line_len == 0 {
                continue;
            }

            let col = rng.random_range(start_col.min(line_len)..=end_col.min(line_len));
            let pos = Position { row, col };

            if allow_space || !self.is_space(&pos) {
                return Some(pos);
            }
        }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(lines: Vec<String>) -> Self {
        Buffer(lines)
    }
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;

    #[test]
    fn test_is_space() {
        let lines = vec![
            String::from("Hello World"),
            String::from("   Leading spaces"),
            String::from("Trailing spaces   "),
            String::new(),
        ];

        let buffer = Buffer::from(lines);

        assert!(!buffer.is_space(&Position { row: 0, col: 0 })); // 'H'
        assert!(buffer.is_space(&Position { row: 0, col: 5 })); // space
        assert!(buffer.is_space(&Position { row: 1, col: 0 })); // space
        assert!(!buffer.is_space(&Position { row: 1, col: 3 })); // 'L'
        assert!(!buffer.is_space(&Position { row: 2, col: 0 })); // 'T'
        assert!(buffer.is_space(&Position { row: 2, col: 17 })); // space
        assert!(!buffer.is_space(&Position { row: 3, col: 0 })); // empty line
    }

    #[test]
    fn test_is_empty_line() {
        let lines = vec![
            String::from("Hello World"),
            String::from("   "),
            String::new(),
            String::from("Not empty"),
        ];

        let buffer = Buffer::from(lines);

        assert!(!buffer.is_empty_line(&Position { row: 0, col: 0 })); // "Hello World"
        assert!(!buffer.is_empty_line(&Position { row: 1, col: 0 })); // Space is not empty
        assert!(buffer.is_empty_line(&Position { row: 2, col: 0 })); // ""
        assert!(!buffer.is_empty_line(&Position { row: 3, col: 0 })); // "Not empty"
    }

    #[test]
    fn test_insert_line() {
        let mut buffer = Buffer::from(vec![String::from("Line 1"), String::from("Line 2")]);

        buffer.insert_line(1, String::from("Inserted Line"));
        assert_eq!(buffer.rows(), 3);
        assert_eq!(buffer.get_line(1).unwrap(), "Inserted Line");
        assert_eq!(buffer.get_line(2).unwrap(), "Line 2");
    }

    #[test]
    fn test_insert_char() {
        let mut buffer = Buffer::from(vec![String::from("Hello"), String::from("World")]);

        buffer.insert_char(Position { row: 0, col: 5 }, '!'); // Insert at end of "Hello"
        assert_eq!(buffer.get_line(0).unwrap(), "Hello!");

        buffer.insert_char(Position { row: 1, col: 0 }, 'A'); // Insert at start of "World"
        assert_eq!(buffer.get_line(1).unwrap(), "AWorld");
    }

    #[test]
    fn test_delete_char() {
        let mut buffer = Buffer::from(vec![String::from("Hello"), String::from("World")]);

        buffer.delete_char(Position { row: 0, col: 1 }); // Delete 'e'
        assert_eq!(buffer.get_line(0).unwrap(), "Hllo");

        buffer.delete_char(Position { row: 1, col: 4 }); // Delete 'd'
        assert_eq!(buffer.get_line(1).unwrap(), "Worl");

        buffer.delete_char(Position { row: 1, col: 10 }); // Out of bounds, no change
        assert_eq!(buffer.get_line(1).unwrap(), "Worl");
    }
}

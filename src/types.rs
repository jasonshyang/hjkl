/// Direction for moving in the buffer
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

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

    pub fn delete_char(&mut self, pos: Position) {
        if let Some(line) = self.0.get_mut(pos.row)
            && pos.col < line.len()
        {
            line.remove(pos.col);
        }
    }
}

/// Represents a position in the text buffer
#[derive(Copy, Clone, PartialEq, Eq, Debug, Default)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    /// Moves the position one character to the specified direction.
    ///
    /// If at the end of a line, moves to the start of the next line.
    ///
    /// Returns true if the move was successful
    pub fn move_one_char(&mut self, buffer: &Buffer, direction: Direction) -> bool {
        if let Some(line) = buffer.get_line(self.row) {
            match direction {
                Direction::Forward => {
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
                }
                Direction::Backward => {
                    if self.col > 0 {
                        self.col -= 1;
                        true
                    } else if self.row > 0 {
                        self.row -= 1;
                        if let Some(prev_line) = buffer.get_line(self.row) {
                            self.col = prev_line.len().saturating_sub(1);
                        } else {
                            self.col = 0;
                        }
                        true
                    } else {
                        false
                    }
                }
            }
        } else {
            false
        }
    }

    /// Moves the position one character to the specified direction, skipping over whitespace.
    ///
    /// For example, if positioned at the start of "   Hello", it will move to 'H'.
    pub fn move_one_char_skip_spaces(&mut self, buffer: &Buffer, direction: Direction) -> bool {
        // Move one step first
        if !self.move_one_char(buffer, direction) {
            return false;
        }

        // Keep moving till we are not on space
        while buffer.is_space(self) {
            if !self.move_one_char(buffer, direction) {
                return false;
            }
        }
        true
    }

    /// Moves the position one line to the specified direction.
    pub fn move_one_line(&mut self, buffer: &Buffer, direction: Direction) -> bool {
        match direction {
            Direction::Forward => {
                if self.row + 1 < buffer.rows() {
                    self.row += 1;
                    let next_line = buffer.get_line(self.row).expect("Row should exist");
                    let line_len = next_line.len();

                    // Adjust column if out of bounds
                    self.col = self.col.min(line_len.saturating_sub(1));
                    true
                } else {
                    false
                }
            }
            Direction::Backward => {
                if self.row > 0 {
                    self.row -= 1;
                    let prev_line = buffer.get_line(self.row).expect("Row should exist");
                    let line_len = prev_line.len();

                    // Adjust column if out of bounds
                    self.col = self.col.min(line_len.saturating_sub(1));
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl From<Vec<String>> for Buffer {
    fn from(lines: Vec<String>) -> Self {
        Buffer(lines)
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

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let buffer = vec![String::from("Hello"), String::from("World"), String::new()].into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.move_one_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 0, col: 1 });

        pos.col = 4; // Move to end of "Hello"
        assert!(pos.move_one_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 0 }); // Should move to start of "World"

        pos.col = 4; // Move to end of "World"
        assert!(pos.move_one_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 0 }); // Should move to empty line

        assert!(!pos.move_one_char(&buffer, Direction::Forward)); // At end of buffer, should return false
    }

    #[test]
    fn test_move_left() {
        let buffer = vec![String::from("Hello"), String::from("World"), String::new()].into();

        let mut pos = Position { row: 1, col: 0 }; // Start at beginning of "World"
        assert!(pos.move_one_char(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 4 }); // Should move to end of "Hello"

        assert!(pos.move_one_char(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 3 });

        pos.col = 0; // Move to start of "Hello"
        assert!(!pos.move_one_char(&buffer, Direction::Backward)); // At start of buffer, should return false
    }

    #[test]
    fn test_move_right_skip_spaces() {
        let buffer = vec![String::from("  Hello"), String::from(" World")].into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 0, col: 2 }); // Should skip spaces to 'H'

        pos = Position { row: 0, col: 6 }; // Move to end of "Hello"
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 1 }); // Should skip space to 'W'
    }

    #[test]
    fn test_move_left_skip_spaces() {
        let buffer = vec![String::from("Hello  "), String::from(" World")].into();

        let mut pos = Position { row: 1, col: 6 }; // Start at end of " World"
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 1, col: 5 }); // Should move right by one char to "l"

        pos = Position { row: 1, col: 0 }; // Move to start of " World"
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 4 }); // Should skip spaces to 'o' in "Hello"
    }

    #[test]
    fn test_move_right_skip_spaces_with_empty_line() {
        let buffer = vec![
            String::from("  Hello"),
            String::new(),
            String::from(" World"),
        ]
        .into();

        let mut pos = Position { row: 0, col: 6 }; // Move to end of "Hello"
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 0 }); // Should stop at empty line
    }

    #[test]
    fn test_move_left_skip_spaces_with_empty_line() {
        let buffer = vec![
            String::from("Hello  "),
            String::new(),
            String::from(" World"),
        ]
        .into();

        let mut pos = Position { row: 2, col: 1 }; // Start at beginning of " World"
        assert!(pos.move_one_char_skip_spaces(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 1, col: 0 }); // Should stop at empty line
    }

    #[test]
    fn test_move_one_line() {
        let buffer = vec![
            String::from("Hello"),
            String::from("World"),
            String::from("Rust"),
        ]
        .into();

        let mut pos = Position { row: 0, col: 2 };
        assert!(pos.move_one_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 2 });

        assert!(pos.move_one_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 2 });

        assert!(!pos.move_one_line(&buffer, Direction::Forward)); // At last line

        assert!(pos.move_one_line(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 1, col: 2 });

        assert!(pos.move_one_line(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 2 });

        assert!(!pos.move_one_line(&buffer, Direction::Backward)); // At first line
    }

    #[test]
    fn test_move_one_line_edge_case() {
        let buffer = vec![
            String::from("Short"),
            String::new(),
            String::from("A much longer line"),
        ]
        .into();

        let mut pos = Position { row: 0, col: 1 };

        assert!(pos.move_one_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 0 });

        // This would need to be changed once memorized j motion is implemented
        assert!(pos.move_one_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 0 });
    }
}

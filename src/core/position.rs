use crate::core::{Buffer, Direction};

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
    pub fn step_char(&mut self, buffer: &Buffer, direction: Direction) -> bool {
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
    pub fn step_char_skip_spaces(&mut self, buffer: &Buffer, direction: Direction) -> bool {
        // Move one step first
        if !self.step_char(buffer, direction) {
            return false;
        }

        // Keep moving till we are not on space
        while buffer.is_space(self) {
            if !self.step_char(buffer, direction) {
                return false;
            }
        }
        true
    }

    /// Moves the position one line to the specified direction.
    pub fn step_line(&mut self, buffer: &Buffer, direction: Direction) -> bool {
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

    /// Jumps to the next occurrence of the target character in the specified direction.
    ///
    /// Returns true if the jump was successful.
    pub fn jump_to_char(&mut self, buffer: &Buffer, target: char, direction: Direction) -> bool {
        if let Some(new_pos) = self.find_char(buffer, target, direction) {
            *self = new_pos;
            true
        } else {
            false
        }
    }

    pub fn jump_before_char(
        &mut self,
        buffer: &Buffer,
        target: char,
        direction: Direction,
    ) -> bool {
        if let Some(mut new_pos) = self.find_char(buffer, target, direction) {
            // Move one step back in the opposite direction
            if !new_pos.step_char(buffer, direction.opposite()) {
                return false;
            }

            *self = new_pos;
            true
        } else {
            false
        }
    }

    /// Finds the next occurrence of the target character in the specified direction.
    pub fn find_char(
        &self,
        buffer: &Buffer,
        target: char,
        direction: Direction,
    ) -> Option<Position> {
        let mut pos = *self;

        loop {
            if !pos.step_char(buffer, direction) {
                return None; // Reached the end without finding
            }

            if let Some(c) = buffer.get_char(&pos)
                && c == target
            {
                return Some(pos);
            }
        }
    }
}

#[cfg(test)]
mod position_tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let buffer = vec![String::from("Hello"), String::from("World"), String::new()].into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.step_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 0, col: 1 });

        pos.col = 4; // Move to end of "Hello"
        assert!(pos.step_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 0 }); // Should move to start of "World"

        pos.col = 4; // Move to end of "World"
        assert!(pos.step_char(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 0 }); // Should move to empty line

        assert!(!pos.step_char(&buffer, Direction::Forward)); // At end of buffer, should return false
    }

    #[test]
    fn test_move_left() {
        let buffer = vec![String::from("Hello"), String::from("World"), String::new()].into();

        let mut pos = Position { row: 1, col: 0 }; // Start at beginning of "World"
        assert!(pos.step_char(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 4 }); // Should move to end of "Hello"

        assert!(pos.step_char(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 3 });

        pos.col = 0; // Move to start of "Hello"
        assert!(!pos.step_char(&buffer, Direction::Backward)); // At start of buffer, should return false
    }

    #[test]
    fn test_move_right_skip_spaces() {
        let buffer = vec![String::from("  Hello"), String::from(" World")].into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 0, col: 2 }); // Should skip spaces to 'H'

        pos = Position { row: 0, col: 6 }; // Move to end of "Hello"
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 1 }); // Should skip space to 'W'
    }

    #[test]
    fn test_move_left_skip_spaces() {
        let buffer = vec![String::from("Hello  "), String::from(" World")].into();

        let mut pos = Position { row: 1, col: 6 }; // Start at end of " World"
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 1, col: 5 }); // Should move right by one char to "l"

        pos = Position { row: 1, col: 0 }; // Move to start of " World"
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Backward));
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
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Forward));
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
        assert!(pos.step_char_skip_spaces(&buffer, Direction::Backward));
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
        assert!(pos.step_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 2 });

        assert!(pos.step_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 2 });

        assert!(!pos.step_line(&buffer, Direction::Forward)); // At last line

        assert!(pos.step_line(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 1, col: 2 });

        assert!(pos.step_line(&buffer, Direction::Backward));
        assert_eq!(pos, Position { row: 0, col: 2 });

        assert!(!pos.step_line(&buffer, Direction::Backward)); // At first line
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

        assert!(pos.step_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 1, col: 0 });

        // This would need to be changed once memorized j motion is implemented
        assert!(pos.step_line(&buffer, Direction::Forward));
        assert_eq!(pos, Position { row: 2, col: 0 });
    }

    #[test]
    fn test_jump_to_char() {
        let buffer = vec![
            String::from("Hello World"),
            String::from("This is a test."),
            String::from("Jump to character."),
        ]
        .into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.jump_to_char(&buffer, 'W', Direction::Forward));
        assert_eq!(buffer.get_char(&pos).unwrap(), 'W');

        assert!(pos.jump_to_char(&buffer, 't', Direction::Forward));
        assert_eq!(buffer.get_char(&pos).unwrap(), 't');

        assert!(!pos.jump_to_char(&buffer, 'z', Direction::Forward));
    }

    #[test]
    fn test_jump_before_char() {
        let buffer = vec![
            String::from("Hello World"),
            String::from("This is a test."),
            String::from("Jump to character."),
        ]
        .into();

        let mut pos = Position { row: 0, col: 0 };
        assert!(pos.jump_before_char(&buffer, 'r', Direction::Forward));
        assert_eq!(buffer.get_char(&pos).unwrap(), 'o');

        assert!(pos.jump_before_char(&buffer, 's', Direction::Forward));
        assert_eq!(buffer.get_char(&pos).unwrap(), 'i');

        assert!(!pos.jump_before_char(&buffer, 'z', Direction::Forward));
    }
}

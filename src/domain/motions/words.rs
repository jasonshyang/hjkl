use crate::domain::{Buffer, Direction, Position};

// ===========================================
// w MOTION
// ===========================================

pub fn w_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !w_motion_once(buffer, &mut position) {
            break; // Can't move further
        }
    }
    position
}

// Jump forwards to the start of a word, stop at empty line
pub fn w_motion_once(buffer: &Buffer, position: &mut Position) -> bool {
    let Some(line) = buffer.get_line(position.row) else {
        return false;
    };

    match word_boundaries(line, position.col) {
        // We are on a word, we need to get to next word start
        Some((_, end)) => {
            // We first move to the end of current word
            position.col = end;

            // We then move right and skip all whitespaces
            position.step_char_skip_spaces(buffer, Direction::Forward)
        }
        // We are on a space
        None => {
            // Just need to move right
            position.step_char_skip_spaces(buffer, Direction::Forward)
        }
    }
}

// ===========================================
// b MOTION
// ===========================================

pub fn b_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !b_motion_once(buffer, &mut position) {
            break; // Can't move further
        }
    }
    position
}

/// Jump backwards to the start of a word
pub fn b_motion_once(buffer: &Buffer, position: &mut Position) -> bool {
    let Some(line) = buffer.get_line(position.row) else {
        return false;
    };

    match word_boundaries(line, position.col) {
        // We are on a word, we need to get to previous word start
        Some((start, _)) => {
            if position.col == start {
                // Already at the start of the word, need to move left first
                if !position.step_char_skip_spaces(buffer, Direction::Backward) {
                    return false;
                }

                let Some(line) = buffer.get_line(position.row) else {
                    return false;
                };

                match word_boundaries(line, position.col) {
                    Some((prev_start, _)) => {
                        // Move to the start of the previous word
                        position.col = prev_start;
                        true
                    }
                    None => true,
                }
            } else {
                // Just move to the start of the word
                position.col = start;
                true
            }
        }
        // We are on a space
        None => {
            // Just need to move left
            position.step_char(buffer, Direction::Backward)
        }
    }
}

// ===========================================
// e MOTION
// ===========================================

/// Emulate e motion
pub fn e_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !e_motion_once(buffer, &mut position) {
            break; // Can't move further
        }
    }
    position
}

/// Forward to the end of word |inclusive|. Does not stop in an empty line.
pub fn e_motion_once(buffer: &Buffer, position: &mut Position) -> bool {
    loop {
        // We first move right by one
        if !position.step_char(buffer, Direction::Forward) {
            // If we can't move right, we're at the end of the buffer
            return false;
        }

        let Some(line) = buffer.get_line(position.row) else {
            return false;
        };

        match word_boundaries(line, position.col) {
            // Landed on a word, jump to end
            Some((_, end)) => {
                position.col = end;
                return true;
            }
            // Position on a space, keep moving
            None => continue,
        }
    }
}

// ===========================================
// Utils
// ===========================================

/// Given a line and a starting column, returns the start and end of the current word
///
/// A word consists of:
/// 1. A sequence of letters, digits and underscores, OR
/// 2. A sequence of other non-blank characters
///
/// Separated with white space (spaces, tabs).
///
/// Returns None if positioned on whitespace or if the position is invalid.
pub fn word_boundaries(line: &str, col: usize) -> Option<(usize, usize)> {
    let chars: Vec<char> = line.chars().collect();
    let len = chars.len();

    if len == 0 || col >= len {
        return None;
    }

    // Return None if on whitespace
    if chars[col].is_whitespace() {
        return None;
    }

    let is_word = is_word_char(chars[col]);
    let mut start = col;
    let mut end = col;

    // Find start of word - continue while same type
    if is_word {
        // Word character: alphanumeric or underscore
        while start > 0 && is_word_char(chars[start - 1]) {
            start -= 1;
        }
        // Find end of word
        while end + 1 < len && is_word_char(chars[end + 1]) {
            end += 1;
        }
    } else {
        // Non-blank, non-word character (punctuation)
        while start > 0 && !chars[start - 1].is_whitespace() && !is_word_char(chars[start - 1]) {
            start -= 1;
        }
        // Find end of word
        while end + 1 < len && !chars[end + 1].is_whitespace() && !is_word_char(chars[end + 1]) {
            end += 1;
        }
    }

    Some((start, end))
}

/// Returns true if the character is a word character (alphanumeric or underscore)
fn is_word_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_boundaries_forward() {
        let line = "Hello, world! This is a test.";
        assert_eq!(word_boundaries(line, 0), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 4), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 5), Some((5, 5))); // ","
        assert_eq!(word_boundaries(line, 6), None); // space
        assert_eq!(word_boundaries(line, 7), Some((7, 11))); // "world"
        assert_eq!(word_boundaries(line, 12), Some((12, 12))); // "!"
        assert_eq!(word_boundaries(line, 13), None); // space
        assert_eq!(word_boundaries(line, 14), Some((14, 17))); // "This"
        assert_eq!(word_boundaries(line, 24), Some((24, 27))); // "test"
        assert_eq!(word_boundaries(line, 28), Some((28, 28))); // "."
    }

    #[test]
    fn test_word_boundaries_backward() {
        let line = "Hello, world! This is a test.";
        assert_eq!(word_boundaries(line, 28), Some((28, 28))); // "."
        assert_eq!(word_boundaries(line, 27), Some((24, 27))); // "test"
        assert_eq!(word_boundaries(line, 23), None); // space
        assert_eq!(word_boundaries(line, 22), Some((22, 22))); // "a"
        assert_eq!(word_boundaries(line, 20), Some((19, 20))); // "is"
        assert_eq!(word_boundaries(line, 17), Some((14, 17))); // "This"
        assert_eq!(word_boundaries(line, 12), Some((12, 12))); // "!"
        assert_eq!(word_boundaries(line, 11), Some((7, 11))); // "world"
        assert_eq!(word_boundaries(line, 5), Some((5, 5))); // ","
        assert_eq!(word_boundaries(line, 4), Some((0, 4))); // "Hello"
        assert_eq!(word_boundaries(line, 0), Some((0, 4))); // "Hello"
    }

    #[test]
    fn test_single_letter_word() {
        let line = "A B C";
        assert_eq!(word_boundaries(line, 0), Some((0, 0))); // "A"
        assert_eq!(word_boundaries(line, 1), None); // space
        assert_eq!(word_boundaries(line, 2), Some((2, 2))); // "B"
        assert_eq!(word_boundaries(line, 3), None); // space
        assert_eq!(word_boundaries(line, 4), Some((4, 4))); // "C"
    }

    #[test]
    fn test_punctuation_words() {
        let line = "foo->bar = '*=*';";
        assert_eq!(word_boundaries(line, 0), Some((0, 2))); // "foo"
        assert_eq!(word_boundaries(line, 3), Some((3, 4))); // "->"
        assert_eq!(word_boundaries(line, 5), Some((5, 7))); // "bar"
        assert_eq!(word_boundaries(line, 8), None); // space
        assert_eq!(word_boundaries(line, 9), Some((9, 9))); // "="
        assert_eq!(word_boundaries(line, 10), None); // space
        assert_eq!(word_boundaries(line, 11), Some((11, 16))); // "'*=*'" - all punctuation grouped
        assert_eq!(word_boundaries(line, 17), None); // past end
    }

    #[test]
    fn test_underscore_in_word() {
        let line = "foo_bar baz_qux";
        assert_eq!(word_boundaries(line, 0), Some((0, 6))); // "foo_bar"
        assert_eq!(word_boundaries(line, 3), Some((0, 6))); // "foo_bar" (from underscore)
        assert_eq!(word_boundaries(line, 7), None); // space
        assert_eq!(word_boundaries(line, 8), Some((8, 14))); // "baz_qux"
    }
}

#[cfg(test)]
mod motion_tests {
    use super::*;
    use crate::domain::Buffer;

    #[test]
    fn test_motion_e() {
        let lines = vec![
            String::from("Hello, world! This is a test."),
            String::from("Another line here."),
            String::from(""),
            String::from("End of the buffer."),
            String::from("const CHAR = '*=*';"),
        ];

        let buffer = Buffer::from(lines);

        let start_pos = Position { row: 0, col: 1 };
        assert_eq!(buffer.get_char(&start_pos).unwrap(), 'e');

        let new_pos = e_motion(&buffer, start_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'o');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), ',');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'd');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '!');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 's');

        let new_pos = e_motion(&buffer, new_pos, 2);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'a');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 't');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '.');

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'r'); // another

        let new_pos = e_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '.'); // .

        let new_pos = e_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'd'); // End

        let new_pos = e_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'r'); // buffer

        let new_pos = e_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'R'); // CHAR

        let new_pos = e_motion(&buffer, new_pos, 2);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), ';'); // ;
    }

    #[test]
    fn test_motion_w() {
        let lines = vec![
            String::from("Hello, world! This is a test."),
            String::from("Another line here."),
            String::from(""),
            String::from("End of the buffer."),
            String::from("const CHAR = '*=*';"),
        ];

        let buffer = Buffer::from(lines);

        let start_pos = Position { row: 0, col: 1 };
        assert_eq!(buffer.get_char(&start_pos).unwrap(), 'e');

        let new_pos = w_motion(&buffer, start_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), ',');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'w');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '!');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'T');

        let new_pos = w_motion(&buffer, new_pos, 2);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'a');

        let new_pos = w_motion(&buffer, new_pos, 2);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '.');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'A'); // Another

        let new_pos = w_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '.'); // .

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert!(buffer.is_empty_line(&new_pos)); // Landed on the empty line

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'E'); // End

        let new_pos = w_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'b'); // buffer

        let new_pos = w_motion(&buffer, new_pos, 4);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '=');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '\'');

        let new_pos = w_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), ';');
    }

    #[test]
    fn test_motion_b() {
        let lines = vec![
            String::from("Hello, world! This is a test."),
            String::from("Another line here."),
            String::from(""),
            String::from("End of the buffer."),
            String::from("const CHAR = '*=*';"),
        ];

        let buffer = Buffer::from(lines);

        let start_pos = Position { row: 4, col: 16 }; // position at '*'
        assert_eq!(buffer.get_char(&start_pos).unwrap(), '*');

        let new_pos = b_motion(&buffer, start_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '\'');

        let new_pos = b_motion(&buffer, new_pos, 2);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'C'); // CHAR

        let new_pos = b_motion(&buffer, new_pos, 3);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 'b'); // buffer

        let new_pos = b_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 't');

        let new_pos = b_motion(&buffer, new_pos, 3);
        assert!(buffer.is_empty_line(&new_pos)); // Landed on empty line

        let new_pos = b_motion(&buffer, new_pos, 1);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), '.');

        let new_pos = b_motion(&buffer, new_pos, 5);
        assert_eq!(buffer.get_char(&new_pos).unwrap(), 't'); // test
    }
}

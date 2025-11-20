use crate::{
    core::{Buffer, Direction, Position},
    utils::word_boundaries,
};

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

#[cfg(test)]
mod motion_tests {
    use super::*;
    use crate::core::Buffer;

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

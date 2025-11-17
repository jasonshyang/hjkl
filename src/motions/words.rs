use crate::{
    types::{Buffer, Position},
    utils::word_boundaries,
};

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
        if !position.move_right(&buffer) {
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
    use crate::types::Buffer;

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

        let start_pos = Position { row: 0, col: 0 };
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
}

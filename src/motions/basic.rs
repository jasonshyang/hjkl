use crate::types::{Buffer, Direction, Position};

/// Moves the cursor left by `count` characters.
pub fn h_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.move_one_char(buffer, Direction::Backward) {
            break;
        }
    }
    position
}

/// Moves the cursor up by `count` lines.
pub fn j_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    // FIXME: in actual vim motion, previous position is memorized if next line is shorter,
    // and when resuming to a longer line, we get back to the same idx
    for _ in 0..count {
        if !position.move_one_line(buffer, Direction::Forward) {
            break;
        }
    }
    position
}

/// Moves the cursor down by `count` lines.
pub fn k_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.move_one_line(buffer, Direction::Backward) {
            break;
        }
    }
    position
}

/// Moves the cursor right by `count` characters.
pub fn l_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.move_one_char(buffer, Direction::Forward) {
            break;
        }
    }
    position
}

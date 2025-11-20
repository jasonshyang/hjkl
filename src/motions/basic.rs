use crate::core::{Buffer, Direction, Position};

/// Moves the cursor left by `count` characters.
pub fn h_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.step_char(buffer, Direction::Backward) {
            break;
        }
    }
    position
}

/// Moves the cursor up by `count` lines.
pub fn j_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.step_line(buffer, Direction::Forward) {
            break;
        }
    }
    position
}

/// Moves the cursor down by `count` lines.
pub fn k_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.step_line(buffer, Direction::Backward) {
            break;
        }
    }
    position
}

/// Moves the cursor right by `count` characters.
pub fn l_motion(buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.step_char(buffer, Direction::Forward) {
            break;
        }
    }
    position
}

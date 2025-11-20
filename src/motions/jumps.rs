use crate::core::{Buffer, Direction, Position};

pub fn f_motion(tar: char, buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.jump_to_char(buffer, tar, Direction::Forward) {
            break;
        }
    }
    position
}

pub fn big_f_motion(tar: char, buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.jump_to_char(buffer, tar, Direction::Backward) {
            break;
        }
    }
    position
}

pub fn t_motion(tar: char, buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.jump_before_char(buffer, tar, Direction::Forward) {
            break;
        }
    }
    position
}

pub fn big_t_motion(tar: char, buffer: &Buffer, mut position: Position, count: usize) -> Position {
    for _ in 0..count {
        if !position.jump_before_char(buffer, tar, Direction::Backward) {
            break;
        }
    }
    position
}

use crate::{
    domain::motions::{basic::*, jumps::*, words::*},
    domain::{Buffer, Position},
};

/// Represents different Vim Motions
#[derive(Clone, Copy)]
pub enum Motion {
    Left,  // h
    Down,  // j
    Up,    // k
    Right, // l

    WordStart,    // w - start of next word
    WordEnd,      // e - end of current/next word
    WordBackward, // b - start of previous word
    // WORDStart,    // W - start of next WORD
    // WORDEnd,      // E - end of current/next WORD
    // WORDBackward, // B - start of previous WORD
    FindNextChar(char), // f{char}
    FindPrevChar(char), // F{char}
    TillNextChar(char), // t{char}
    TillPrevChar(char), // T{char}

                        // LineStart, // 0
                        // LineEnd,   // $
}

impl Motion {
    pub fn is_find_till(&self) -> bool {
        matches!(
            self,
            Motion::FindNextChar(_)
                | Motion::FindPrevChar(_)
                | Motion::TillNextChar(_)
                | Motion::TillPrevChar(_)
        )
    }

    pub fn is_vertical(&self) -> bool {
        matches!(self, Motion::Up | Motion::Down)
    }

    pub fn needs_target(&self) -> bool {
        matches!(
            self,
            Motion::FindNextChar(_)
                | Motion::FindPrevChar(_)
                | Motion::TillNextChar(_)
                | Motion::TillPrevChar(_)
        )
    }

    pub fn reverse_find_till(&self) -> Option<Motion> {
        match self {
            Motion::FindNextChar(c) => Some(Motion::FindPrevChar(*c)),
            Motion::FindPrevChar(c) => Some(Motion::FindNextChar(*c)),
            Motion::TillNextChar(c) => Some(Motion::TillPrevChar(*c)),
            Motion::TillPrevChar(c) => Some(Motion::TillNextChar(*c)),
            _ => None,
        }
    }

    /// Applies the motion to the given buffer and position, returning the new position.
    pub fn apply(&self, buffer: &Buffer, position: Position, count: usize) -> Position {
        match self {
            Motion::Left => h_motion(buffer, position, count),
            Motion::Down => k_motion(buffer, position, count),
            Motion::Up => j_motion(buffer, position, count),
            Motion::Right => l_motion(buffer, position, count),
            Motion::WordStart => w_motion(buffer, position, count),
            Motion::WordEnd => e_motion(buffer, position, count),
            Motion::WordBackward => b_motion(buffer, position, count),
            Motion::FindNextChar(tar) => f_motion(*tar, buffer, position, count),
            Motion::FindPrevChar(tar) => big_f_motion(*tar, buffer, position, count),
            Motion::TillNextChar(tar) => t_motion(*tar, buffer, position, count),
            Motion::TillPrevChar(tar) => big_t_motion(*tar, buffer, position, count),
        }
    }
}

use crate::{
    motions::{
        basic::{h_motion, j_motion, k_motion, l_motion},
        words::{b_motion, e_motion, w_motion},
    },
    types::{Buffer, Position},
};

/// Represents different Vim Motions
pub enum Motion {
    Left,         // h
    Down,         // j
    Up,           // k
    Right,        // l
    WordStart,    // w - start of next word
    WordEnd,      // e - end of current/next word
    WordBackward, // b - start of previous word
    WORDStart,    // W - start of next WORD
    WORDEnd,      // E - end of current/next WORD
    WORDBackward, // B - start of previous WORD
}

impl Motion {
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
            _ => {
                unimplemented!("This motion is not yet implemented");
            }
        }
    }
}

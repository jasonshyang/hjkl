use crate::{
    motions::{b_motion, e_motion, h_motion, j_motion, k_motion, l_motion, w_motion},
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
    pub fn from_char(c: char) -> Option<Motion> {
        match c {
            'h' => Some(Motion::Left),
            'j' => Some(Motion::Down),
            'k' => Some(Motion::Up),
            'l' => Some(Motion::Right),
            'w' => Some(Motion::WordStart),
            'e' => Some(Motion::WordEnd),
            'b' => Some(Motion::WordBackward),
            'W' => Some(Motion::WORDStart),
            'E' => Some(Motion::WORDEnd),
            'B' => Some(Motion::WORDBackward),
            _ => None,
        }
    }

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

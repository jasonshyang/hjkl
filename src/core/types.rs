/// Direction for moving in the buffer
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Direction {
    Forward,
    Backward,
}

impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Direction::Forward => Direction::Backward,
            Direction::Backward => Direction::Forward,
        }
    }
}

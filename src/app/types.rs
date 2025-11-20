use crate::motions::Motion;

#[derive(Clone, Copy)]
pub enum Action {
    Motion((Motion, Option<usize>)),
    Noop,
    Pending,
    NewGame,
    Quit,
}

impl Action {
    pub fn single_motion(motion: Motion) -> Self {
        Action::Motion((motion, None))
    }

    pub fn repeated_motion(motion: Motion, count: usize) -> Self {
        Action::Motion((motion, Some(count)))
    }
}

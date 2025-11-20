use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{app::types::Action, motions::Motion, utils::BoundedQueue};

const EVENT_HISTORY_LEN: usize = 32;
const MOTION_HISTORY_LEN: usize = 8;

#[derive(Default)]
pub enum InputState {
    /// Waiting for initial input
    #[default]
    Idle,
    /// Accumulated a count
    Counting(usize),
    AwaitingTarget {
        motion: &'static str,
        count: Option<usize>,
    },
    AwaitingCombo {
        prefix: &'static str,
        count: Option<usize>,
    },
}

pub struct InputManager {
    state: InputState,
    event_history: BoundedQueue<KeyEvent>,
    motion_history: BoundedQueue<Motion>,
}

impl Default for InputManager {
    fn default() -> Self {
        Self {
            state: InputState::default(),
            event_history: BoundedQueue::new(EVENT_HISTORY_LEN),
            motion_history: BoundedQueue::new(MOTION_HISTORY_LEN),
        }
    }
}

impl InputManager {
    pub fn reset(&mut self) {
        self.state = InputState::default();
        self.event_history.clear();
        self.motion_history.clear();
    }

    pub fn keys_iter(&self) -> impl Iterator<Item = &KeyEvent> {
        self.event_history.reverse_iter()
    }

    pub fn handle(&mut self, key: KeyEvent) -> Action {
        let action = match &self.state {
            InputState::Idle => self.handle_idle(key),
            InputState::Counting(count) => self.handle_counting(*count, key),
            InputState::AwaitingTarget { motion, count } => self.handle_target(motion, *count, key),
            InputState::AwaitingCombo { prefix, count } => self.handle_combo(prefix, *count, key),
        };

        self.event_history.push(key);
        action
    }

    fn map_key_to_motion(key: KeyEvent) -> Option<Motion> {
        match (key.code, key.modifiers) {
            (KeyCode::Char('h'), KeyModifiers::NONE) => Some(Motion::Left),
            (KeyCode::Char('l'), KeyModifiers::NONE) => Some(Motion::Right),
            (KeyCode::Char('j'), KeyModifiers::NONE) => Some(Motion::Down),
            (KeyCode::Char('k'), KeyModifiers::NONE) => Some(Motion::Up),
            (KeyCode::Char('w'), KeyModifiers::NONE) => Some(Motion::WordStart),
            (KeyCode::Char('e'), KeyModifiers::NONE) => Some(Motion::WordEnd),
            (KeyCode::Char('b'), KeyModifiers::NONE) => Some(Motion::WordBackward),
            _ => None,
        }
    }

    fn handle_idle(&mut self, key: KeyEvent) -> Action {
        match (key.code, key.modifiers) {
            (KeyCode::Char(c @ '1'..='9'), KeyModifiers::NONE) => {
                let digit = c.to_digit(10).unwrap() as usize;
                self.state = InputState::Counting(digit);
                Action::Pending
            }
            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "f",
                    count: None,
                };
                Action::Pending
            }
            (KeyCode::Char('F'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "F",
                    count: None,
                };
                Action::Pending
            }
            (KeyCode::Char('t'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "t",
                    count: None,
                };
                Action::Pending
            }
            (KeyCode::Char('T'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "T",
                    count: None,
                };
                Action::Pending
            }

            (KeyCode::Char(';'), KeyModifiers::NONE) => {
                if let Some(last) = self.motion_history.last()
                    && last.is_find_till()
                {
                    Action::single_motion(*last)
                } else {
                    Action::Noop
                }
            }
            (KeyCode::Char(','), KeyModifiers::NONE) => {
                if let Some(last) = self.motion_history.last()
                    && let Some(reversed) = last.reverse_find_till()
                {
                    Action::single_motion(reversed)
                } else {
                    Action::Noop
                }
            }

            (KeyCode::Char(':'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingCombo {
                    prefix: ":",
                    count: None,
                };
                Action::Pending
            }

            _ => {
                if let Some(motion) = Self::map_key_to_motion(key) {
                    self.motion_history.push(motion);
                    Action::single_motion(motion)
                } else {
                    Action::Noop
                }
            }
        }
    }

    fn handle_counting(&mut self, current: usize, key: KeyEvent) -> Action {
        match (key.code, key.modifiers) {
            (KeyCode::Char(c @ '0'..='9'), KeyModifiers::NONE) => {
                let digit = c.to_digit(10).expect("Checked range") as usize;
                let new_count = current * 10 + digit;
                self.state = InputState::Counting(new_count);
                Action::Pending
            }
            (KeyCode::Char('f'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "f",
                    count: Some(current),
                };
                Action::Pending
            }
            (KeyCode::Char('F'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "F",
                    count: Some(current),
                };
                Action::Pending
            }
            (KeyCode::Char('t'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "t",
                    count: Some(current),
                };
                Action::Pending
            }
            (KeyCode::Char('T'), KeyModifiers::NONE) => {
                self.state = InputState::AwaitingTarget {
                    motion: "T",
                    count: Some(current),
                };
                Action::Pending
            }

            _ => {
                self.state = InputState::Idle;

                if let Some(motion) = Self::map_key_to_motion(key) {
                    self.motion_history.push(motion);
                    Action::repeated_motion(motion, current)
                } else {
                    Action::Noop
                }
            }
        }
    }

    fn handle_target(
        &mut self,
        motion: &'static str,
        count: Option<usize>,
        key: KeyEvent,
    ) -> Action {
        match key.code {
            KeyCode::Char(c) => {
                let motion = match motion {
                    "f" => Motion::FindNextChar(c),
                    "F" => Motion::FindPrevChar(c),
                    "t" => Motion::TillNextChar(c),
                    "T" => Motion::TillPrevChar(c),
                    _ => unreachable!("Motion not recognized"),
                };

                self.motion_history.push(motion);
                self.state = InputState::Idle;

                if let Some(cnt) = count {
                    Action::repeated_motion(motion, cnt)
                } else {
                    Action::single_motion(motion)
                }
            }
            _ => Action::Noop,
        }
    }

    fn handle_combo(
        &mut self,
        prefix: &'static str,
        _count: Option<usize>,
        key: KeyEvent,
    ) -> Action {
        match (prefix, key.code, key.modifiers) {
            (":", KeyCode::Char('q'), KeyModifiers::NONE) => {
                self.state = InputState::Idle;
                Action::Quit
            }
            (":", KeyCode::Char('n'), KeyModifiers::NONE) => {
                self.state = InputState::Idle;
                Action::NewGame
            }
            _ => {
                self.state = InputState::Idle;
                Action::Noop
            }
        }
    }
}

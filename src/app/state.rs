use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::generator::generate_random_rust_code_buffer;
use crate::motions::Motion;
use crate::types::{Buffer, Position};

/// Represents the state of the game, including the text buffer, cursor position, and quit status.
pub struct GameState {
    /// The text buffer being viewed.
    buffer: Buffer,
    /// The current cursor position in the buffer.
    cursor: Position,
    /// Indicates whether the game should quit.
    should_quit: bool,
    /// The last key event that was pressed,
    /// stored to handle multi-key commands.
    last_pressed: Option<KeyEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        let buffer = generate_random_rust_code_buffer();

        Self {
            buffer,
            cursor: Position::default(),
            should_quit: false,
            last_pressed: None,
        }
    }
}

impl GameState {
    /// Resets the game state to its initial values and
    /// generates a new random Rust code buffer.
    pub fn reset(&mut self) {
        self.buffer = generate_random_rust_code_buffer();
        self.cursor = Position::default();
        self.should_quit = false;
        self.last_pressed = None;
    }

    /// Returns a reference to the current text buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Returns the number of lines in the buffer.
    pub fn buffer_lines(&self) -> usize {
        self.buffer.rows()
    }

    /// Returns the current cursor position.
    pub fn cursor(&self) -> Position {
        self.cursor
    }

    pub fn last_pressed(&self) -> Option<String> {
        self.last_pressed.map(|k| k.code.to_string())
    }

    /// Returns whether the game should quit.
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Handles a key event, apply Vim motions and update state accordingly.
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        match (key.code, key.modifiers) {
            // Basic motions
            (KeyCode::Char('h'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::Left);
            }
            (KeyCode::Char('l'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::Right);
            }
            (KeyCode::Char('j'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::Down);
            }
            (KeyCode::Char('k'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::Up);
            }

            // Word motions
            (KeyCode::Char('w'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::WordStart);
            }
            (KeyCode::Char('e'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::WordEnd);
            }
            (KeyCode::Char('b'), KeyModifiers::NONE) => {
                self.apply_motion(Motion::WordBackward);
            }

            // Quit
            (KeyCode::Char('q'), KeyModifiers::NONE) => {
                if let Some(last_key) = self.last_pressed {
                    if last_key.code == KeyCode::Char(':')
                        && last_key.modifiers == KeyModifiers::NONE
                    {
                        self.should_quit = true;
                    }
                }
            }
            (KeyCode::Char('c'), KeyModifiers::CONTROL) => self.should_quit = true,

            // New Game
            (KeyCode::Char('n'), KeyModifiers::NONE) => {
                if let Some(last_key) = self.last_pressed {
                    if last_key.code == KeyCode::Char(':')
                        && last_key.modifiers == KeyModifiers::NONE
                    {
                        self.reset();
                    }
                }
            }
            _ => {}
        }

        self.last_pressed = Some(key);
    }

    fn apply_motion(&mut self, motion: Motion) {
        self.cursor = motion.apply(&self.buffer, self.cursor, 1);
    }
}

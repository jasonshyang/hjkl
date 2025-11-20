use std::{io, time::Duration};

use crossterm::event::Event;
use ratatui::Terminal;

use crate::{
    app::{Editor, input::InputManager, types::Action},
    ui::UiManager,
};

const INTERVAL: Duration = Duration::from_millis(10);

/// Main game structure managing state and UI.
#[derive(Default)]
pub struct Game {
    editor: Editor,
    input: InputManager,
    ui: UiManager,
}

impl Game {
    /// Starts a new round by resetting the game state and UI.
    pub fn new_round(&mut self) {
        self.editor.reset();
        self.input.reset();
        self.ui.reset();
    }

    /// Runs the game loop within the provided terminal.
    pub fn run_in<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui.render(f, &self.editor, self.input.keys_iter()))?;

            if crossterm::event::poll(INTERVAL)?
                && let Event::Key(key) = crossterm::event::read()?
            {
                let action = self.input.handle(key);

                match action {
                    Action::Motion((motion, count)) => self.editor.apply_motion(motion, count),
                    Action::Quit => break,
                    Action::NewGame => self.new_round(),
                    _ => { /* noop */ }
                }
            }
        }
        Ok(())
    }
}

use std::{io, time::Duration};

use crossterm::event::Event;
use ratatui::Terminal;

use crate::{app::GameState, ui::UiManager};

const INTERVAL: Duration = Duration::from_millis(50);

/// Main game structure managing state and UI.
#[derive(Default)]
pub struct Game {
    state: GameState,
    ui: UiManager,
}

impl Game {
    /// Starts a new round by resetting the game state and UI.
    pub fn new_round(&mut self) {
        self.state.reset();
        self.ui.reset();
    }

    /// Runs the game loop within the provided terminal.
    pub fn run_in<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            terminal.draw(|f| self.ui.render(f, &self.state))?;

            if crossterm::event::poll(INTERVAL)?
                && let Event::Key(key) = crossterm::event::read()?
            {
                self.state.handle_key_event(key);
            }

            if self.state.should_quit() {
                break;
            }
        }
        Ok(())
    }
}

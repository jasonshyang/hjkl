use std::{io, time::Duration};

use crossterm::event::Event;
use ratatui::Terminal;

use crate::{
    app::input::{InputManager, UserAction},
    domain::{EnemyConfig, GameConfig, World},
    tui::{FileSelectAction, UiAction, UiManager},
};

const INTERVAL: Duration = Duration::from_millis(10);

/// Represents the current mode of the game.
#[derive(Default)]
enum GameMode {
    /// Main menu selection
    #[default]
    Menu,
    /// File selection screen
    FileSelect,
    /// Main game
    Game,
}

/// Main game structure orchestrating state, input, and UI.
#[derive(Default)]
pub struct Game {
    mode: GameMode,
    world: World,
    input: InputManager,
    ui: UiManager,
}

impl Game {
    /// Starts a new round by resetting the game state and UI.
    pub fn new_round(&mut self, file_path: Option<String>) {
        let game_config = GameConfig {
            enemy: EnemyConfig::default(),
            file_path,
        };
        self.world = World::new(game_config);
        self.input.reset();
        self.ui.reset();
    }

    /// Runs the game loop within the provided terminal.
    pub fn run_in<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
    ) -> io::Result<()> {
        loop {
            match self.mode {
                // Handle menu
                GameMode::Menu => {
                    terminal.draw(|f| self.ui.render_menu(f))?;

                    if crossterm::event::poll(INTERVAL)?
                        && let Event::Key(key) = crossterm::event::read()?
                    {
                        let action = self.ui.handle_menu_key(key);
                        match action {
                            UiAction::StartGame => self.mode = GameMode::FileSelect,
                            UiAction::Quit => break,
                            UiAction::Noop => {}
                        }
                    }
                }
                // Handle file selection
                GameMode::FileSelect => {
                    terminal.draw(|f| self.ui.render_file_select(f))?;

                    if crossterm::event::poll(INTERVAL)?
                        && let Event::Key(key) = crossterm::event::read()?
                    {
                        let action = self.ui.handle_file_select_key(key);
                        match action {
                            FileSelectAction::Confirm(path) => {
                                self.mode = GameMode::Game;
                                self.new_round(Some(path));
                            }
                            FileSelectAction::UseRandom => {
                                self.mode = GameMode::Game;
                                self.new_round(None);
                            }
                            FileSelectAction::Cancel => self.mode = GameMode::Menu,
                            FileSelectAction::Noop => {}
                        }
                    }
                }
                // Main game loop
                GameMode::Game => {
                    self.world.tick();

                    // Pull game events from world to be used by other components
                    let events = self.world.pull_events();

                    // Render the game UI
                    terminal.draw(|f| {
                        self.ui
                            .render_game(f, &self.world, &events, self.input.keys_iter())
                    })?;

                    if crossterm::event::poll(INTERVAL)?
                        && let Event::Key(key) = crossterm::event::read()?
                    {
                        let action = self.input.handle_key(key);
                        match action {
                            UserAction::Motion((motion, count)) => {
                                self.world.apply_motion(motion, count);
                            }
                            UserAction::Quit => self.mode = GameMode::Menu,
                            UserAction::NewGame => self.mode = GameMode::FileSelect,
                            _ => {}
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

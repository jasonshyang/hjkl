use crate::{
    domain::{GameEvent, World},
    tui::{
        Effect, Effects, FileSelectAction, FileSelector,
        menu::{Menu, MenuAction},
        renderer,
        theme::STATUS_BAR_HEIGHT,
        viewport::Viewport,
    },
};
use crossterm::event::KeyEvent;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

/// Actions that can be taken in the UI.
pub enum UiAction {
    StartGame,
    Noop,
    Quit,
}

/// UI Manager handling rendering of different UI components.
pub struct UiManager {
    menu: Menu,
    viewport: Viewport,
    effects: Effects,
    file_selector: FileSelector,
}

impl Default for UiManager {
    fn default() -> Self {
        Self {
            menu: Menu::default(),
            viewport: Viewport::default(),
            effects: Effects::default(),
            file_selector: FileSelector::new("src/main.rs"),
        }
    }
}

impl UiManager {
    pub fn reset(&mut self) {
        self.menu = Menu::default();
        self.viewport = Viewport::default();
        self.effects = Effects::default();
        self.file_selector.reset("src/main.rs");
    }

    pub fn handle_menu_key(&mut self, key: KeyEvent) -> UiAction {
        match self.menu.handle_key(key) {
            MenuAction::Start => UiAction::StartGame,
            MenuAction::Quit => UiAction::Quit,
            MenuAction::Noop => UiAction::Noop,
        }
    }

    pub fn handle_file_select_key(&mut self, key: KeyEvent) -> FileSelectAction {
        self.file_selector.handle_key(key)
    }

    pub fn render_menu(&self, f: &mut Frame) {
        renderer::render_menu(f, &self.menu);
    }

    pub fn render_file_select(&self, f: &mut Frame) {
        renderer::render_file_select(f, self.file_selector.input(), self.file_selector.error());
    }

    pub fn render_game<'a>(
        &mut self,
        f: &mut Frame,
        game: &World,
        events: &[GameEvent],
        keys_iter: impl Iterator<Item = &'a KeyEvent>,
    ) {
        // Handle world events
        self.handle_events(events);

        let chunks = Layout::default()
            .constraints([Constraint::Min(0), Constraint::Length(STATUS_BAR_HEIGHT)])
            .split(f.area());

        // Update viewport based on available height (subtract 2 for borders)
        let visible_height = chunks[0].height.saturating_sub(2) as usize;

        // Updates the viewport based on cursor position and visible area height.
        self.viewport
            .adjust_for_cursor(game.cursor().pos(), game.buffer_lines(), visible_height);

        renderer::render_world(f, game, &self.effects, &self.viewport, chunks[0]);
        renderer::render_status_bar(f, game, keys_iter, chunks[1]);

        // Cleanup expired effects
        self.effects.cleanup();
    }

    fn handle_events(&mut self, events: &[GameEvent]) {
        // Process events and spawn visual effects
        for event in events {
            match event {
                GameEvent::EnemyDestroyed { position } => {
                    self.effects.spawn_effect(Effect::collision(*position));
                }
                GameEvent::CursorMoved {
                    position,
                    timestamp,
                } => {
                    self.effects
                        .spawn_effect(Effect::trailing(*position, *timestamp));
                }
            }
        }
    }
}

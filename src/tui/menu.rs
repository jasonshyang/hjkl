use std::fmt::Display;

use crossterm::event::{KeyCode, KeyEvent};

/// Menu options available in the main menu
#[derive(Clone, Copy)]
pub enum MenuOption {
    Start,
    Quit,
}

impl Display for MenuOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            MenuOption::Start => "Start",
            MenuOption::Quit => "Quit",
        };
        write!(f, "{}", text)
    }
}

/// Actions as a result of menu input.
pub enum MenuAction {
    Start,
    Quit,
    Noop,
}

/// Main menu structure managing options and selection
pub struct Menu {
    selected: usize,
    options: Vec<MenuOption>,
}

impl Default for Menu {
    fn default() -> Self {
        Self {
            selected: 0,
            options: vec![MenuOption::Start, MenuOption::Quit],
        }
    }
}

impl Menu {
    pub fn options(&self) -> &Vec<MenuOption> {
        &self.options
    }

    pub fn selected_idx(&self) -> usize {
        self.selected
    }

    fn navigate_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn navigate_down(&mut self) {
        if self.selected < self.options.len() - 1 {
            self.selected += 1;
        }
    }

    fn selected_option(&self) -> MenuOption {
        self.options[self.selected]
    }

    /// Handles a key event in the menu.
    pub fn handle_key(&mut self, key: KeyEvent) -> MenuAction {
        match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                self.navigate_down();
                MenuAction::Noop
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.navigate_up();
                MenuAction::Noop
            }
            KeyCode::Enter => match self.selected_option() {
                MenuOption::Start => MenuAction::Start,
                MenuOption::Quit => MenuAction::Quit,
            },
            _ => MenuAction::Noop,
        }
    }
}

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Actions as a result of file selection input.
pub enum FileSelectAction {
    Confirm(String),
    UseRandom,
    Cancel,
    Noop,
}

/// Manages the file selection UI and input handling.
pub struct FileSelector {
    input: String,
    error: Option<String>,
}

impl FileSelector {
    pub fn new(default_path: &str) -> Self {
        Self {
            input: default_path.to_string(),
            error: None,
        }
    }

    pub fn input(&self) -> &str {
        &self.input
    }

    pub fn error(&self) -> &Option<String> {
        &self.error
    }

    /// Handles a key event in the file selector.
    pub fn handle_key(&mut self, key: KeyEvent) -> FileSelectAction {
        match (key.code, key.modifiers) {
            // Generate random code
            (KeyCode::Char('r'), KeyModifiers::CONTROL) => FileSelectAction::UseRandom,
            // Confirm the entered file path
            (KeyCode::Enter, _) => {
                if self.input.is_empty() {
                    self.error = Some("Please enter a file path".to_string());
                    FileSelectAction::Noop
                } else if !self.input.ends_with(".rs") {
                    self.error = Some("File must be a .rs file".to_string());
                    FileSelectAction::Noop
                } else {
                    FileSelectAction::Confirm(self.input.clone())
                }
            }
            // Cancel file selection
            (KeyCode::Esc, _) => FileSelectAction::Cancel,
            // Handle backspace
            (KeyCode::Backspace, _) => {
                self.input.pop();
                self.error = None;
                FileSelectAction::Noop
            }
            // Handle character input
            (KeyCode::Char(c), _) => {
                self.input.push(c);
                self.error = None;
                FileSelectAction::Noop
            }
            _ => FileSelectAction::Noop,
        }
    }

    pub fn reset(&mut self, default_path: &str) {
        self.input = default_path.to_string();
        self.error = None;
    }
}

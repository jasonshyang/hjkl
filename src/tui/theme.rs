use std::time::Duration;

use ratatui::style::Color;

pub const GAME_TITLE: &str = "👾 HJKL: Code Invaders 👾";
pub const MENU_TITLE: &str = "Menu";
pub const FILE_SELECTION_TITLE: &str = "Select Rust File";
pub const FILE_SELECTION_INSTRUCTION: &str =
    "Enter path to .rs file | Ctrl+R for random | ESC to go back";
pub const STATUS_INSTRUCTIONS: &str = "Press ':q' to quit, ':n' for new round";

pub const PLAYER_CHAR: &str = "▓";
pub const ENEMY_CHAR: &str = "👾";

pub const STATUS_BAR_HEIGHT: u16 = 3;
pub const VIEWPORT_PADDING: usize = 3;
pub const BORDER_LENGTH: u16 = 2; // 1 for top border + 1 for bottom border

pub const STATUS_BG_COLOR: Color = Color::DarkGray;
pub const STATUS_FG_COLOR: Color = Color::White;
pub const MENU_TITLE_COLOR: Color = Color::Cyan;
pub const MENU_SELECTED_COLOR: Color = Color::Cyan;
pub const MENU_LINE_COLOR: Color = Color::White;
pub const FILE_SELECTION_INPUT_COLOR: Color = Color::Cyan;

pub const MENU_SIZE: (u16, u16) = (60, 12); // width, height
pub const FILE_SELECTION_SIZE: (u16, u16) = (70, 12); // width, height

pub const COLLISION_EFFECT_DURATION: Duration = Duration::from_millis(200);
pub const TRAILING_EFFECT_DURATION: Duration = Duration::from_millis(200);

pub const SYNTAX_KEYWORD_COLOR: Color = Color::Rgb(242, 195, 92);
pub const SYNTAX_TYPE_COLOR: Color = Color::Rgb(166, 123, 64);
pub const SYNTAX_STRING_COLOR: Color = Color::Rgb(136, 171, 152);
pub const SYNTAX_NUMBER_COLOR: Color = Color::Rgb(242, 195, 92);
pub const SYNTAX_COMMENT_COLOR: Color = Color::Rgb(103, 128, 121);
pub const SYNTAX_PUNCTUATION_COLOR: Color = Color::Rgb(154, 155, 158);
pub const SYNTAX_NORMAL_COLOR: Color = Color::White;

use ratatui::style::Color;

pub const TITLE: &str = "HJKL Vim Motions Practice";
pub const STATUS_INSTRUCTIONS: &str = "Press ':q' to quit, ':n' for new round";

pub const STATUS_BAR_HEIGHT: u16 = 3;
pub const VIEWPORT_PADDING: usize = 3;
pub const BORDER_LENGTH: u16 = 2; // 1 for top border + 1 for bottom border

pub const CURSOR_BG_COLOR: Color = Color::White;
pub const CURSOR_FG_COLOR: Color = Color::Black;
pub const STATUS_BG_COLOR: Color = Color::DarkGray;
pub const STATUS_FG_COLOR: Color = Color::White;

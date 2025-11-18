use crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use hjkl::app::Game;
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;

fn main() -> io::Result<()> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the game
    let mut game = Game::default();
    let res = game.run_in(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    // Handle any errors from the game run
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }

    Ok(())
}

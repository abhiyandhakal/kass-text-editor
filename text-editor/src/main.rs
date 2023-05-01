use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Result};

mod enums;
mod filetree;
mod kass;
mod statusbar;
mod window;

fn main() -> Result<()> {
    // move cursor to 0,0
    // enter alternate screen

    execute!(
        stdout(),
        cursor::SavePosition,
        terminal::EnterAlternateScreen,
        cursor::MoveTo(0, 0)
    )?;
    // enable raw mode
    terminal::enable_raw_mode()?;

    // text editor
    let mut kass = kass::Kass::new()?;
    kass.run()?;

    // disable raw mode
    terminal::disable_raw_mode()?;
    // leave alternate screen
    execute!(
        stdout(),
        cursor::RestorePosition,
        terminal::LeaveAlternateScreen
    )?;

    Ok(())
}

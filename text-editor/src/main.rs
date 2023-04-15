use crossterm::{cursor, execute, terminal};
use std::{
    env::args,
    io::{stdout, Result},
};

mod kass;
mod mode;
mod statusbar;

fn main() -> Result<()> {
    // move cursor to 0,0
    // enter alternate screen
    execute!(
        stdout(),
        cursor::MoveTo(0, 0),
        terminal::EnterAlternateScreen
    )?;
    // enable raw mode
    terminal::enable_raw_mode()?;

    // get terminal size
    let mut height: usize = 0;
    let mut width: usize = 0;
    if let Some((w, h)) = term_size::dimensions() {
        height = h as usize;
        width = w as usize
    } else {
        print!("Unable to get term size :(\n")
    }

    // get file path
    let args: Vec<String> = args().collect();
    let mut filepath: &String = &String::from("n/a");

    if args.len() > 1 {
        filepath = &args[1];
    }

    // text editor
    let mut kass = kass::Kass::new(height, width, filepath)?;
    kass.run()?;

    // disable raw mode
    terminal::disable_raw_mode()?;
    // leave alternate screen
    execute!(stdout(), terminal::LeaveAlternateScreen)?;

    Ok(())
}

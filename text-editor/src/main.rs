use crossterm::{execute, terminal, Result};
use std::io::{stdin, stdout, Read};

fn main() -> Result<()> {
    let mut stdout = stdout();

    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    let mut colon_pressed = false;

    for i in stdin().bytes() {
        let character = i.unwrap() as char;
        print!("{}", character);

        // colon
        if character == ':' {
            colon_pressed = true;
        }

        if colon_pressed {
            if character == 'q' {
                break;
            }
        }
    }

    Ok(())
}

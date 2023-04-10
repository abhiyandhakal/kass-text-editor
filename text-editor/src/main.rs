use crossterm::{cursor, execute, terminal};
use std::{
    io::{stdin, stdout, Read, Result, Write},
    process::exit,
};

// modes
#[derive(Copy, Clone)]
enum Mode {
    Insert,
    Normal,
    Visual,
    Command,
}

fn main() -> Result<()> {
    let mut stdin = stdin();
    let mut stdout = stdout();

    // clear terminal
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorUp))?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // modes
    let mut mode = Mode::Normal;
    let mut mode_changed: bool = false;

    // enable raw mode
    terminal::enable_raw_mode().expect("could not enable raw mode");

    let mut buf = [0; 1];

    while stdin.read(&mut buf)? == 1 {
        let character = buf[0] as char;

        (mode, mode_changed) = handle_modes(&mode, character as u8);

        if !mode_changed {
            match mode {
                Mode::Insert => handle_insert_mode(character, &mut mode_changed),
                Mode::Command => handle_command_mode(character, &mut mode_changed),
                Mode::Normal => {}
                _ => {
                    println!("\ridk what is happening")
                }
            }
        }
    }

    terminal::disable_raw_mode().expect("could not disable raw mode");

    Ok(())
}

// handle modes
fn handle_modes(current_mode: &Mode, pressed_key_code: u8) -> (Mode, bool) {
    let mut new_mode: Mode = *current_mode;
    let mut mode_changed: bool = true;

    match *current_mode {
        Mode::Normal => {
            match pressed_key_code {
                // insert mode
                105 => new_mode = Mode::Insert, // i
                97 => new_mode = Mode::Insert,  // a

                // visual mode
                118 => new_mode = Mode::Visual, // v

                // command mode
                58 => new_mode = Mode::Command, // :

                _ => {
                    mode_changed = false;
                }
            }
        }
        _ => {
            // go back to normal mode
            match pressed_key_code {
                27 => new_mode = Mode::Normal, // Esc
                _ => {
                    mode_changed = false;
                }
            }
        }
    }

    (new_mode, mode_changed)
}

// insert mode task
fn handle_insert_mode(character: char, mode_changed: &mut bool) {
    if !character.is_control() {
        print!("{}", character);
        stdout().flush().expect("could not flush");
    }

    *mode_changed = false;
}

fn handle_command_mode(character: char, mode_changed: &mut bool) {
    match character {
        'q' => {
            terminal::disable_raw_mode().expect("could not disable raw mode");
            exit(0)
        }
        _ => {}
    }

    *mode_changed = false;
}

use crossterm::terminal;
use std::{
    io::{stdin, stdout, Read, Result, Write},
    mem::discriminant,
};

struct RawMode;

impl RawMode {
    fn enable(&self) {
        terminal::enable_raw_mode().expect("could not enable raw mode");
    }
    fn disable(&self) {
        terminal::disable_raw_mode().expect("could not disable raw mode");
    }
}

// modes
enum Mode {
    Insert,
    Normal,
    Visual,
    Command,
}

fn main() -> Result<()> {
    let _raw_mode = RawMode;
    let mut stdout = stdout();
    let mut stdin = stdin();

    // modes
    let mut mode = Mode::Insert;

    // enable raw mode
    _raw_mode.enable();

    let mut buf = [0; 1];

    while stdin.read(&mut buf)? == 1 {
        let character = buf[0] as char;

        mode = handle_modes(&mode, character as u8);

        if character == 'q' {
            break;
        }

        if character.is_control() {
            print!("{}", character as u8);
            stdout.flush()?;
        } else {
            print!("{}", character);
            stdout.flush()?;
        }
    }

    // disable raw mode
    _raw_mode.disable();

    Ok(())
}

// handle modes
fn handle_modes(current_mode: &Mode, pressed_key_code: u8) -> Mode {
    let mut new_mode: Mode = *current_mode;

    if discriminant(current_mode) == discriminant(&Mode::Normal) {
    } else {
    }

    match current_mode {
        &Mode::Normal => {
            println!("\r{pressed_key_code}");

            match pressed_key_code {
                105 => new_mode = Mode::Insert,
                _ => new_mode = Mode::Normal,
            }
        }
        _ => println!("\r wtf are u doing here??"),
    }

    // if discriminant(&new_mode) == discriminant(&Mode::Normal) {
    //     println!("\rnormal");
    // } else if discriminant(&new_mode) == discriminant(&Mode::Insert) {
    //     println!("\rInsert");
    // } else if discriminant(&new_mode) == discriminant(&Mode::Visual) {
    //     println!("\rvisual");
    // }

    new_mode
}

use crossterm::{cursor, execute, terminal};
use std::io::{stdin, stdout, Read, Result, Write};

// modes
#[derive(Copy, Clone, Debug)]
enum Mode {
    Insert,
    Normal,
    Visual,
    Command,
}

#[derive(Debug)]
struct Kass {
    current_mode: Mode,
    mode_changed: bool,

    buf: [u8; 1],
    quit_kass: bool,
}

impl Kass {
    // constructor
    fn new() -> Kass {
        // clear terminal
        execute!(stdout(), terminal::Clear(terminal::ClearType::FromCursorUp))
            .expect("could not clear terminal");
        // move cursor to 0,0
        execute!(stdout(), cursor::MoveTo(0, 0)).expect("could not move cursor to (0,0)");
        // enable raw mode
        terminal::enable_raw_mode().expect("could not enable raw mode");

        Kass {
            current_mode: Mode::Normal,
            mode_changed: false,
            buf: [0],
            quit_kass: false,
        }
    }

    fn run(&mut self) {
        while stdin().read(&mut self.buf).expect("could not read") == 1 {
            self.handle_modes();

            if !self.mode_changed {
                match self.current_mode {
                    Mode::Insert => self.handle_insert_mode(),
                    Mode::Command => self.handle_command_mode(),
                    Mode::Normal => {}
                    _ => println!("\ridk what is happening"),
                }
            }

            if self.quit_kass {
                break;
            }
        }
    }

    fn handle_modes(&mut self) {
        match self.current_mode {
            Mode::Normal => match self.buf[0] {
                105 => self.current_mode = Mode::Insert, // i
                97 => self.current_mode = Mode::Insert,  // a

                // visual mode
                118 => self.current_mode = Mode::Visual, // v

                // command mode
                58 => self.current_mode = Mode::Command, // :

                _ => self.mode_changed = false,
            },
            _ => match self.buf[0] {
                27 => self.current_mode = Mode::Normal, // Esc
                _ => self.mode_changed = false,
            },
        }
    }

    fn handle_insert_mode(&mut self) {
        let character = self.buf[0] as char;

        if !character.is_control() {
            print!("{}", character);
            stdout().flush().expect("could not flush")
        }

        self.mode_changed = false;
    }

    fn handle_command_mode(&mut self) {
        match self.buf[0] {
            113 => {
                terminal::disable_raw_mode().expect("could not disable raw mode");
                self.quit_kass = true;
            }
            _ => {}
        }

        self.mode_changed = false;
    }
}

fn main() -> Result<()> {
    let mut kass = Kass::new();
    kass.run();

    terminal::disable_raw_mode()?;

    Ok(())
}

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal,
};
use std::io::{stdout, Result, Write};

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

    key_event: KeyEvent,

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
        // crossterm flags

        Kass {
            current_mode: Mode::Normal,
            mode_changed: false,
            key_event: KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            quit_kass: false,
        }
    }

    fn run(&mut self) -> Result<()> {
        loop {
            if let Event::Key(event) = event::read()? {
                self.key_event = event;

                self.handle_modes()?;

                if !self.mode_changed {
                    match self.current_mode {
                        Mode::Insert => self.handle_insert_mode()?,
                        Mode::Command => self.handle_command_mode()?,
                        Mode::Normal => println!("\rnormal mode"),
                        Mode::Visual => println!("\rvisual mode"),
                    }
                }

                // quit kass
                if self.quit_kass {
                    break;
                }
            }
        }

        Ok(())
    }

    fn handle_modes(&mut self) -> Result<()> {
        match self.current_mode {
            Mode::Normal => match self.key_event {
                // insert mode
                KeyEvent {
                    code: KeyCode::Char('i'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    self.current_mode = Mode::Insert;
                    self.mode_changed = true;
                }
                KeyEvent {
                    code: KeyCode::Char('a'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => {
                    self.current_mode = Mode::Insert;
                    self.mode_changed = true;
                }

                // visual mode
                KeyEvent {
                    code: KeyCode::Char('v'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => self.current_mode = Mode::Visual,

                // command mode
                KeyEvent {
                    code: KeyCode::Char(':'),
                    modifiers: KeyModifiers::NONE,
                    ..
                } => self.current_mode = Mode::Command,

                _ => {
                    self.mode_changed = false;
                    println!("no key satisfies");
                }
            },
            _ => match self.key_event {
                KeyEvent {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                    ..
                } => self.current_mode = Mode::Normal,

                _ => self.mode_changed = false,
            },
        }
        Ok(())
    }

    fn handle_insert_mode(&mut self) -> Result<()> {
        print!("{:?}", self.key_event.code);
        stdout().flush()?;

        self.mode_changed = false;

        Ok(())
    }

    fn handle_command_mode(&mut self) -> Result<()> {
        match self.key_event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                terminal::disable_raw_mode().expect("could not disable raw mode");
                self.quit_kass = true;
            }
            _ => {}
        }

        self.mode_changed = false;

        Ok(())
    }
}

fn main() -> Result<()> {
    let mut kass = Kass::new();
    kass.run()?;

    terminal::disable_raw_mode()?;

    Ok(())
}

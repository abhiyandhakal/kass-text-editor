use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal,
};
use std::{
    io::{stdout, Result, Write},
    string,
};

use super::mode::*;
use super::statusbar::*;

#[derive(Debug, Clone)]
pub struct Kass {
    current_mode: Mode,
    mode_changed: bool,

    key_event: KeyEvent,
    character: char,

    quit_kass: bool,

    text: String,

    filepath: String,

    terminal_width: usize,
    terminal_height: usize,
}

impl Kass {
    // constructor
    pub fn new(height: usize, width: usize, filepath: &String) -> Result<Kass> {
        Ok(Kass {
            current_mode: Mode::Normal,
            mode_changed: false,
            key_event: KeyEvent {
                code: KeyCode::Char('f'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            character: 'f',
            text: String::new(),
            quit_kass: false,
            filepath: String::from(filepath),
            terminal_height: height,
            terminal_width: width,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        loop {
            if let Event::Key(event) = event::read()? {
                // set key_event
                self.key_event = event;

                // set character
                match event.code {
                    KeyCode::Char(c) => self.character = c,
                    _ => {}
                }

                self.handle_modes();

                if !self.mode_changed {
                    match self.current_mode {
                        Mode::Insert => self.handle_insert_mode()?,
                        Mode::Command => self.handle_command_mode()?,
                        // Mode::Normal => println!("\rnormal mode"),
                        // Mode::Visual => println!("\rvisual mode"),
                        _ => {}
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

    fn handle_modes(&mut self) {
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
    }

    fn handle_insert_mode(&mut self) -> Result<()> {
        match self.key_event {
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.text.pop();
                self.refresh_screen()?;
            }
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => {
                self.text.push('\n');
                execute!(stdout(), cursor::MoveToNextLine(1))?;
            }
            _ => {
                // print
                if !self.character.is_control() {
                    self.text.push(self.character);

                    let output = write!(stdout(), "{}", self.character);
                    stdout().flush()?;
                    drop(output);
                }
            }
        }

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
                terminal::disable_raw_mode()?;
                self.quit_kass = true;
            }
            KeyEvent {
                code: KeyCode::Char('w'),
                modifiers: KeyModifiers::NONE,
                ..
            } => {}
            _ => {}
        }

        self.mode_changed = false;

        Ok(())
    }

    fn refresh_screen(&self) -> Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::All),
        )?;

        write!(stdout(), "{}", self.text)?;
        stdout().flush()?;

        Ok(())
    }
}

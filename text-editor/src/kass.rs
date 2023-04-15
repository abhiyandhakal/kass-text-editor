use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers},
    execute, terminal,
};
use std::io::{stdout, Result, Write};

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
    command: String,

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
            command: String::from(""),
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
                }
            },

            Mode::Command => match self.key_event {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => {
                    self.command = String::from("");
                    self.refresh_screen(0, 0, &self.text)?;
                    self.current_mode = Mode::Normal;
                }
                _ => self.mode_changed = false,
            },

            _ => match self.key_event {
                KeyEvent {
                    code: KeyCode::Esc, ..
                } => self.current_mode = Mode::Normal,
                _ => self.mode_changed = false,
            },
        }

        Ok(())
    }

    fn handle_insert_mode(&mut self) -> Result<()> {
        match self.key_event {
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.text.pop();
                self.refresh_screen(0, 0, &self.text)?;
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
        let position_x = 0;
        let position_y = self.terminal_height - 1;

        execute!(
            stdout(),
            cursor::MoveTo(position_x as u16, position_y as u16)
        )?;

        match self.key_event {
            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.command.pop();
                self.refresh_screen(position_x, position_y, &self.command)?;
            }

            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => match self.command.as_str() {
                ":q" => self.quit_kass = true,

                _ => {
                    self.command = String::from("");
                    self.refresh_screen(0, 0, &self.text)?;
                    self.current_mode = Mode::Normal;
                }
            },

            _ => {
                if !self.character.is_control() {
                    self.command.push(self.character);

                    write!(stdout(), "{}", self.command)?;
                    stdout().flush()?;
                }
            }
        }

        self.mode_changed = false;

        Ok(())
    }

    fn refresh_screen(&self, width: usize, height: usize, text: &String) -> Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(width as u16, height as u16),
            terminal::Clear(terminal::ClearType::All),
        )?;

        write!(stdout(), "{}", text)?;
        stdout().flush()?;

        Ok(())
    }
}

use std::{format, io::Result, path::Path, vec};

use crossterm::event::{self, Event, KeyEvent, KeyEventState, KeyModifiers};
use serde_json::Value;
use tui::{backend::Backend, Terminal};

use crate::{
    editor::Editor,
    enums::*,
    mode_handlers::{
        command::handle_command_mode, insert::handle_insert_mode, normal::handle_normal_mode,
    },
    position::Position,
    ui::ui,
};

pub struct App {
    pub mode: Mode,
    pub tabs: Vec<Editor>,

    pub command: String,
    pub error: String,
    pub info: String,

    pub action: Action,
    pub clipboard: Vec<String>,
    pub active_index: usize,
}

impl App {
    fn new() -> Result<App> {
        let mut filepath = "unnamed".to_string();
        let mut counter = 0;

        while Path::new(&filepath).exists() {
            counter += 1;
            filepath = format!("{}-{}", filepath, counter);
        }

        Ok(App {
            mode: Mode::Normal,
            command: String::new(),
            tabs: vec![Editor::new(filepath.clone())?],
            clipboard: vec![],
            active_index: 0,
            error: String::new(),
            info: String::new(),
            action: Action::Command,
        })
    }
    pub fn next(&mut self) {
        self.active_index = (self.active_index + 1) % self.tabs.len();
    }

    pub fn previous(&mut self) {
        if self.active_index > 0 {
            self.active_index -= 1;
        } else {
            self.active_index = self.tabs.len() - 1;
        }
    }
}

pub struct Kass {
    pub app: App,
    pub key_event: KeyEvent,
    pub cursor: Position,
    pub editor_size: (u16, u16),
    pub buf: String,

    // settings
    pub line_number: LineNumber,
}

impl Kass {
    pub fn new() -> Result<Kass> {
        let app = App::new()?;
        Ok(Kass {
            app,
            key_event: KeyEvent {
                code: crossterm::event::KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: crossterm::event::KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
            cursor: Position::new(),
            editor_size: (0, 0),
            buf: String::new(),

            line_number: LineNumber::None,
        })
    }

    pub fn run<B: Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        config: Option<Value>,
    ) -> Result<()> {
        if let Some(config) = config {
            let mut close = false;

            if let Value::Object(settings) = &config {
                for (key, value) in settings.iter() {
                    match key.as_str() {
                        "line_number" => match value.as_str() {
                            Some(value) => match value {
                                "none" => self.line_number = LineNumber::None,
                                "absolute" => self.line_number = LineNumber::Absolute,
                                "relative" => self.line_number = LineNumber::Relative,
                                _ => {
                                    self.set_error("Provide a valid value for line number");
                                }
                            },
                            None => {
                                self.set_error("Provide a value for line number");
                            }
                        },
                        "command_mode" => {}
                        key => {
                            self.set_error(format!("{} in the config doesn't exist", key).as_str());
                        }
                    }
                }
            }

            loop {
                terminal.draw(|f| ui(self, f))?;

                if let Event::Key(key) = event::read()? {
                    self.key_event = key;

                    match self.app.mode {
                        Mode::Normal => handle_normal_mode(self)?,
                        Mode::Command => handle_command_mode(self, &mut close, &config)?,
                        Mode::Insert => handle_insert_mode(self)?,
                    }
                }

                if close {
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn set_info(&mut self, info: &str) {
        self.app.action = Action::Info;
        self.app.info = info.to_string();
    }

    pub fn set_error(&mut self, error: &str) {
        self.app.action = Action::Error;
        self.app.error = error.to_string();
    }
}

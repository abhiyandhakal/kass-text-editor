use crossterm::event::{KeyEvent, KeyEventState, KeyModifiers};

use crate::enums::Mode;

#[derive(Debug, Clone)]
pub struct Editor {
    pub rows: Vec<String>,
    filepath: String,
    key_event: KeyEvent,
}

impl Editor {
    pub fn new(filepath: String) -> Editor {
        Editor {
            rows: vec![String::new()],
            filepath,
            key_event: KeyEvent {
                code: crossterm::event::KeyCode::Esc,
                modifiers: KeyModifiers::NONE,
                kind: crossterm::event::KeyEventKind::Press,
                state: KeyEventState::NONE,
            },
        }
    }

    pub fn update_event(&mut self, key_event: KeyEvent) {
        self.key_event = key_event
    }

    pub fn handle_insert_mode(&mut self, mode: &mut Mode) {
        match self.key_event.code {
            crossterm::event::KeyCode::Char(ch) => {
                if let Some(last) = self.rows.last_mut() {
                    last.push(ch);
                } else {
                    self.rows.push(format!("{}", ch));
                }
            }

            crossterm::event::KeyCode::Esc => *mode = Mode::Normal,

            crossterm::event::KeyCode::Enter => self.rows.push(String::new()),

            _ => {}
        }
    }
}

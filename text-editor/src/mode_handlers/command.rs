use std::io::Result;

use crossterm::event::{self, KeyCode};

use crate::{editor::Editor, enums::Mode, kass::Kass};

pub fn handle_command_mode(kass: &mut Kass, close: &mut bool) -> Result<()> {
    match kass.key_event.code {
        event::KeyCode::Char(ch) => match ch {
            _ => kass.app.command.push(ch),
        },
        KeyCode::Esc => {
            kass.app.mode = Mode::Normal;
            kass.app.command = String::new();
        }
        KeyCode::Enter => {
            match kass.app.command.as_str() {
                ":q" => {
                    kass.app.tabs.remove(kass.app.active_index);

                    if kass.app.tabs.len() == 0 {
                        *close = true;
                    } else if kass.app.tabs.len() == kass.app.active_index {
                        kass.app.active_index -= 1;
                    }
                }
                ":qa" => *close = true,
                ":tabnew" => {
                    kass.app.tabs.push(Editor::new());
                    kass.app.active_index = kass.app.tabs.len() - 1;
                }
                _ => {}
            };

            if kass.app.command.contains(":e") {}

            kass.app.mode = Mode::Normal;
            kass.app.command = String::new();
        }
        KeyCode::Backspace => {
            if kass.app.command.len() != 0 {
                kass.app.command.pop();
            }
        }
        _ => {}
    }

    Ok(())
}

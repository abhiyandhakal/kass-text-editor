use crate::{enums::Mode, kass::Kass};
use crossterm::event::{self, KeyEvent};
use std::io::Result;

pub fn handle_normal_mode(kass: &mut Kass) -> Result<()> {
    match kass.key_event {
        KeyEvent {
            code: event::KeyCode::Char(c),
            ..
        } => match c {
            'i' => {
                kass.app.tabs[kass.app.active_index].move_left(1);
                kass.app.mode = Mode::Insert;
            }
            'a' => {
                kass.app.mode = Mode::Insert;
            }
            ':' => {
                kass.app.mode = Mode::Command;
                kass.app.command.push(':');
            }
            // navigation
            'l' => kass.app.tabs[kass.app.active_index].move_right(1),
            'h' => {
                if kass.cursor.x != 1 {
                    kass.app.tabs[kass.app.active_index].move_left(1);
                }
            }
            'j' => kass.app.tabs[kass.app.active_index].move_down(1),
            'k' => kass.app.tabs[kass.app.active_index].move_up(1),
            _ => {}
        },
        KeyEvent {
            code: event::KeyCode::Tab,
            ..
        } => kass.app.next(),
        KeyEvent {
            code: event::KeyCode::BackTab,
            ..
        } => kass.app.previous(),
        _ => {}
    }

    Ok(())
}

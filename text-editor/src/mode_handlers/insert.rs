use std::io::Result;

use crossterm::event;

use crate::{enums::Mode, kass::Kass};

pub fn handle_insert_mode(kass: &mut Kass) -> Result<()> {
    match kass.key_event.code {
        event::KeyCode::Char(c) => {
            if kass.cursor.x as usize
                == kass.app.tabs[kass.app.active_index].rows[kass.cursor.y as usize].len()
                || kass.app.tabs[kass.app.active_index].rows[kass.cursor.y as usize].len() == 0
            {
                kass.app.tabs[kass.app.active_index].rows[kass.cursor.y as usize].push(c);
            } else {
                kass.app.tabs[kass.app.active_index].rows[kass.cursor.y as usize]
                    .insert(kass.cursor.x as usize, c);
            }

            kass.app.tabs[kass.app.active_index].move_right(1);
        }
        event::KeyCode::Backspace => {
            kass.app.tabs[kass.app.active_index].delete();
        }
        event::KeyCode::Enter => {
            kass.app.tabs[kass.app.active_index].goto_newline()?;
        }
        event::KeyCode::Esc => {
            kass.app.mode = Mode::Normal;
        }
        _ => {}
    }

    Ok(())
}
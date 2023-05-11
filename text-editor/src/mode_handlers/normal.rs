use crate::{enums::Mode, kass::Kass};
use crossterm::event::{self, KeyEvent};
use std::io::Result;

use super::group::Group;

pub fn handle_normal_mode(kass: &mut Kass) -> Result<()> {
    let command_list = vec![
        Group::new("i", insert_i),
        // Group::new("a", )
    ];

    match kass.key_event {
        KeyEvent {
            code: event::KeyCode::Char(c),
            ..
        } => match c {
            'i' => insert_i(kass),
            'a' => insert_a(kass),
            ':' => go_to_command(kass),
            // navigation
            'l' => nav_l(kass),
            'h' => nav_h(kass),
            'j' => nav_j(kass),
            'k' => nav_k(kass),
            _ => {}
        },
        KeyEvent {
            code: event::KeyCode::Tab,
            ..
        } => next_tab(kass),
        KeyEvent {
            code: event::KeyCode::BackTab,
            ..
        } => prev_tab(kass),
        _ => {}
    }

    // functions
    fn insert_i(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_left(1);
        kass.app.mode = Mode::Insert;
    }
    fn insert_a(kass: &mut Kass) {
        kass.app.mode = Mode::Insert;
    }
    fn nav_l(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_right(1);
    }
    fn nav_h(kass: &mut Kass) {
        if kass.cursor.x != 1 {
            kass.app.tabs[kass.app.active_index].move_left(1);
        }
    }
    fn go_to_command(kass: &mut Kass) {
        kass.app.mode = Mode::Command;
        kass.app.command.push(':');
    }
    fn nav_j(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_down(1);
    }
    fn nav_k(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_up(1)
    }
    fn next_tab(kass: &mut Kass) {
        kass.app.next();
    }
    fn prev_tab(kass: &mut Kass) {
        kass.app.previous();
    }

    Ok(())
}

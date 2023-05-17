use std::{io::Result, path::Path};

use crossterm::event::{self, KeyCode};

use crate::{editor::Editor, enums::Mode, kass::Kass};

pub fn handle_command_mode(kass: &mut Kass, close: &mut bool) -> Result<()> {
    let mut prefix_list: Vec<(&str, fn(&str, &mut bool, &mut Kass))> = vec![
        (":e", edit_file),
        (":q", quit),
        (":qa", quit_all),
        (":tabnew", new_tab),
    ];

    match kass.key_event.code {
        event::KeyCode::Char(ch) => match ch {
            _ => kass.app.command.push(ch),
        },
        KeyCode::Esc => {
            kass.app.mode = Mode::Normal;
            kass.app.command = String::new();
        }
        KeyCode::Enter => {
            let command = String::from(kass.app.command.clone());
            let mut separated = command.splitn(2, ' ');
            if let Some(prefix) = separated.next() {
                let rest = separated.next().unwrap_or("");
                match prefix_list.iter_mut().find(|(p, _)| *p == prefix) {
                    Some((_, func)) => func(rest, close, kass),
                    None => {}
                }
            } else {
                // do nothing
            }

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

fn edit_file(input: &str, _close: &mut bool, kass: &mut Kass) {
    kass.app.tabs[kass.app.active_index]
        .set_filepath(input.to_string())
        .expect("Couldn't set filepath");
}

fn quit(input: &str, close: &mut bool, kass: &mut Kass) {
    let mut to_remove = kass.app.active_index;

    if let Ok(number) = i32::from_str_radix(input, 10) {
        to_remove = number as usize;
    }

    if to_remove < kass.app.tabs.len() {
        kass.app.tabs.remove(to_remove);
    } else {
        kass.app.tabs.remove(kass.app.active_index);
    }

    if kass.app.tabs.len() == 0 {
        *close = true;
    } else if kass.app.tabs.len() == kass.app.active_index {
        kass.app.active_index -= 1;
    }
}

fn quit_all(_input: &str, close: &mut bool, _kass: &mut Kass) {
    *close = true;
}

fn new_tab(input: &str, _close: &mut bool, kass: &mut Kass) {
    if !Path::new(input).is_dir() {
        let mut new_editor = Editor::default();
        if input != "" {
            new_editor =
                Editor::new(input.to_string()).expect("Couldn't create new editor instance");
        }

        kass.app.tabs.push(new_editor);
        kass.app.active_index = kass.app.tabs.len() - 1;
    }
}

// fn new_tab(input: &str, _close: &mut bool, kass: &mut Kass) {
//     if !Path::new(input).is_dir() {
//         let mut new_editor = Editor::default();
//         if input != "" {
//             new_editor.set_filepath(input.to_string());
//         }
//
//         kass.app.tabs.push(new_editor);
//         kass.app.active_index = kass.app.tabs.len() - 1;
//     }
// }

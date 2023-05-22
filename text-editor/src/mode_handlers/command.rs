use std::{fs::read_to_string, io::Result, path::Path};

use crossterm::event::{self, KeyCode};
use serde_json::Value;

use crate::{
    editor::Editor,
    enums::{Action, Mode},
    kass::Kass,
};

pub fn handle_command_mode(kass: &mut Kass, close: &mut bool) -> Result<()> {
    let mut prefix_with_function_list: Vec<(&str, fn(&str, &mut bool, &mut Kass))> = vec![
        // (":e", edit_file),
        // (":q", quit),
        // (":qa", quit_all),
        // (":tabnew", new_tab),
        // (":w", write),
    ];

    // parse json file
    let config_string = match read_to_string("config.json") {
        Ok(content) => content,
        Err(_) => {
            kass.app.action = Action::Error;
            kass.app.error = "config not found.".to_string();

            "".to_string()
        }
    };

    let config_parsed: Value = serde_json::from_str(config_string.as_str())?;

    if let Value::Object(commands) = &config_parsed["commands"] {
        for (key, value) in commands.iter() {
            match key.as_str() {
                "edit_file" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, edit_file)),
                    None => {}
                },
                "quit" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, quit)),
                    None => {}
                },
                "quit_all" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, quit_all)),
                    None => {}
                },
                "new_tab" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, new_tab)),
                    None => {}
                },
                "write" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, write)),
                    None => {}
                },
                key => {
                    kass.app.action = Action::Error;
                    kass.app.error = format!("{} in the config doesn't exist", key.to_string())
                }
            }
        }
    } else {
        kass.app.action = Action::Error;
        kass.app.error = "Commands not found in the config".to_string();
    }

    match kass.key_event.code {
        event::KeyCode::Char(ch) => match ch {
            _ => kass.app.command.push(ch),
        },
        KeyCode::Esc => {
            kass.app.mode = Mode::Normal;
            kass.app.command = String::new();
        }
        KeyCode::Enter => {
            let command = &String::from(kass.app.command.clone())[1..];
            let mut separated = command.splitn(2, ' ');

            if let Some(prefix) = separated.next() {
                let rest = separated.next().unwrap_or("");
                match prefix_with_function_list
                    .iter_mut()
                    .find(|(p, _)| *p == prefix)
                {
                    Some((_, func)) => func(rest, close, kass),
                    None => {
                        kass.app.action = Action::Error;
                        kass.app.error = "Command not found.".to_string();
                    }
                }
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
    if !Path::new(input).is_dir() {
        match kass.app.tabs[kass.app.active_index].set_filepath(input.to_string()) {
            Ok(_) => {}
            Err(_) => {
                kass.app.action = Action::Error;
                kass.app.error = "Couldn't edit file".to_string();
            }
        }
    } else {
        kass.app.action = Action::Error;
        kass.app.error = "Cannot edit a directory. Provide a file path".to_string();
    }
}

fn quit(input: &str, close: &mut bool, kass: &mut Kass) {
    let mut to_remove = kass.app.active_index;

    if let Ok(number) = i32::from_str_radix(input, 10) {
        to_remove = number as usize;
    }
    if kass.app.tabs[to_remove]
        .is_saved()
        .expect("Couldn't save file")
    {
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
    } else {
        kass.app.action = Action::Error;
        kass.app.error = "File not saved".to_string();
    }
}

fn quit_all(_input: &str, close: &mut bool, _kass: &mut Kass) {
    *close = true;
}

fn new_tab(input: &str, _close: &mut bool, kass: &mut Kass) {
    if !Path::new(input).is_dir() {
        let mut filepath = "unnamed".to_string();
        let mut counter = 0;

        while Path::new(&filepath).exists() {
            counter += 1;
            filepath = format!("{}-{}", filepath, counter);
        }

        for tab in kass.app.tabs.iter() {
            if tab.title == filepath {
                counter += 1;
                filepath = format!("unnamed-{}", counter);
            }
        }

        let mut new_editor = Editor::new(filepath.clone()).expect("Couln't create file 1");

        if input != "" {
            new_editor =
                Editor::new(input.to_string()).expect("Couldn't create new editor instance");
        }

        kass.app.tabs.push(new_editor);
        kass.app.active_index = kass.app.tabs.len() - 1;
    } else {
        kass.app.action = Action::Error;
        kass.app.error = "Provide a filepath".to_string();
    }
}

fn write(_input: &str, _close: &mut bool, kass: &mut Kass) {
    match kass.app.tabs[kass.app.active_index].save() {
        Ok(_) => {
            kass.app.action = Action::Info;
            kass.app.info = format!("{} saved.", kass.app.tabs[kass.app.active_index].title);
        }
        Err(e) => {
            kass.app.action = Action::Error;
            kass.app.error = e.to_string();
        }
    }
}

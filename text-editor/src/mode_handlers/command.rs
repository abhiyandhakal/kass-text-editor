use std::io::Result;

use crossterm::event::{self, KeyCode};
use serde_json::Value;

use crate::functions;
use crate::{enums::Mode, kass::Kass};

pub fn handle_command_mode(kass: &mut Kass, close: &mut bool, config: &Value) -> Result<()> {
    let mut prefix_with_function_list: Vec<(&str, fn(&str, &mut bool, &mut Kass))> = vec![];

    if let Value::Object(commands) = &config["command_mode"] {
        for (key, value) in commands.iter() {
            match key.as_str() {
                "edit_file" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::edit_file)),
                    None => {}
                },
                "quit" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::quit)),
                    None => {}
                },
                "quit_all" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::quit_all)),
                    None => {}
                },
                "new_tab" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::new_tab)),
                    None => {}
                },
                "write" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::write)),
                    None => {}
                },
                "force_quit" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::force_quit)),
                    None => {}
                },
                "force_quit_all" => match value.as_str() {
                    Some(value) => {
                        prefix_with_function_list.push((value, functions::force_quit_all))
                    }
                    None => {}
                },
                "write_all" => match value.as_str() {
                    Some(value) => prefix_with_function_list.push((value, functions::write_all)),
                    None => {}
                },
                "write_and_quit" => match value.as_str() {
                    Some(value) => {
                        prefix_with_function_list.push((value, functions::write_and_quit))
                    }
                    None => {}
                },
                "write_and_quit_all" => match value.as_str() {
                    Some(value) => {
                        prefix_with_function_list.push((value, functions::write_and_quit_all))
                    }
                    None => {}
                },
                key => {
                    kass.set_error(format!("{} in the config doesn't exist", key).as_str());
                }
            }
        }
    } else {
        kass.set_error("Commands not found in the config");
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
                    None => kass.set_error("Command not found."),
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

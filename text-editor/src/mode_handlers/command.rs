use core::panic;
use std::{env, fs::read_to_string, io::Result, path::PathBuf};

use crossterm::event::{self, KeyCode};
use serde_json::Value;

use crate::functions;
use crate::{
    enums::{Action, Mode},
    kass::Kass,
};

pub fn handle_command_mode(kass: &mut Kass, close: &mut bool) -> Result<()> {
    let mut prefix_with_function_list: Vec<(&str, fn(&str, &mut bool, &mut Kass))> = vec![];

    // Determine the appropriate directory based on the operating system
    let config_dir = if cfg!(unix) {
        match env::var_os("XDG_CONFIG_HOME") {
            Some(dir) => PathBuf::from(dir).join("kass"),
            None => {
                let home_dir: PathBuf = match dirs::home_dir() {
                    Some(dir) => dir,
                    None => panic!("home directory not found"),
                };
                home_dir.join(".config").join("kass")
            }
        }
    } else if cfg!(windows) {
        match env::var_os("APPDATA") {
            Some(app_data) => PathBuf::from(app_data).join("kass"),
            None => panic!("Unable to determine the configuration directory."),
        }
    } else {
        panic!("Unsupported operating system.");
    };
    // Create the full path for the configuration file
    let config_file = config_dir.join("config.json");

    // parse json file
    let config_string = match read_to_string(config_file) {
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

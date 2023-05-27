use std::{env, fs::read_to_string, io::stdout, path::PathBuf};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use kass::Kass;
use serde_json::Value;
use tui::{backend::CrosstermBackend, Terminal};

mod editor;
mod enums;
mod functions;
mod kass;
mod mode_handlers;
mod position;
mod ui;

fn main() {
    let mut kass_editor = match Kass::new() {
        Ok(editor) => Some(editor),
        Err(e) => {
            println!("{}", e);
            None
        }
    };

    if let Some(editor) = &mut kass_editor {
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
            Err(e) => {
                editor.set_error(e.to_string().as_str());

                "".to_string()
            }
        };

        let config_parsed: Option<Value> = match serde_json::from_str(config_string.as_str()) {
            Ok(conf) => Some(conf),
            Err(e) => {
                editor.set_error(e.to_string().as_str());

                None
            }
        };

        match enable_raw_mode() {
            Ok(_) => {}
            Err(e) => editor.set_error(e.to_string().as_str()),
        }

        match execute!(stdout(), EnterAlternateScreen, EnableMouseCapture) {
            Ok(_) => {}
            Err(e) => editor.set_error(e.to_string().as_str()),
        };

        let backend = CrosstermBackend::new(stdout());

        let mut terminal = match Terminal::new(backend) {
            Ok(terminal) => Some(terminal),
            Err(e) => {
                editor.set_error(e.to_string().as_str());
                None
            }
        };

        if let Some(terminal) = &mut terminal {
            match editor.run(terminal, config_parsed) {
                Ok(_) => {}
                Err(e) => editor.set_error(e.to_string().as_str()),
            };

            match disable_raw_mode() {
                Ok(_) => {}
                Err(e) => editor.set_error(e.to_string().as_str()),
            };

            match execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            ) {
                Ok(_) => {}
                Err(e) => editor.set_error(e.to_string().as_str()),
            };

            match terminal.show_cursor() {
                Ok(_) => {}
                Err(e) => editor.set_error(e.to_string().as_str()),
            };
        }
    }
}

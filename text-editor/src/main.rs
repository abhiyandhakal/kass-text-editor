use std::io::stdout;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use kass::Kass;
use tui::{backend::CrosstermBackend, Terminal};

mod editor;
mod enums;
mod functions;
mod kass;
mod mode_handlers;
mod position;

fn main() {
    let mut kass_editor = match Kass::new() {
        Ok(editor) => Some(editor),
        Err(_) => None,
    };

    if let Some(editor) = &mut kass_editor {
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
            match editor.run(terminal) {
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

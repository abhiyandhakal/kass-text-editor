use std::path::Path;

use crate::{editor::Editor, kass::Kass};

pub fn goto_line(kass: &mut Kass, line_number: usize) {
    let current_pos = kass.app.tabs[kass.app.active_index].rowoff + kass.cursor.y;

    if line_number == kass.app.tabs[kass.app.active_index].rows.len() {
        let last_line = kass.app.tabs[kass.app.active_index].rows.len() - 1;

        kass.app.tabs[kass.app.active_index].cursor.y =
            kass.app.tabs[kass.app.active_index].editor_size.y - 1;

        kass.app.tabs[kass.app.active_index].rowoff =
            last_line as u16 - kass.app.tabs[kass.app.active_index].editor_size.y + 1;
    } else if current_pos >= line_number as u16 - 1 {
        for _ in 0..current_pos - line_number as u16 + 1 {
            kass.app.tabs[kass.app.active_index].move_up(1);
        }
    } else if current_pos < line_number as u16 - 1 {
        for _ in 0..line_number as u16 - 1 - current_pos {
            kass.app.tabs[kass.app.active_index].move_down(1);
        }
    }
}

pub fn edit_file(input: &str, _close: &mut bool, kass: &mut Kass) {
    if !Path::new(input).is_dir() {
        match kass.app.tabs[kass.app.active_index].set_filepath(input.to_string()) {
            Ok(_) => {}
            Err(e) => kass.set_error(e.to_string().as_str()),
        }
    } else {
        kass.set_error("Cannot edit a directory. Provide a file path")
    }
}

pub fn quit(input: &str, close: &mut bool, kass: &mut Kass) {
    let mut to_remove = kass.app.active_index;

    if let Ok(number) = i32::from_str_radix(input, 10) {
        to_remove = number as usize;
    }

    if kass.app.tabs[to_remove].is_saved() {
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
        kass.set_error("File not saved")
    }
}

pub fn quit_all(_input: &str, close: &mut bool, kass: &mut Kass) {
    let mut saved = true;

    for tab in kass.app.tabs.iter() {
        if !tab.is_saved() {
            saved = false;
            kass.set_error(format!("{} is not saved!", tab.title).as_str());
            break;
        }
    }

    if saved {
        *close = true;
    }
}

pub fn new_tab(input: &str, _close: &mut bool, kass: &mut Kass) {
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
        kass.set_error("Provide a filepath");
    }
}

pub fn write(_input: &str, _close: &mut bool, kass: &mut Kass) {
    match kass.app.tabs[kass.app.active_index].save() {
        Ok(_) => {
            kass.set_info(
                format!("{} saved.", kass.app.tabs[kass.app.active_index].title).as_str(),
            );
        }
        Err(e) => {
            kass.set_error(e.to_string().as_str());
        }
    }
}

pub fn force_quit(input: &str, close: &mut bool, kass: &mut Kass) {
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

pub fn force_quit_all(_input: &str, close: &mut bool, _kass: &mut Kass) {
    *close = true;
}

pub fn write_all(_input: &str, _close: &mut bool, kass: &mut Kass) {
    let mut i = 0;
    loop {
        match kass.app.tabs[kass.app.active_index].save() {
            Ok(_) => {
                kass.set_info(
                    format!("{} saved.", kass.app.tabs[kass.app.active_index].title).as_str(),
                );
            }
            Err(e) => {
                kass.set_error(e.to_string().as_str());
                break;
            }
        }
        i += 1;

        if i == kass.app.tabs.len() {
            break;
        }
    }
}

pub fn write_and_quit(input: &str, close: &mut bool, kass: &mut Kass) {
    write(input, close, kass);
    quit(input, close, kass);
}

pub fn write_and_quit_all(input: &str, close: &mut bool, kass: &mut Kass) {
    write_all(input, close, kass);
    quit_all(input, close, kass);
}

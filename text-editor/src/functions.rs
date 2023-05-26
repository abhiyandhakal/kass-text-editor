use std::path::Path;

use crate::{editor::Editor, kass::Kass};

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

pub fn quit_all(_input: &str, close: &mut bool, _kass: &mut Kass) {
    *close = true;
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

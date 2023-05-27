use crate::{
    enums::{Action, Element, Mode},
    kass::Kass,
};
use crossterm::event::{self, KeyCode, KeyEvent};
use std::io::Result;

fn add_to_buf(kass: &mut Kass) {
    match kass.key_event {
        KeyEvent {
            code: KeyCode::Char(c),
            ..
        } => kass.buf.push(c),
        _ => {}
    }
}

fn parse_buf(input: String) -> Vec<Element> {
    // Parse elements into a list
    let mut elements_list: Vec<Element> = Vec::new();
    let mut current_number = String::new();

    for ch in input.chars() {
        if ch.is_digit(10) {
            current_number.push(ch);
        } else {
            if !current_number.is_empty() {
                let number = current_number.parse().unwrap();
                elements_list.push(Element::Num(number));
                current_number.clear();
            }
            elements_list.push(Element::Char(ch));
        }
    }

    if !current_number.is_empty() {
        let number = current_number.parse().unwrap();
        elements_list.push(Element::Num(number));
    }

    elements_list
}

pub fn handle_normal_mode(kass: &mut Kass) -> Result<()> {
    add_to_buf(kass);
    let parsed_buf = parse_buf(kass.buf.clone());
    let mut _action = Action::Default;

    if !parsed_buf.is_empty() {
        match parsed_buf[0] {
            Element::Char(ch) => match ch {
                'd' => _action = Action::Delete,
                _ => {}
            },
            _ => {}
        }

        match parsed_buf[parsed_buf.len() - 1] {
            Element::Char(c) => match c {
                'i' => insert_i(kass),
                'a' => insert_a(kass),
                ':' => go_to_command(kass),
                'h' => {
                    let mut count = 1;
                    if parsed_buf.len() >= 2 {
                        count = match parsed_buf[parsed_buf.len() - 2] {
                            Element::Num(num) => num,
                            _ => 1,
                        };
                    }
                    for _ in 0..count {
                        nav_h(kass)
                    }
                }
                'j' => {
                    let mut count = 1;
                    if parsed_buf.len() >= 2 {
                        count = match parsed_buf[parsed_buf.len() - 2] {
                            Element::Num(num) => num,
                            _ => 1,
                        };
                    }
                    for _ in 0..count {
                        nav_j(kass)
                    }
                }
                'k' => {
                    let mut count = 1;
                    if parsed_buf.len() >= 2 {
                        count = match parsed_buf[parsed_buf.len() - 2] {
                            Element::Num(num) => num,
                            _ => 1,
                        };
                    }
                    for _ in 0..count {
                        nav_k(kass)
                    }
                }
                'l' => {
                    let mut count = 1;
                    if parsed_buf.len() >= 2 {
                        count = match parsed_buf[parsed_buf.len() - 2] {
                            Element::Num(num) => num,
                            _ => 1,
                        };
                    }
                    for _ in 0..count {
                        nav_l(kass)
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    match kass.key_event {
        KeyEvent {
            code: event::KeyCode::Tab,
            ..
        } => next_tab(kass),
        KeyEvent {
            code: event::KeyCode::BackTab,
            ..
        } => prev_tab(kass),
        KeyEvent {
            code: KeyCode::Esc, ..
        } => kass.buf = String::new(),
        _ => {}
    }

    // functions
    fn insert_i(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_left(1);
        kass.app.mode = Mode::Insert;
        kass.buf.clear();
    }
    fn insert_a(kass: &mut Kass) {
        kass.app.mode = Mode::Insert;
        kass.buf.clear();
    }
    fn nav_l(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_right(1);
        kass.buf.clear();
    }
    fn nav_h(kass: &mut Kass) {
        if kass.cursor.x != 1 {
            kass.app.tabs[kass.app.active_index].move_left(1);
        }
        kass.buf.clear();
    }
    fn go_to_command(kass: &mut Kass) {
        kass.app.mode = Mode::Command;
        kass.app.command.push(':');
        kass.buf.clear();
    }
    fn nav_j(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_down(1);
        kass.buf.clear();
    }
    fn nav_k(kass: &mut Kass) {
        kass.app.tabs[kass.app.active_index].move_up(1);
        kass.buf.clear();
    }
    fn next_tab(kass: &mut Kass) {
        kass.app.next();
        kass.buf.clear();
    }
    fn prev_tab(kass: &mut Kass) {
        kass.app.previous();
        kass.buf.clear();
    }

    Ok(())
}

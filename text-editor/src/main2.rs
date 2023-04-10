use crossterm::{cursor, execute, terminal};
use std::collections::LinkedList;
use std::fs::{self, read_to_string};
use std::io::{stdin, stdout, Read, Result, Write};

struct CleanUp;

impl CleanUp {
    fn clean(&self) {
        // disables raw mode, terminal goes back to normal
        terminal::disable_raw_mode().expect("couldn't disable raw mode");
    }
}

fn main() -> Result<()> {
    let _clean_up = CleanUp;

    let mut stdout = stdout();

    // enables raw mode, terminal commands don't work
    terminal::enable_raw_mode()?;

    // clear terminal
    execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorUp))?;
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // open a file
    let mut myfile = fs::OpenOptions::new().append(true).open("hello.txt")?;

    // read the file
    let contents = read_to_string("hello.txt")?;

    let mut character_list = LinkedList::new();

    // itering contents and storing in a linked list
    let _contents = contents.clone();
    for c in _contents.into_bytes().iter() {
        let character = *c as char;

        character_list.push_back(character);
    }

    // print the previous contents of the file
    print!("{}", contents);
    stdout.flush()?;

    let mut buffer = [0];

    while stdin().read(&mut buffer)? == 1 {
        let character = buffer[0] as char;

        // quit if 'q' is typed
        if character == 'q' {
            break;
        }

        if !character.is_control() {
            // // instantaneous printing of typed characters
            // print!("{}", buffer[0] as char);
            // stdout.flush()?;

            // myfile.write(&[character as u8])?;

            for character in character_list.iter() {
                // clear terminal
                execute!(stdout, terminal::Clear(terminal::ClearType::FromCursorUp))?;
                execute!(stdout, cursor::MoveTo(0, 0))?;

                print!("{}", character);
                stdout.flush()?;
            }
        } else {
            print!("{}", character as u8);
            stdout.flush()?;
        }
    }

    _clean_up.clean();

    Ok(())
}

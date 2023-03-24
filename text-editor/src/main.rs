use crossterm::{execute, terminal, Result};
use std::{
    collections::LinkedList,
    fs::OpenOptions,
    io::{stdin, stdout, Read, Write},
};

mod stack;

fn main() -> Result<()> {
    let mut stdout = stdout();

    let mut line_stack = stack::Stack {
        list: LinkedList::new(),
    };
    println!("popped element: {}", line_stack.pop());

    // clears the terminal
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

    // // commands like <Ctrl-z>, <Ctrl-c> don't work
    // // Right now, nothing works, so commented
    // terminal::enable_raw_mode()?;

    // checks if the colon is pressed to enter a command
    let mut colon_pressed = false;

    // open a file
    let mut myfile = OpenOptions::new()
        .append(true)
        .open("/home/abhiyan/hello.txt")?;

    // reads through every character entered
    for i in stdin().bytes() {
        let character = i.unwrap() as char;
        print!("{}", character);

        if character == ':' {
            colon_pressed = true;
        }

        let character_arr = [character as u8];
        myfile.write(&character_arr)?;

        if colon_pressed {
            // quit
            if character == 'q' {
                break;
            } else if character == 'w' {
                println!("write");
            }
        }
    }

    Ok(())
}

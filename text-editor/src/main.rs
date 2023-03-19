use std::{
    fs::OpenOptions,
    io::{stdin, Write},
};

fn main() {
    let mut text = String::new();

    // open a file
    let mut myfile = OpenOptions::new()
        .append(true)
        .open("/home/abhiyan/hello.txt")
        .expect("couldn't open file");

    loop {
        stdin()
            .read_line(&mut text)
            .expect("couldn't read the line");

        myfile.write(&text.as_bytes()).expect("couldn't append");
    }
}

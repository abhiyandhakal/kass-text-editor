pub struct Statusbar {
    mode: String,
}

impl Statusbar {
    pub fn new(mode: String) -> Statusbar {
        println!("helloworld\r");

        Statusbar {
            mode: String::from("Normal"),
        }
    }
}

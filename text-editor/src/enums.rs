#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug, Clone, Copy)]
pub enum CommandAction {
    Command,
    Error,
    Info,
}

#[derive(Debug, Clone, Copy)]
pub enum LineNumber {
    None,
    Relative,
    Absolute,
}

#[derive(Debug)]
pub enum Element {
    Num(usize),
    Char(char),
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Delete,
    Default,
}

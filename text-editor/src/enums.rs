#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Normal,
    Insert,
    Command,
}

#[derive(Debug, Clone, Copy)]
pub enum Action {
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

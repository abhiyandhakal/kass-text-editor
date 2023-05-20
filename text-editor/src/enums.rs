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

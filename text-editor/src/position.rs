#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Position {
    pub fn new() -> Position {
        Position { x: 0, y: 0 }
    }

    pub fn set_pos(&mut self, x: u16, y: u16) {
        self.x = x;
        self.y = y;
    }
}

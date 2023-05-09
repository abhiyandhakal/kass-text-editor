use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Editor {
    pub rows: Vec<String>,
    filepath: String,
    pub cursor: Position,
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            rows: vec![String::new()],
            filepath: String::new(),
            cursor: Position::new(),
        }
    }

    pub fn move_right(&mut self, steps: u16) {
        let current_row = self.cursor.y;

        let mut pos_x = self.cursor.x + steps;

        if pos_x as usize >= self.rows[current_row as usize].len() {
            pos_x = self.rows[current_row as usize].len() as u16;
        }

        self.cursor.set_pos(pos_x, current_row);
    }

    pub fn move_left(&mut self, steps: u16) {
        let current_row = self.cursor.y;

        let mut pos_x = 0;

        if self.cursor.x >= steps {
            pos_x = self.cursor.x - steps;
        }

        self.cursor.set_pos(pos_x, current_row);
    }

    pub fn move_down(&mut self, steps: u16) {
        let mut pos_x = self.cursor.x;
        let mut pos_y = self.cursor.y + steps;

        if pos_y >= self.rows.len() as u16 {
            pos_y = self.rows.len() as u16 - 1;
        }

        if pos_x >= self.rows[pos_y as usize - steps as usize].len() as u16 {
            pos_x = self.rows[pos_y as usize].len() as u16;
        }

        self.cursor.set_pos(pos_x, pos_y);
    }
    pub fn move_up(&mut self, steps: u16) {
        let mut pos_x = self.cursor.x;
        let pos_y = if self.cursor.y <= 0 {
            0
        } else {
            self.cursor.y - steps
        };

        if pos_x >= self.rows[pos_y as usize + steps as usize].len() as u16 {
            pos_x = self.rows[pos_y as usize].len() as u16;
        }

        self.cursor.set_pos(pos_x, pos_y);
    }
}

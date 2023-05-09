use std::io::Result;

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

    pub fn insert_row(&mut self, idx: usize, row_content: String) {
        if idx > self.rows.len() {
            return;
        }
        self.rows.insert(idx, row_content);
    }

    pub fn goto_newline(&mut self) -> Result<()> {
        let row_idx = self.cursor.y as usize;
        if self.cursor.x == 0 {
            self.insert_row(row_idx, String::from(""));
        } else {
            let content = self.rows[self.cursor.y as usize].split_off(self.cursor.x as usize);
            self.insert_row(row_idx + 1, content);
        };

        self.cursor.x = 0;

        self.cursor.y += 1;
        Ok(())
    }
}

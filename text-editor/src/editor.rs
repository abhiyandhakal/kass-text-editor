use std::io::Result;

use crate::position::Position;

#[derive(Debug, Clone)]
pub struct Bound {
    pub x1: u16,
    pub x2: u16,
}

#[derive(Debug, Clone)]
pub struct Editor {
    pub rows: Vec<String>,
    pub filepath: String,
    pub cursor: Position,
    pub coloff: u16,
    pub rowoff: u16,
    pub bounds: (Bound, Bound),
}

impl Editor {
    pub fn new() -> Editor {
        Editor {
            rows: vec![String::new()],
            filepath: String::new(),
            cursor: Position::new(),
            coloff: 0,
            rowoff: 0,
            bounds: (Bound { x1: 0, x2: 0 }, Bound { x1: 0, x2: 0 }),
        }
    }

    pub fn move_right(&mut self, steps: u16) {
        let current_row = self.cursor.y + self.rowoff;

        let mut pos_x = self.cursor.x + steps;

        if pos_x as usize >= self.rows[current_row as usize].len() {
            pos_x = self.rows[current_row as usize].len() as u16;
        }

        self.cursor.x = pos_x;
    }

    pub fn move_left(&mut self, steps: u16) {
        let current_row = self.cursor.y + self.rowoff;

        let mut pos_x = 0;

        if self.cursor.x >= steps {
            pos_x = self.cursor.x - steps;
        }

        self.cursor.set_pos(pos_x, current_row);
    }

    pub fn move_down(&mut self, steps: u16) {
        let curr_row = self.cursor.y + self.rowoff;
        let mut pos_x = self.cursor.x;
        let pos_y = if curr_row >= self.rows.len() as u16 - 1 {
            self.rows.len() as u16 - 1
        } else {
            self.cursor.y + steps
        };

        if pos_x >= self.rows[curr_row as usize].len() as u16 {
            pos_x = self.rows[pos_y as usize].len() as u16;
        } else {
            pos_x = if pos_x >= self.rows[pos_y as usize].len() as u16 {
                self.rows[pos_y as usize].len() as u16
            } else {
                pos_x
            };
        }

        self.cursor.set_pos(pos_x, pos_y);

        if self.cursor.y > self.bounds.1.x2 {
            self.rowoff += 1;
        }
    }
    pub fn move_up(&mut self, steps: u16) {
        let mut pos_x = self.cursor.x;
        let pos_y = if self.cursor.y <= 0 {
            self.cursor.y
        } else {
            self.cursor.y - steps
        };

        if pos_x >= self.rows[self.cursor.y as usize].len() as u16 {
            pos_x = self.rows[pos_y as usize].len() as u16;
        } else {
            pos_x = if pos_x >= self.rows[pos_y as usize].len() as u16 {
                self.rows[pos_y as usize].len() as u16
            } else {
                pos_x
            };
        }

        self.cursor.set_pos(pos_x, pos_y);

        if self.cursor.y < self.bounds.1.x2 {
            if self.rowoff != 0 {
                self.rowoff -= 1;
            }
        }
    }

    pub fn insert_row(&mut self, idx: usize, row_content: String) {
        if idx > self.rows.len() {
            return;
        }
        self.rows.insert(idx, row_content);
    }

    pub fn goto_newline(&mut self) -> Result<()> {
        let row_idx = self.cursor.y as usize + self.rowoff as usize;

        if self.cursor.x == 0 {
            self.insert_row(row_idx, String::from(""));
        } else {
            let content = self.rows[row_idx].split_off(self.cursor.x as usize);
            self.insert_row(row_idx + 1, content);
        };

        self.cursor.x = 0;

        if self.cursor.y <= self.bounds.1.x2 - self.rowoff - 2 {
            self.cursor.y += 1;
        } else {
            self.rowoff += 1;
        }

        Ok(())
    }

    // handling deletion of character
    pub fn delete(&mut self) {
        if self.cursor.y > self.rows.len() as u16 {
            return;
        }
        if self.cursor.x == 0 && self.cursor.y == 0 {
            return;
        }

        let curr_row = self.cursor.y as usize;

        if self.cursor.x > 0 {
            if self.del_char(self.cursor.x as usize - 1) {
                if self.cursor.x > self.rows[curr_row].len() as u16 {
                    self.cursor.x = self.rows[curr_row].len() as u16;
                } else {
                    self.cursor.x -= 1;
                }
            }
        } else {
            let row_content = self.rows[curr_row].clone();

            self.cursor.x = self.rows[curr_row - 1].len() as u16;
            self.rows[curr_row - 1].push_str(row_content.as_str());
            self.rows.remove(curr_row);
            self.cursor.y -= 1;
        }
    }

    fn del_char(&mut self, idx: usize) -> bool {
        if idx >= self.rows[self.cursor.y as usize].len() {
            false
        } else {
            self.rows[self.cursor.y as usize].remove(idx);
            true
        }
    }

    fn scroll(&mut self) {
        // for vertical scrolling
        let terminal_height = self.bounds.1.x2 - self.rowoff - 2;
        if self.cursor.y < self.rowoff {
            self.rowoff = self.cursor.y;
        }
        if self.cursor.y >= self.rowoff + terminal_height {
            self.rowoff = self.cursor.y - terminal_height + 1;
        }

        // for horizontal scrolling
        if self.cursor.x < self.coloff {
            self.coloff = self.cursor.x;
        }
        if self.cursor.x >= self.coloff + self.bounds.1.x2 {
            self.coloff = self.cursor.x - self.bounds.1.x2 + 1;
        }
    }
}

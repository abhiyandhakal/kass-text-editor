use std::{
    fs::{read_to_string, OpenOptions},
    io::{prelude::*, Result},
    path::Path,
};

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
    pub editor_size: Position,
    pub title: String,
}

impl Editor {
    fn file_to_rows(filepath: String) -> Result<Vec<String>> {
        let mut rows: Vec<String> = vec![String::new()];

        if Path::new(filepath.as_str()).is_file() {
            let content = read_to_string(filepath)?;

            // let myrows = content.split('\n').collect();
            rows = content.lines().map(String::from).collect();
        }

        Ok(rows)
    }

    fn rows_to_file(rows: Vec<String>) -> String {
        let content = rows.join("\n");

        content
    }

    pub fn new(filepath: String) -> Result<Editor> {
        let mut rows = Self::file_to_rows(filepath.clone())?;

        if rows.len() == 0 {
            rows.push(String::new())
        }

        let file_name: String = match Path::new(filepath.as_str()).file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => String::from("New Tab"),
        };

        let title = file_name;

        Ok(Editor {
            rows,
            filepath,
            cursor: Position::new(),
            coloff: 0,
            rowoff: 0,
            bounds: (Bound { x1: 0, x2: 0 }, Bound { x1: 0, x2: 0 }),
            editor_size: Position::new(),
            title,
        })
    }

    pub fn is_saved(&self) -> bool {
        let rows = match Self::file_to_rows(self.filepath.clone()) {
            Ok(rows) => Some(rows),
            Err(_) => None,
        };

        if let Some(rows) = rows {
            if self.rows.clone() == rows {
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn save(&mut self) -> Result<()> {
        // Open the file with write mode and create it if it doesn't exist
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&self.filepath)?;

        file.write_all(Self::rows_to_file(self.rows.clone()).as_bytes())?;

        Ok(())
    }

    pub fn set_filepath(&mut self, filepath: String) -> Result<()> {
        let file_name: String = match Path::new(filepath.as_str()).file_name() {
            Some(filename) => filename.to_string_lossy().to_string(),
            None => String::from("New Tab"),
        };

        self.title = file_name;
        self.filepath = filepath;
        self.rows = Self::file_to_rows(self.filepath.clone())?;

        if self.rows.len() == 0 {
            self.rows.push(String::new());
        }

        Ok(())
    }

    pub fn boundary(&mut self, terminal_width: u16, terminal_height: u16) {
        self.editor_size.x = terminal_width;
        self.editor_size.y = terminal_height - 1;
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
        let mut pos_x = 0;

        if self.cursor.x >= steps {
            pos_x = self.cursor.x - steps;
        }

        self.cursor.set_pos(pos_x, self.cursor.y);
    }

    pub fn move_down(&mut self, steps: u16) {
        let curr_row = self.cursor.y + self.rowoff;
        let mut pos_x = self.cursor.x;
        let pos_y = if curr_row >= self.rows.len() as u16 - 1 {
            self.rows.len() as u16 - 1
        } else {
            curr_row + steps
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

        self.cursor.x = pos_x;

        if curr_row + 1 < self.rows.len() as u16 {
            if self.cursor.y <= self.editor_size.y - 1 {
                self.cursor.y += 1;
            } else {
                self.rowoff += 1;
            }
        }

        // execute!(stdout()
        //     .queue(cursor::MoveTo(40, 6))
        //     .expect("err")
        //     .queue(Print(format!(
        //         "{} ,{}, {}, {}",
        //         self.rows.len(),
        //         curr_row,
        //         self.cursor.y,
        //         self.rowoff
        //     )))
        //     .expect("err"));
    }
    pub fn move_up(&mut self, steps: u16) {
        let curr_row = self.cursor.y + self.rowoff;

        let mut pos_x = self.cursor.x;
        let pos_y: u16 = self.cursor.y.saturating_sub(steps);

        if pos_x >= self.rows[curr_row as usize].len() as u16 {
            pos_x = self.rows[curr_row.saturating_sub(steps) as usize].len() as u16;
        } else {
            pos_x = if pos_x >= self.rows[pos_y as usize].len() as u16 {
                self.rows[curr_row.saturating_sub(steps) as usize].len() as u16
            } else {
                pos_x
            };
        }

        self.cursor.x = pos_x;

        if pos_y < self.editor_size.y + 1 {
            if self.rowoff != 0 && self.cursor.y == 0 {
                self.rowoff = self.rowoff.saturating_sub(1);
            } else {
                self.cursor.y = pos_y;
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
}
